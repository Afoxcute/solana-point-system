use anchor_lang::error_code;

#[error_code]
pub enum AppErrorCode {
    #[msg("Not enough quest")]
    NotEnoughQuest,
    #[msg("Wrong Authority")]
    WrongAuthority,
}
