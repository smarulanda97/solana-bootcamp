use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use super::transfer_tokens;
use crate::constants::ANCHOR_DISCRIMINATOR;
use crate::state::Offer;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        mut,
        associated_token::authority = maker,
        associated_token::mint = offered_token_mint,
        associated_token::token_program = token_program,
    )]
    pub maker_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mint::token_program = token_program)]
    pub offered_token_mint: InterfaceAccount<'info, Mint>,
    #[account(mint::token_program = token_program)]
    pub requested_token_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = maker,
        space = ANCHOR_DISCRIMINATOR + Offer::INIT_SPACE,
        seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>,
    #[account(
        init,
        payer = maker,
        associated_token::authority = offer,
        associated_token::mint = offered_token_mint,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn transfer_offered_tokens_to_vault(ctx: &Context<MakeOffer>, token_amount: u64) -> Result<()> {
    let from = &ctx.accounts.maker_token_account;
    let to = &ctx.accounts.vault;
    let mint = &ctx.accounts.offered_token_mint;
    let authority = &ctx.accounts.maker;
    let token_program = &ctx.accounts.token_program;

    transfer_tokens(from, to, token_amount, mint, authority, token_program)?;

    Ok(())
}

pub fn save_offer(ctx: Context<MakeOffer>, id: u64, requested_token_amount: u64) -> Result<()> {
    ctx.accounts.offer.set_inner(Offer {
        id,
        maker: ctx.accounts.maker.key(),
        offered_token_mint: ctx.accounts.offered_token_mint.key(),
        requested_token_mint: ctx.accounts.requested_token_mint.key(),
        requested_token_amount,
        bump: ctx.bumps.offer,
    });

    Ok(())
}
