use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use solana_program::{ed25519_program, hash::hash, sysvar::instructions::load_instruction_at_checked};

use crate::{errors::CustomError, state::Bet};

#[derive(Accounts)]
pub struct ResolveBet<'info> {
    pub house: Signer<'info>,
    
    #[account(mut)]
    ///CHECK: This is safe
    pub player: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    
    #[account(
        mut,
        close = player,
        seeds = [b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub bet: Account<'info, Bet>,

    #[account(
        address = solana_program::sysvar::instructions::ID
    )]
    /// CHECK: This is safe
    pub instructions_sysvar: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> ResolveBet<'info> {
    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        let ix = load_instruction_at_checked(
            0, 
            &self.instructions_sysvar.to_account_info()
        )?;

        // Make sure the instruction is addressed to the ed25519 program
        require_keys_eq!(ix.program_id, ed25519_program::ID, CustomError::Ed25519Program);
        
        // Make sure there are no accounts present
        require_eq!(ix.accounts.len(), 0, CustomError::Ed25519Accounts);

        // Get the first index
        let signatures = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        require_eq!(signatures.len(), 1, CustomError::Ed25519DataLength);
        let signature_in_program = &signatures[0];

        require!(signature_in_program.is_verifiable, CustomError::Ed25519Header);

        // Ensure public keys match
        require_keys_eq!(
            signature_in_program.public_key.unwrap(),
            self.house.key(),
            CustomError::Ed25519Pubkey
        );

        // Ensure signatures match
        require!(
            signature_in_program.signature.unwrap().eq(sig),
            CustomError::Ed25519Signature
        );

        // Ensure messages match
        require!(
            signature_in_program.message.as_ref().unwrap().eq(&self.bet.to_slice()),
            CustomError::Ed25519Message
        );

        Ok(())
    }

    pub fn resolve_bet(&mut self, sig: &[u8], bumps: &ResolveBetBumps) -> Result<()> {
        let hash = hash(sig).to_bytes();

        let mut hash_16: [u8; 16] = [0; 16];
        hash_16.copy_from_slice(&hash[0..16]);
        let lower = u128::from_le_bytes(hash_16);

        hash_16.copy_from_slice(&hash[16..32]);
        let upper = u128::from_le_bytes(hash_16);

        // We will get a number between 1 - 100
        let roll = lower.wrapping_add(upper).wrapping_rem(100) as u8 + 1;

        if self.bet.roll > roll {
            let payout = (self.bet.amount as u128)
                .checked_mul(10000 - 150 as u128) // 150 = 1.5% House edge
                .unwrap()
                .checked_div(self.bet.roll as u128 - 1)
                .unwrap()
                .checked_div(100)
                .unwrap() as u64;

            let cpi_program = self.system_program.to_account_info();

            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info(),
            };
    
            let seeds = [b"vault", &self.house.key().to_bytes()[..], &[bumps.vault]];
    
            let signer_seeds = &[&seeds[..]][..];
    
            let ctx = CpiContext::new_with_signer(
                cpi_program,
                cpi_accounts,
                signer_seeds
            );
            transfer(ctx, payout)?;
        }

        Ok(())
    }
}
