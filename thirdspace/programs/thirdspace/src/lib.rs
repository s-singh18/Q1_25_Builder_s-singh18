use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

declare_id!("2GdsDdH2jbDMVs7k5jPKZur2hEMaPm5sJA2DHMtRs8zN");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Payments>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Payments>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()?;
        Ok(())
    }

    pub fn tip(ctx: Context<Tip>, amount: u64, fee_bps: u64) -> Result<()> {
        ctx.accounts.tip(amount, fee_bps)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds = [b"state", user.key().as_ref()],
        space = VaultState::INIT_SPACE,
        bump
    )]
    pub state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.state.vault_bump = bumps.vault;
        self.state.state_bump = bumps.state;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Payments<'info> {
    pub user: Signer<'info>,

    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = state.state_bump
    )]
    pub state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"state", user.key().as_ref()],
        bump = state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Payments<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_account = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_account);

        transfer(cpi_context, amount)?;

        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_account = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.state.to_account_info().key.as_ref(),
            &[self.state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_account, signer_seeds);

        transfer(cpi_context, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Close<'info> {
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"state", user.key().as_ref()],
        bump = state.state_bump,
        close = user
    )]
    pub state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"state", state.key().as_ref()],
        bump = state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let balance = self.vault.get_lamports();

        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.state.to_account_info().key.as_ref(),
            &[self.state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_context, balance)?;

        Ok(())
    }
}

#[account]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
    pub total_fees_collected: u64,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1;
}

#[derive(Accounts)]
pub struct Tip<'info> {
    #[account(mut)]
    pub tipper: Signer<'info>,  // Sender of the tip

    #[account(mut)]
    pub recipient: SystemAccount<'info>,  // Receiver of the tip

    #[account(
        mut,
        seeds = [b"state", tipper.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,  // Vault state tracking fees

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,  // Vault PDA storing fees

    pub system_program: Program<'info, System>,
}
impl<'info> Tip<'info> {
    pub fn tip(&mut self, amount: u64, fee_bps: u64) -> Result<()> {
        let tipper = &self.tipper;
        let recipient = &self.recipient;
        let vault = &self.vault;
        let vault_state = &mut self.vault_state;

        require!(fee_bps <= 10_000, ErrorCode::InvalidFee); // Ensure valid fee (0-100%)

        let fee_amount = amount * fee_bps / 10_000;
        let recipient_amount = amount - fee_amount;

        // Get a reference to the system program AccountInfo
        let cpi_program = self.system_program.to_account_info();

        // Transfer fee to the vault
        let cpi_account = Transfer {
            from: tipper.to_account_info(),
            to: vault.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program.clone(), cpi_account);  // Clone reference to reuse later
        transfer(cpi_context, fee_amount)?;

        // Update total fees collected
        vault_state.total_fees_collected += fee_amount;

        // Transfer remaining tip to recipient
        let cpi_account_recipient = Transfer {
            from: tipper.to_account_info(),
            to: recipient.to_account_info(),
        };
        let cpi_context_recipient = CpiContext::new(cpi_program, cpi_account_recipient);
        transfer(cpi_context_recipient, recipient_amount)?;

        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Fee basis points must be between 0 and 10000.")]
    InvalidFee,
}

