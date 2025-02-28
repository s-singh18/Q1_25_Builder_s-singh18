use anchor_lang::prelude::*;

declare_id!("5dXaBJdMLKkDDvP6WWuAo8KUaBQi2AQSrdqjk6BBeeHq");

mod context;
mod state;
mod error;

pub use context::*;

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<InitializeConfig>, points_per_stake: u8, max_stake: u8, freeze_period: u32) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.initialize_config(points_per_stake, max_stake, freeze_period, &ctx.bumps);
        Ok(())
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user(bumps);
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
