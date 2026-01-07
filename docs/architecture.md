# Architecture

fixr is divided into three main surfaces.

## Layers

| Layer | Location | Purpose |
|-------|----------|---------|
| On-chain program | ```programs/fixr_core``` | Registers disclosure policies, anchors proofs, dispatches CPIs to Token-2022 |
| Prover crate | ```crates/zk_prover``` | Produces deterministic selective disclosure proofs. Stub today, SP1 backed later |
| SDK | ```sdk``` | TypeScript client that mirrors the prover and the program API |

## Data flow

1. A user selects a predicate via the SDK or the web demo.
2. The prover crate derives a commitment, hashes public inputs, and binds the
   witness into a proof hash.
3. The on-chain program stores the commitment as a ```DisclosurePolicy``` and
   opens a ```ProofRecord``` PDA when a proof is submitted.
4. Consumer programs or SDK clients verify the proof and act on the result
   (for example, releasing a Token-2022 confidential transfer).

## Account layout

Every core account carries a ```_padding``` field to keep struct sizes
aligned to 8 bytes so the Anchor discriminator plus the struct fits within a
deterministic rent allocation.

## Error surface

The ```FixrError``` enum covers expired policies, replayed nonces, invalid
proofs, and authority mismatches. The SDK maps each code to a stable string so
UI layers can render diagnostics without relying on Anchor internals.

<!-- rev-01046 -->
