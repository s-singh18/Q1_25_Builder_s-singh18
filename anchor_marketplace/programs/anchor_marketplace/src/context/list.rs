use crate::instruction::Listing;
use crate::state::marketplace::Marketplace;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::{MasterEditionAccount, MetadataAccount};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        seeds = [b"marketplace", name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker
    )]
    pub maker_mint_ata: InterfaceAccount<'info, TokenAccount>,
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub listing: Account<'info, Listing>,
    pub collection_int: InterfaceAccount<'info, Mint>,
    pub metadata: Account<'info, MetadataAccount>,
    pub master_edition: Account<'info, MasterEditionAccount>,
    pub metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}
