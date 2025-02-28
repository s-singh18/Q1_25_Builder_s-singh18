use anchor_lang::prelude::*;

#[account]
// #[derive(InitSpace)]
pub struct Marketplace {
    pub maker: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
    pub bump: u8,
}

impl Space for Marketplace {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1; // bytes
}
