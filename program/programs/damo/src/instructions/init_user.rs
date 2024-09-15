pub use crate::errors::AppErrorCode;
use crate::state::user_data::UserData;
use crate::{constants::MAX_QUEST, AppData};
use anchor_lang::prelude::*;

pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
    ctx.accounts.user.energy = MAX_QUEST;
    ctx.accounts.user.last_login = Clock::get()?.unix_timestamp;
    ctx.accounts.user.authority = ctx.accounts.signer.key();
    Ok(())
}

#[derive(Accounts)]
#[instruction(level_seed: String)]
pub struct InitUser<'info> {
    #[account(
        init,
        payer = signer,
        space = 1000, // 8+32+x+1+8+8+8 But taking 1000 to have space to expand easily.
        seeds = [b"user".as_ref(), signer.key().as_ref()],
        bump,
    )]
    pub user: Account<'info, UserData>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 1000, // 8 + 8 for anchor account discriminator and the u64. Using 1000 to have space to expand easily.
        seeds = [level_seed.as_ref()],
        bump,
    )]
    pub user_data: Account<'info, UserData>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
