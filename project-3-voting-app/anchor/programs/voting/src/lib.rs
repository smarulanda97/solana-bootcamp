use anchor_lang::prelude::*;

declare_id!("74dCTDkTRbZTogHjgCDSQNn59JQBgd2GwkzUCaZqGHW6");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        poll_id: u64,
        poll_description: String,
        poll_start_time: u64,
        poll_end_time: u64,
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll_account;
        poll.id = poll_id;
        poll.description = poll_description;
        poll.start_time = poll_start_time;
        poll.end_time = poll_end_time;
        poll.candidate_count = 0;

        Ok(())
    }

    pub fn initialize_candidate(
        ctx: Context<InitializeCandidate>,
        _poll_id: u64,
        name: String,
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll_account;
        poll.candidate_count += 1;

        let candidate = &mut ctx.accounts.candidate_account;
        candidate.name = name;
        candidate.votes = 0;

        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, poll_id: u64, candidate_name: String) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate_account;
        candidate.votes += 1;

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct PollAccount {
    pub id: u64,
    #[max_len(280)]
    pub description: String,
    pub start_time: u64,
    pub end_time: u64,
    pub candidate_count: u64,
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    #[max_len(60)]
    name: String,
    votes: u64,
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, name: String)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        init,
        payer = signer,
        space = 8 + CandidateAccount::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), name.as_bytes()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate_name: String)]
pub struct Vote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_bytes()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,
}