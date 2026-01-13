use anchor_lang::prelude::*;
use sha2::{Digest, Sha256};

use crate::errors::FixrError;
use crate::state::{DisclosurePolicy, ProofRecord, VerifyProofArgs};

#[derive(Accounts)]
#[instruction(args: VerifyProofArgs)]
pub struct VerifyProof<'info> {
    #[account(mut)]
    pub policy: Account<'info, DisclosurePolicy>,
    #[account(
        init,
        payer = submitter,
        space = 8 + ProofRecord::LEN,
        seeds = [b"proof", policy.key().as_ref(), &args.proof_hash],
        bump,
    )]
    pub proof_record: Account<'info, ProofRecord>,
    #[account(mut)]
    pub submitter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<VerifyProof>, args: VerifyProofArgs) -> Result<()> {
    let policy = &mut ctx.accounts.policy;
    let now = Clock::get()?.unix_timestamp;
    require!(now <= policy.expires_at, FixrError::PolicyExpired);

    let mut hasher = Sha256::new();
    hasher.update(&args.proof_hash);
    hasher.update(&args.public_inputs_hash);
    let digest = hasher.finalize();

    let expected_prefix = &policy.commitment[..8];
    let actual_prefix = &digest[..8];
    require!(expected_prefix == actual_prefix, FixrError::InvalidProof);

    policy.usage_count = policy.usage_count.saturating_add(1);

    let record = &mut ctx.accounts.proof_record;
    record.policy = policy.key();
    record.submitter = ctx.accounts.submitter.key();
    record.proof_hash = args.proof_hash;
    record.public_inputs_hash = args.public_inputs_hash;
    record.submitted_at = now;
    record.verified = true;
    record.bump = ctx.bumps.proof_record;
    record._padding = [0u8; 6];
    Ok(())
}
// rev-01054
