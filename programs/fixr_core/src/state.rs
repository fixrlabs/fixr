//! On-chain account layouts for the fixr core program.

use anchor_lang::prelude::*;

/// Configuration that governs the protocol registry.
#[account]
pub struct ProtocolRegistry {
    pub authority: Pubkey,
    pub fee_basis_points: u16,
    pub proof_counter: u64,
    pub policy_counter: u64,
    pub created_at: i64,
    pub bump: u8,
    pub _padding: [u8; 7],
}

impl ProtocolRegistry {
    /// Byte layout without Anchor's discriminator.
    pub const LEN: usize = 32 + 2 + 8 + 8 + 8 + 1 + 7;
}

/// Stored selective-disclosure policy.
#[account]
pub struct DisclosurePolicy {
    pub owner: Pubkey,
    pub commitment: [u8; 32],
    pub predicate_kind: u8,
    pub threshold: u64,
    pub created_at: i64,
    pub expires_at: i64,
    pub usage_count: u64,
    pub bump: u8,
    pub _padding: [u8; 7],
}

impl DisclosurePolicy {
    pub const LEN: usize = 32 + 32 + 1 + 8 + 8 + 8 + 8 + 1 + 7;
}

/// Stored proof artifact produced off-chain and anchored on-chain.
#[account]
pub struct ProofRecord {
    pub policy: Pubkey,
    pub submitter: Pubkey,
    pub proof_hash: [u8; 32],
    pub public_inputs_hash: [u8; 32],
    pub submitted_at: i64,
    pub verified: bool,
    pub bump: u8,
    pub _padding: [u8; 6],
}

impl ProofRecord {
    pub const LEN: usize = 32 + 32 + 32 + 32 + 8 + 1 + 1 + 6;
}

/// Argument types shared between instruction handlers.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct InitConfig {
    pub fee_basis_points: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct DisclosureArgs {
    pub predicate_kind: u8,
    pub threshold: u64,
    pub commitment: [u8; 32],
    pub expires_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct GenerateProofArgs {
    pub policy_seed: [u8; 16],
    pub nonce: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct VerifyProofArgs {
    pub proof_hash: [u8; 32],
    pub public_inputs_hash: [u8; 32],
}
