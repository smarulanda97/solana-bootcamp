#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("Bc6GJdPyJM4rjQoq2zuDW2WHveAZpQVeHTuvNgJUgTfF");

#[program]
pub mod crudapp {
    use super::*;

    pub fn create_journal_entry(
        ctx: Context<CreateJournalEntry>,
        title: String,
        message: String,
    ) -> Result<()> {
        let journal_entry_account = &mut ctx.accounts.journal_entry;
        journal_entry_account.author = *ctx.accounts.author.key;
        journal_entry_account.title = title;
        journal_entry_account.message = message;

        Ok(())
    }

    pub fn update_journal_entry(
        ctx: Context<UpdateJournalEntry>,
        _title: String,
        message: String,
    ) -> Result<()> {
        let journal_entry_account = &mut ctx.accounts.journal_entry;
        journal_entry_account.message = message;

        Ok(())
    }

    pub fn delete_journal_entry(_ctx: Context<DeleteJournalEntry>, _title: String) -> Result<()> {
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct JournalEntryState {
    pub author: Pubkey,
    #[max_len(50)]
    pub title: String,
    #[max_len(1000)]
    pub message: String,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateJournalEntry<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), author.key().as_ref()],
        bump,
        space = 8 + JournalEntryState::INIT_SPACE,
        payer = author
    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    #[account(mut)]
    pub author: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateJournalEntry<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), author.key().as_ref()],
        bump,
        realloc = 8 + JournalEntryState::INIT_SPACE,
        realloc::payer = author,
        realloc::zero = true
    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    #[account(mut)]
    pub author: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteJournalEntry<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), author.key().as_ref()],
        bump,
        close = author
    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    #[account(mut)]
    pub author: Signer<'info>,

    pub system_program: Program<'info, System>,
}
