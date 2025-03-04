use anchor_lang::prelude::*;

use anchor_spl::{associated_token::AssociatedToken, token_interface::{TokenAccount, Mint, TransferChecked, transfer_checked, TokenInterface}};
use constant_product_curve::{ConstantProduct, LiquidityPair};


use crate::state::*;


#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub swap_caller: Signer<'info>,
    // Let's have the mints
    pub mint_token_x: Box<InterfaceAccount<'info, Mint>>,

    pub mint_token_y: Box<InterfaceAccount<'info, Mint>>,

    // Token Accounts For Both Tokens to swapper
    #[account(
        init_if_needed,
        payer = swap_caller,
        associated_token::mint = mint_token_x,
        associated_token::authority = swap_caller,
    )]
    pub swap_caller_ata_x: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = swap_caller,
        associated_token::mint = mint_token_y,
        associated_token::authority = swap_caller,
    )]
    pub swap_caller_ata_y: Box<InterfaceAccount<'info, TokenAccount>>,

    // Vaults token accounts that hold the tokens to be swapped.
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

    // Liquidity mint Token
    #[account(
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub mint_lp_token: Box<InterfaceAccount<'info, Mint>>,

    // Config Account Also here
    #[account(
        //has_one = mint_token_x,
        //has_one = mint_token_y,
        seeds= [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
    )]
    pub config: Box<Account<'info, Config>>,

    // Tokens And System Programs
    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Interface<'info, TokenInterface>,
}



impl<'info> Swap<'info> {

    pub fn swap(&mut self, is_x: bool, amount: u64, min_amount: u64) -> Result<()> {
        require!(self.config.locked == false, AmmError::AmmPoolLocked);
        require!(amount > 0, AmmError::InvalidAmount);

        let mut curve = ConstantProduct::init(
            self.vault_ata_x.amount,
            self.vault_ata_y.amount,
            self.mint_lp_token.supply,
            self.config.fee,
            None,
        ).map_err(AmmError::from)?;

        let p = match is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y,
        };

        let res = curve.swap(p, amount, min_amount).map_err(AmmError::from)?;
        require!(res.deposit != 0, AmmError::InvalidAmount);
        require!(res.withdraw != 0, AmmError::InvalidAmount);

        self.deposit_tokens(is_x, res.deposit)?;

        self.withdraw_tokens(!is_x, res.withdraw)?;


        Ok(())
    }


    pub fn deposit_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {

        let (mint_token, from, to, decimals) = match is_x {
            true => (
                self.mint_token_x.to_account_info(),
                self.swap_caller_ata_x.to_account_info(),
                self.vault_ata_x.to_account_info(),
                self.mint_token_x.decimals,
            ),
            false => (
                self.mint_token_y.to_account_info(),
                self.swap_caller_ata_y.to_account_info(),
                self.vault_ata_y.to_account_info(),
                self.mint_token_y.decimals,
            ),
        };

        let accounts = TransferChecked {
            from,
            mint: mint_token,
            to,
            authority: self.swap_caller.to_account_info()
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), accounts);

        transfer_checked(cpi_ctx, amount, decimals)?;

        Ok(())
    }

    pub fn withdraw_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {

        let (mint_token, from, to, decimals) = match is_x {
            true => (
                self.mint_token_x.to_account_info(),
                self.vault_ata_x.to_account_info(),
                self.swap_caller_ata_x.to_account_info(),
                self.mint_token_x.decimals
            ),
            false => (
                self.mint_token_y.to_account_info(),
                self.vault_ata_y.to_account_info(),
                self.swap_caller_ata_y.to_account_info(),
                self.mint_token_y.decimals
            )
        };

        let accounts = TransferChecked {
            from,
            mint: mint_token,
            to,
            authority: self.config.to_account_info()
        };

        let seed_bytes = self.config.seed.to_le_bytes();
        let seeds = &[
            b"config",
            seed_bytes.as_ref(),
            &[self.config.config_bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), accounts, signer_seeds);

        transfer_checked(cpi_ctx, amount, decimals)?;

        Ok(())
    }
}