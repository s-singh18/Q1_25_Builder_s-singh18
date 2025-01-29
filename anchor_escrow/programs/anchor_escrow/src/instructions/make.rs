use crate::EscrowState;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: Box<InterfaceAccount<'info, Mint>>, // ex: BONK
    pub mint_b: Box<InterfaceAccount<'info, Mint>>, // ex: USDC
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        payer = maker,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        space = 8 + EscrowState::INIT_SPACE,
    )]
    pub escrow: Box<Account<'info, EscrowState>>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    pub fn init_escrow(&mut self, seed: u64, recieve: u64, bump: &MakeBumps) -> Result<()> {
        self.escrow.set_inner(EscrowState {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            recieve_amount: recieve,
            bump: bump.escrow,
        });
        Ok(())
    }

    pub fn deposit(&mut self, deposit: u64) -> Result<()> {
        let cpi_program: AccountInfo<'_> = self.token_program.to_account_info();
        let cpi_account: TransferChecked<'_> = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
            mint: self.mint_a.to_account_info(),
        };
        let cpi_ctx: CpiContext<'_, '_, '_, '_, _> = CpiContext::new(cpi_program, cpi_account);
        transfer_checked(cpi_ctx, deposit, self.mint_a.decimals)?;
        Ok(())
    }
}
