use anchor_lang::prelude::*;

declare_id!("818CBEy6ueRQKeFEQhRpWbNYrCprtbe6PuiE6WCX38Hf");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorites(
        context: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Greeting from {}", context.program_id);

        let user_pubkey = context.accounts.user.key();

        msg!(
            "User {}'s favorite number is {}, color is {}, hobbies are {}",
            user_pubkey,
            number,
            color,
            hobbies.join(",")
        );

        context
            .accounts
            .favorites
            .set_inner(Favorites::from((number, color, hobbies)));

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,
    #[max_len(50)]
    pub color: String,
    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

impl From<(u64, String, Vec<String>)> for Favorites {
    fn from(values: (u64, String, Vec<String>)) -> Self {
        Self {
            number: values.0,
            color: values.1,
            hobbies: values.2,
        }
    }
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump 
    )]
    pub favorites: Account<'info, Favorites>,
    pub system_program: Program<'info, System>,
}
