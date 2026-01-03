//! Proof generation skeleton modelled after the SP1 workflow.
//!
//! The real prover pipeline compiles circuits into a RISC-V ELF and feeds them
//! to SP1. In this repository we expose a deterministic stub that mirrors the
//! final API so SDK consumers can develop against it today.

pub mod circuit;
pub mod commitment;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

/// Errors returned by the prover.
#[derive(Debug, Error)]
pub enum ProverError {
    #[error("unsupported predicate kind: {0}")]
    Unsupported(u8),
    #[error("missing private input: {0}")]
    MissingInput(&'static str),
    #[error("serialization failure: {0}")]
    Serialize(String),
}

/// Public inputs shared between prover and verifier.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PublicInputs {
    pub predicate_kind: u8,
    pub threshold: u64,
    pub commitment: [u8; 32],
}

/// Proof artifact emitted by the prover.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Proof {
    pub proof_hash: [u8; 32],
    pub public_inputs_hash: [u8; 32],
    pub bytes: Vec<u8>,
}

/// Private witness values held only by the user.
#[derive(Debug, Clone)]
pub struct Witness {
    pub balance_lamports: u64,
    pub blinding: [u8; 32],
}

/// Generate a proof for the supplied predicate.
pub fn prove(inputs: &PublicInputs, witness: &Witness) -> Result<Proof, ProverError> {
    if inputs.predicate_kind > 5 {
        return Err(ProverError::Unsupported(inputs.predicate_kind));
    }

    let mut hasher = Sha256::new();
    hasher.update(&[inputs.predicate_kind]);
    hasher.update(&inputs.threshold.to_le_bytes());
    hasher.update(&inputs.commitment);
    let public_inputs_hash: [u8; 32] = hasher.finalize().into();

    let mut proof_hasher = Sha256::new();
    proof_hasher.update(&public_inputs_hash);
    proof_hasher.update(&witness.balance_lamports.to_le_bytes());
    proof_hasher.update(&witness.blinding);
    let proof_hash: [u8; 32] = proof_hasher.finalize().into();

    let mut bytes = Vec::with_capacity(96);
    bytes.extend_from_slice(&public_inputs_hash);
    bytes.extend_from_slice(&proof_hash);
    bytes.extend_from_slice(&witness.blinding);

    Ok(Proof {
        proof_hash,
        public_inputs_hash,
        bytes,
    })
}

/// Verify that a proof was produced for the supplied public inputs.
pub fn verify(proof: &Proof, inputs: &PublicInputs) -> bool {
    if proof.bytes.len() < 96 {
        return false;
    }
    let mut hasher = Sha256::new();
    hasher.update(&[inputs.predicate_kind]);
    hasher.update(&inputs.threshold.to_le_bytes());
    hasher.update(&inputs.commitment);
    let computed: [u8; 32] = hasher.finalize().into();
    computed == proof.public_inputs_hash && proof.bytes[..32] == computed
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_inputs() -> PublicInputs {
        PublicInputs {
            predicate_kind: 1,
            threshold: 10,
            commitment: [7u8; 32],
        }
    }

    fn sample_witness() -> Witness {
        Witness {
            balance_lamports: 12_345,
            blinding: [9u8; 32],
        }
    }

    #[test]
    fn proof_round_trip() {
        let inputs = sample_inputs();
        let proof = prove(&inputs, &sample_witness()).expect("prove succeeds");
        assert!(verify(&proof, &inputs));
    }

    #[test]
    fn reject_unsupported_predicate() {
        let mut inputs = sample_inputs();
        inputs.predicate_kind = 9;
        let err = prove(&inputs, &sample_witness()).unwrap_err();
        assert!(matches!(err, ProverError::Unsupported(_)));
    }
}
// rev-01042
