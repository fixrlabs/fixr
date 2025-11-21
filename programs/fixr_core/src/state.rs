use anchor_lang::prelude::*;

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
    pub const LEN: usize = 32 + 2 + 8 + 8 + 8 + 1 + 7;
}
