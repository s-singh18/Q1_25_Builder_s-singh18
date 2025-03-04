use anchor_lang::prelude::*;

use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenInterface}};

use crate::state::Config;


#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Init<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    pub mint_token_x: InterfaceAccount<'info, Mint>,

    pub mint_token_y: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = initializer,
        seeds = [b"lp", config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config,
    )]   
    pub mint_lp_token: InterfaceAccount<'info, Mint>,
/* 
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_token_x,
        associated_token::authority = config,
    )]
    pub vault_token_x: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_token_y,
        associated_token::authority = mint_token_x,
    )]
    pub vault_token_y: Box<InterfaceAccount<'info, Mint>>,

    THESE 2 TOKEN ACCOUNTS SHOULD BE INITIALIZED UPON DEPOSIT.
*/
    #[account(
        init,
        payer = initializer,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump,
        space = 8 + Config::INIT_SPACE,
    )]
    pub config: Box<Account<'info, Config>>,

    pub associated_token: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Init<'info> {

    pub fn init(&mut self, seed: u64, fee: u16, bump: u8, lp_bump: u8) -> Result<()> {

        self.config.set_inner(Config {
            mint_x: self.mint_token_x.key(),
            mint_y: self.mint_token_y.key(),
            authority: Some(self.initializer.key()),
            config_bump: bump,
            lp_bump: lp_bump,
            locked: false,
            fee: fee,
            seed: seed,
        });

        Ok(())
    }
}