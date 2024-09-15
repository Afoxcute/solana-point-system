pub use crate::errors::AppErrorCode;
pub use crate::state::app_data::AppData;
use crate::state::user_data::UserData;
use anchor_lang::prelude::*;
use session_keys::{Session, SessionToken};

pub fn task_init(mut ctx: Context<TaskInit>, counter: u16, amount: u64) -> Result<()> {
    let account: &mut &mut TaskInit<'_> = &mut ctx.accounts;
    account.user.update_Quest()?;
    account.user.print()?;

    if account.user.quest < amount {
        return err!(AppErrorCode::NotEnoughQuest);
    }

    account.user.last_id = counter;
    account.user.task_init(amount)?;
    account.app_data.on_task_initialized(amount)?;

    msg!(
        "You initialized a task and got 1 task. You have {} task and {} quest left.",
        ctx.accounts.user.wood,
        ctx.accounts.user.energy
    );
    Ok(())
}

#[derive(Accounts, Session)]
#[instruction(level_seed: String)]
pub struct TaskInit<'info> {
    #[session(
        // The ephemeral key pair signing the transaction
        signer = signer,
        // The authority of the user account which must have created the session
        authority = user.authority.key()
    )]
    // Session Tokens are passed as optional accounts
    pub session_token: Option<Account<'info, SessionToken>>,

    // There is one UserData account
    #[account(
        mut,
        seeds = [b"user".as_ref(), user.authority.key().as_ref()],
        bump,
    )]
    pub user: Account<'info, UserData>,


    #[account(
        init_if_needed,
        payer = signer,
        space = 1000,
        seeds = [level_seed.as_ref()],
        bump,
    )]
    pub user_data: Account<'info, UserData>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
