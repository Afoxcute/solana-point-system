pub use crate::errors::AppErrorCode;
pub use anchor_lang::prelude::*;
pub use session_keys::{session_auth_or, Session, SessionError};
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
use instructions::*;

declare_id!("MkabCfyUD6rBTaYHpgKBBpBo5qzWA2pK2hrGGKMurJt");

#[program]
pub mod solana-point-system {

    use super::*;

    pub fn init_user(ctx: Context<InitUser>, _level_seed: String) -> Result<()> {
        init_user::init_user(ctx)
    }


    #[session_auth_or(
        ctx.accounts.user.authority.key() == ctx.accounts.signer.key(),
        UserErrorCode::WrongAuthority
    )]
    pub fn task_init(ctx: Context<TaskInit>, _level_seed: String, counter: u16) -> Result<()> {
        task_init::task_init(ctx, counter, 1)
    }
}
