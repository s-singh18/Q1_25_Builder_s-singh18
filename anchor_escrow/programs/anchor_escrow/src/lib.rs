use anchor_lang::prelude::*;

declare_id!("4kDN1ppUg9BPyvCjBWYaWUyZWME3J1a8tsioH15XXz5H");

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, receive: u64, deposit: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.send()?;
        ctx.accounts.withdraw()?;
        ctx.accounts.close()?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund()?;
        ctx.accounts.close_refund()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
