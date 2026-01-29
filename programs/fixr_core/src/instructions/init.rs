use anchor_lang::prelude::*;

use crate::errors::FixrError;
use crate::state::{InitConfig, ProtocolRegistry};

#[derive(Accounts)]
#[instruction(cfg: InitConfig)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + ProtocolRegistry::LEN,
        seeds = [b"fixr-registry"],
        bump,
    )]
    pub registry: Account<'info, ProtocolRegistry>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, cfg: InitConfig) -> Result<()> {
    require!(cfg.fee_basis_points <= 2_000, FixrError::FeeOutOfRange);

    let registry = &mut ctx.accounts.registry;
    registry.authority = ctx.accounts.authority.key();
    registry.fee_basis_points = cfg.fee_basis_points;
    registry.proof_counter = 0;
    registry.policy_counter = 0;
    registry.created_at = Clock::get()?.unix_timestamp;
    registry.bump = ctx.bumps.registry;
    registry._padding = [0u8; 7];
    Ok(())
}
// rev-01066
