use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("3bs6aNhD8AT9sujHq4eo7ssPzqDfQrebFu43zRiRem2f");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Init>, seed: u64, fee: u16) -> Result<()> {

        ctx.accounts.init(seed, fee, ctx.bumps.config, ctx.bumps.mint_lp_token)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount_lp: u64, max_x: u64, max_y: u64) -> Result<()> {

        ctx.accounts.deposit(amount_lp, max_x, max_y)?;
        Ok(())
    }


    pub fn withdraw(ctx: Context<Withdraw>, lp_amount: u64, min_x: u64, min_y: u64) -> Result<()> {

        ctx.accounts.withdraw(lp_amount, min_x, min_y)?;

        Ok(())
    }


    pub fn swap(ctx: Context<Swap>, is_x: bool, amount: u64, min_amount: u64) -> Result<()> {
        
        ctx.accounts.swap(is_x, amount, min_amount)?;
        Ok(())
    }
}
