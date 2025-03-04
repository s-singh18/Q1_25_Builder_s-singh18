use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct Config {
    pub authority: Option<Pubkey>,

    pub fee: u16,

    pub seed: u64,

    pub mint_x: Pubkey,

    pub mint_y: Pubkey,

    pub locked: bool,

    pub config_bump: u8,

    pub lp_bump: u8,
    
}