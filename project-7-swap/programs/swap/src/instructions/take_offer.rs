use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
    TransferChecked,
};

use super::transfer_tokens;
use crate::state::Offer;

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    pub offered_token_mint: InterfaceAccount<'info, Mint>,
    pub requested_token_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = requested_token_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_requested_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = offered_token_mint,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_offered_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = requested_token_mint,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_requested_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = offered_token_mint,
        has_one = requested_token_mint,
        seeds = [b"offer", maker.key().as_ref(), offer.id.to_le_bytes().as_ref()],
        bump = offer.bump
    )]
    pub offer: Box<Account<'info, Offer>>,
    #[account(
        mut,
        associated_token::mint = offered_token_mint,
        associated_token::authority = offer,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn transfer_requested_tokens_to_maker(ctx: &Context<TakeOffer>) -> Result<()> {
    let from = &ctx.accounts.taker_requested_token_account;
    let to = &ctx.accounts.maker_requested_token_account;
    let token_amount = &ctx.accounts.offer.requested_token_amount;
    let mint = &ctx.accounts.requested_token_mint;
    let authority = &ctx.accounts.taker;
    let token_program = &ctx.accounts.token_program;

    transfer_tokens(from, to, *token_amount, mint, authority, token_program)?;

    Ok(())
}

pub fn close_offer(ctx: Context<TakeOffer>) -> Result<()> {
    let accounts = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.taker_offered_token_account.to_account_info(),
        mint: ctx.accounts.offered_token_mint.to_account_info(),
        authority: ctx.accounts.offer.to_account_info(),
    };

    let program = ctx.accounts.token_program.to_account_info();
    let seeds = &[
        b"offer",
        ctx.accounts.maker.to_account_info().key.as_ref(),
        &ctx.accounts.offer.id.to_le_bytes()[..],
        &[ctx.accounts.offer.bump],
    ];
    let signer_seeds = [&seeds[..]];

    let cpi_context = CpiContext::new_with_signer(program, accounts, &signer_seeds);
    let amount = ctx.accounts.vault.amount;
    let decimals = ctx.accounts.offered_token_mint.decimals;

    transfer_checked(cpi_context, amount, decimals)?;

    let accounts = CloseAccount {
        account: ctx.accounts.vault.to_account_info(),
        destination: ctx.accounts.taker.to_account_info(),
        authority: ctx.accounts.offer.to_account_info(),
    };

    let program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new_with_signer(program, accounts, &signer_seeds);

    close_account(cpi_context)?;

    Ok(())
}
