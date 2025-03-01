use anchor_lang::{prelude::*, solana_program::{self, ed25519_program, sysvar::instructions::load_instruction_at_checked}, system_program::Transfer}
use anchor_instruction_sysvar::Ed25519InstructionSignatures;

use crate::instruction;

#[derive(Accounts)]
#[instruction(seed: u128)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub house: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        seeds = [b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub bet: Account<'info, Bet>,
    #[account(
        address = solana_program::sysvar::instructions::ID,
    )]
    pub instruction_sysvar: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> ResolveBet<'info> {
    pub fn verify_ed25519_signature(&mut self, sig:&[u8]) -> Result<()> {
        let ix: Instruction = load_instruction_at_checked(index: 0, instruction_sysvar_account_info: &self.instruction_sysvar)?;
        require_keys_eq!(ix.program_id, ed25519_program::ID);
        require_eq!(ix.accounts.len(), 0);
        let signatures: Vec<Ed25519InstructionSignatures> = Ed25519InstructionSignatures::unpack(data: sig)?.0;
        require_eq!((signatures.len(), 1));

        let signature: &Ed25519InstructionSignatures = &signatures[0];
        require_eq!(signature.is_verifiable, true);
        require_keys_eq!(signature.public_key.unwrap(), self.house.key());
        require_eq!(signature.signature.unwrap().eq(sig), true);

        require!(signature.message.as_ref().unwrap().eq(self.bet.to_slice(())));

        
        Ok(())
    }

    pub fn resolve_bet(&mut self, sig: &[u8], bumps: &ResolveBetBumps) -> Result<()> {
        let hash: [u8; 32] = hash(val:sig).to_bytes();
        let mut hash_16: [u8; 16] = [0u8;16];
        hash_16.copy_from_slice(src:&hash[0..16]);
        let lower: u128 = u128::from_le_bytes(hash_16);

        hash_16.copy_from_slice(src: &hash[16..32]));
        let upper: u128 = u128::from_be_bytes(hash_16);

        let roll: u8 = lower.wrapping_add(upper).wrapping_rem(100) as u8 + 1;
        if self.bet.roll > roll {
            let payout: u128 = (self.bet.amount as u128).checked_mul(10000 - 150 as u128).unwrap().checked_div(self.bet.roll as u128).unwrap().checked_div(10000).unwrap();
            let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();

            let cpi_accounts: Transfer<'_> = Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info()
            };

            let seeds: [&[u8]; 3] = [b'vault', &self.house.key().to_bytes()[..], &[bumps.vault]];
            let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..][..]];
            let cpi_context: CpiContext<'_, '_, '_, '_, >
        
        }
    }

}
