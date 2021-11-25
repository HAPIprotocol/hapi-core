use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("Unexpected account has been used.")]
    UnexpectedAccount,
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Non-sequential case ID.")]
    NonSequentialCaseId,
}
