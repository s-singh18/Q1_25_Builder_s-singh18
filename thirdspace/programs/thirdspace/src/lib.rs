use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::program::invoke;

declare_id!("2GdsDdH2jbDMVs7k5jPKZur2hEMaPm5sJA2DHMtRs8zN");

#[program]
pub mod tipping_contract {
    use super::*;

    pub fn tip(ctx: Context<Tip>, amount: u64, fee_bps: u64) -> Result<()> {
        let tipper = &ctx.accounts.tipper;
        let recipient = &ctx.accounts.recipient;
        let platform_fee_account = &ctx.accounts.platform_fee_account;

        require!(fee_bps <= 10_000, ErrorCode::InvalidFee); // Max fee is 100%

        let fee_amount = amount * fee_bps / 10_000;
        let recipient_amount = amount - fee_amount;

        // Transfer to recipient
        invoke(
            &system_instruction::transfer(tipper.key, recipient.key, recipient_amount),
            &[
                tipper.to_account_info(),
                recipient.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Transfer fee to platform account
        invoke(
            &system_instruction::transfer(tipper.key, platform_fee_account.key, fee_amount),
            &[
                tipper.to_account_info(),
                platform_fee_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Tip<'info> {
    #[account(mut)]
    pub tipper: Signer<'info>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    #[account(mut)]
    pub platform_fee_account: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Fee basis points must be between 0 and 10000.")]
    InvalidFee,
}