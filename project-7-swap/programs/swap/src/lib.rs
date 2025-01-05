pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5WkbHpXJ2b7dxgCa6hctg4W1htZ6NXLmL9M4HxJy9ujk");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(
        ctx: Context<MakeOffer>,
        id: u64,
        offered_token_amount: u64,
        requested_token_amount: u64,
    ) -> Result<()> {
        instructions::make_offer::transfer_offered_tokens_to_vault(&ctx, offered_token_amount)?;
        instructions::make_offer::save_offer(ctx, id, requested_token_amount)
    }

    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
        instructions::take_offer::transfer_requested_tokens_to_maker(&ctx)?;
        instructions::take_offer::close_offer(ctx)
    }
}
