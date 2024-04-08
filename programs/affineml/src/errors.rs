use anchor_lang::prelude::*;

#[error_code]
pub enum BizError {
    #[msg("the task is ended")]
    EndedError,
    #[msg("the task must be ended")]
    MustEndError,
    #[msg("too many user")]
    TooManyUserError,
    #[msg("too few user")]
    TooFewUserError,
    #[msg("reveal error")]
    RevealError,
    #[msg("mint error")]
    MintError,
    #[msg("claimer error")]
    ClaimerError,
    #[msg("have claimed error")]
    HaveClaimedError,
    #[msg("have commited error")]
    HaveCommitedError,
}
