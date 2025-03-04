use anchor_lang::prelude::*;

use anchor_spl::{associated_token::AssociatedToken, token_interface::{TokenAccount, Mint, Burn, burn, TransferChecked, transfer_checked, TokenInterface}};

use constant_product_curve::ConstantProduct;

use crate::state::{Config, error::*};


#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub liquidity_provider: Signer<'info>,

    pub mint_token_x: Box<InterfaceAccount<'info, Mint>>,

    pub mint_token_y: Box<InterfaceAccount<'info, Mint>>,

    //The mint_lp_token
    #[account(
        mut,
        seeds = [b"lp", config.seed.to_le_bytes().as_ref()],
        bump = config.lp_bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub mint_lp_token: Box<InterfaceAccount<'info, Mint>>,

    // Liquidity Provider ATA for tokens x and y
    #[account(
        mut,
        associated_token::mint = mint_token_x,
        associated_token::authority = liquidity_provider,
    )]
    pub liquidity_provider_ata_x: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_token_y,
        associated_token::authority = liquidity_provider,
    )]
    pub liquidity_provider_ata_y: Box<InterfaceAccount<'info, TokenAccount>>,

    // Liquidity Provider ATA for lp token
    #[account(
        mut,
        associated_token::mint = mint_lp_token,
        associated_token::authority = liquidity_provider,
        associated_token::token_program = token_program,
    )]
    pub liquidity_provider_ata_lp_token: Box<InterfaceAccount<'info, TokenAccount>>,

    // Vault Tokens for x and y
    #[account(
        mut,
        associated_token::mint = mint_token_x,
        associated_token::authority = config,
    )]
    pub vault_ata_x: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_token_y,
        associated_token::authority = config,
    )]
    pub vault_ata_y: Box<InterfaceAccount<'info, TokenAccount>>,

    // Config Account
    #[account(
        seeds= [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
    )]
    pub config: Box<Account<'info, Config>>,

    // Tokens And System Programs
    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}



impl<'info> Withdraw<'info> {

    // Withdraw function
    pub fn withdraw(&mut self, lp_amount: u64, min_x: u64, min_y: u64) -> Result<()> {

        require!(self.config.locked == false, AmmError::AmmPoolLocked);
        require!(lp_amount > 0, AmmError::InvalidAmount);
        require!(min_x > 0 || min_y > 0, AmmError::InvalidAmount);

        let amounts =  ConstantProduct::xy_withdraw_amounts_from_l(
            self.vault_ata_x.amount,
            self.vault_ata_y.amount,
            self.mint_lp_token.supply,
            lp_amount,
            6
        ).map_err(AmmError::from)?;

        require!(amounts.x >= min_x && amounts.y >= min_y, AmmError::SlippageExceeded);

        // Let's call withdraw tokens for x or y and burn lp tokens
        self.withdraw_tokens(true, amounts.x)?;

        self.withdraw_tokens(false, amounts.y)?;

        self.burn_lp_tokens(lp_amount)?;

        Ok(())
    }


    pub fn withdraw_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {

        let (mint_token, from, to, decimals) = match is_x {
            true => (
                self.mint_token_x.to_account_info(),
                self.vault_ata_x.to_account_info(),
                self.liquidity_provider_ata_x.to_account_info(),
                self.mint_token_x.decimals
            ),
            false => (
                self.mint_token_y.to_account_info(),
                self.vault_ata_y.to_account_info(),
                self.liquidity_provider_ata_y.to_account_info(),
                self.mint_token_y.decimals
            )
        };

        let cpi_program = self.token_program.to_account_info();

        let accounts = TransferChecked {
            from: from,
            mint: mint_token,
            to: to,
            authority: self.config.to_account_info(),
        };

        let seeds_bytes = self.config.seed.to_le_bytes();
        let seeds = &[
            b"config",
            seeds_bytes.as_ref(),
            &[self.config.config_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, accounts, signer_seeds);

        transfer_checked(cpi_ctx, amount, decimals)?;

        Ok(())
    }


    pub fn burn_lp_tokens(&mut self, lp_amount_to_burn: u64) -> Result<()> {

        let cpi_program = self.token_program.to_account_info();

        let accounts = Burn {
            mint: self.mint_lp_token.to_account_info(),
            from: self.liquidity_provider_ata_lp_token.to_account_info(),
            authority: self.liquidity_provider.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, accounts);

        burn(cpi_ctx, lp_amount_to_burn)?;

        Ok(())
    }
}