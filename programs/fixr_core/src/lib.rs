//! fixr core Anchor program.
//!
//! This module wires together the instructions for registering disclosure
//! policies, submitting selective disclosure proofs, and dispatching
//! confidential Token-2022 transfers that depend on verified proofs.

use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

pub use instructions::*;

declare_id!("Fixr1111111111111111111111111111111111111111");

#[program]
pub mod fixr_core {
    use super::*;

    /// Initialise the protocol registry and assign the authority key.
    pub fn initialize(ctx: Context<Initialize>, cfg: InitConfig) -> Result<()> {
        instructions::init::handler(ctx, cfg)
    }

    /// Register a disclosure policy and persist its commitment on chain.
    pub fn register_disclosure(
        ctx: Context<RegisterDisclosure>,
        policy: DisclosureArgs,
    ) -> Result<()> {
        instructions::disclosure::handler(ctx, policy)
    }

    /// Generate a proof request slot for an off-chain prover to fulfil.
    pub fn generate_proof(ctx: Context<GenerateProof>, args: GenerateProofArgs) -> Result<()> {
        instructions::generate_proof::handler(ctx, args)
    }

    /// Verify a submitted proof against the registered disclosure policy.
    pub fn verify_proof(ctx: Context<VerifyProof>, args: VerifyProofArgs) -> Result<()> {
        instructions::verify_proof::handler(ctx, args)
    }
}
