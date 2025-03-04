use anchor_lang::prelude::*;

use anchor_spl::{associated_token::AssociatedToken, token_interface::{mint_to, transfer_checked, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked}};

use crate::state::{Config, error::*};


use constant_product_curve::{self, ConstantProduct};


#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub liquidity_provider: Signer<'info>,

    pub mint_token_x: Box<InterfaceAccount<'info, Mint>>,

    pub mint_token_y: Box<InterfaceAccount<'info, Mint>>,


    #[account(
        mut,
        seeds = [b"lp", seed.to_le_bytes().as_ref()],
        bump = config.lp_bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub mint_lp_token: Box<InterfaceAccount<'info, Mint>>,

    // Swap Initiator ATA account
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
        init_if_needed,
        payer = liquidity_provider,
        associated_token::mint = mint_lp_token,
        associated_token::authority = liquidity_provider,
        associated_token::token_program = token_program,
    )]
    pub liquidity_provider_ata_lp_token: Box<InterfaceAccount<'info, TokenAccount>>,

    // Vault Account To Hold The Being Swapped Tokens, Kinda an Escrow
    #[account(
        init_if_needed,
        payer = liquidity_provider,
        associated_token::mint = mint_token_x,
        associated_token::authority = config,
    )]
    pub vault_token_x: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = liquidity_provider,
        associated_token::mint = mint_token_y,
        associated_token::authority = config,
    )]
    pub vault_token_y: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Box<Account<'info, Config>>,

   // System and token Programs here
   pub system_program: Program<'info, System>,

   pub token_program: Interface<'info, TokenInterface>,

   pub associated_token_program: Program<'info, AssociatedToken>, 
}




impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount_lp: u64, max_x: u64, max_y: u64) -> Result<()> {

        require!(self.config.locked != false, AmmError::AmmPoolLocked);
        require!(amount_lp > 0, AmmError::InvalidAmount);

        let (x, y) = match self.mint_lp_token.supply == 0 && self.vault_token_x.amount == 0 &&
        self.vault_token_y.amount == 0 {
            true => (max_x, max_y),
            false => {
                let amounts = ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_token_x.amount,
                    self.vault_token_y.amount,
                    self.mint_lp_token.supply,
                    amount_lp,
                    6
                ).unwrap();
                (amounts.x, amounts.y)
            }
        };

       
        require!(x <= max_x && y <= max_y, AmmError::SlippageExceeded);


        // Let's deposit the tokens into the vaults
        // token X
        self.deposit_tokens(true, x)?;
        // token Y
        self.deposit_tokens(false, y)?;
        // Mint lp tokens
        self.mint_lp_tokens(amount_lp)?;
        Ok(())
    }


    // Deposit Tokens function
    pub fn deposit_tokens(&self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to) = match is_x {
            true => (self.liquidity_provider_ata_x.to_account_info(), self.vault_token_x.to_account_info()),
            false => (self.liquidity_provider_ata_y.to_account_info(), self.vault_token_y.to_account_info()),
        };

        let (mint, decimals) = match is_x {
            true => (self.mint_token_x.to_account_info(), self.mint_token_x.decimals),
            false => (self.mint_token_y.to_account_info(), self.mint_token_y.decimals)
        };

        let cpi_program = self.token_program.to_account_info();

        // Using TransferChecked because we are swapping tokens
        let cpi_accounts = TransferChecked {
            from,
            mint,
            to,
            authority: self.liquidity_provider.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, amount, decimals)?;

        Ok(())
    }
    

    // Mint LP Tokens
    pub fn mint_lp_tokens(&self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo {
            mint: self.mint_lp_token.to_account_info(),
            to: self.liquidity_provider_ata_lp_token.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let seed_bytes = self.config.seed.to_le_bytes();
        let seeds = &[
            b"config",
            seed_bytes.as_ref(),
            &[self.config.config_bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, amount)?;

        Ok(())
    }

    
    
    
    
}