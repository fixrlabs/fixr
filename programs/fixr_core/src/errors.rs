//! Error codes for the core Anchor program.

use anchor_lang::prelude::*;

#[error_code]
pub enum FixrError {
    #[msg("Predicate kind is not supported by this build.")]
    UnsupportedPredicate,
    #[msg("The disclosure policy has expired.")]
    PolicyExpired,
    #[msg("Proof hash did not match the registered commitment.")]
    InvalidProof,
    #[msg("Requested fee exceeds the configured maximum.")]
    FeeOutOfRange,
    #[msg("The submitted nonce has already been used.")]
    NonceReplay,
    #[msg("Authority mismatch for the requested action.")]
    UnauthorizedAuthority,
    #[msg("Account data is shorter than required layout.")]
    AccountTooSmall,
}
// rev-01029
