use anchor_lang::prelude::*;

declare_id!("9XYjZ8ycM9JwpgBUzPxNmgt2LqtABFEjC9AJGKi8kwvm");

mod context;
mod errors;
mod state;
use context::*;
use errors::*;

#[program]
pub mod anchor_marketplace {
    use super::*;

    pub fn initialize(mut ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.initialize(name, fee, &ctx.bumps);
        Ok(())
    }

    pub fn listing(ctx: Context<List>) -> Result<()> {
        Ok(())
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        Ok(())
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
