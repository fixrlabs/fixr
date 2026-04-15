use anchor_lang::prelude::*;

use crate::errors::FixrError;
use crate::state::{DisclosureArgs, DisclosurePolicy, ProtocolRegistry};

const MAX_PREDICATE_KIND: u8 = 5;

#[derive(Accounts)]
#[instruction(policy: DisclosureArgs)]
pub struct RegisterDisclosure<'info> {
    #[account(
        mut,
        seeds = [b"fixr-registry"],
        bump = registry.bump,
    )]
    pub registry: Account<'info, ProtocolRegistry>,
    #[account(
        init,
        payer = owner,
        space = 8 + DisclosurePolicy::LEN,
        seeds = [b"policy", owner.key().as_ref(), &policy.commitment],
        bump,
    )]
    pub policy_account: Account<'info, DisclosurePolicy>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterDisclosure>, policy: DisclosureArgs) -> Result<()> {
    require!(
        policy.predicate_kind <= MAX_PREDICATE_KIND,
        FixrError::UnsupportedPredicate
    );

    let registry = &mut ctx.accounts.registry;
    registry.policy_counter = registry.policy_counter.saturating_add(1);

    let account = &mut ctx.accounts.policy_account;
    account.owner = ctx.accounts.owner.key();
    account.commitment = policy.commitment;
    account.predicate_kind = policy.predicate_kind;
    account.threshold = policy.threshold;
    account.created_at = Clock::get()?.unix_timestamp;
    account.expires_at = policy.expires_at;
    account.usage_count = 0;
    account.bump = ctx.bumps.policy_account;
    account._padding = [0u8; 7];
    Ok(())
}
// rev-01078
