# SP1 integration notes

The current ```zk_prover``` crate and the SDK prover emit deterministic
hashes, not real ZK proofs. This page records the design decisions that the
SP1 integration will follow when the upstream toolchain stabilises.

## Circuit layout

The circuit reads a fixed 32-byte blinding from the secret witness, reads
predicate configuration from the public inputs, and emits a commitment plus a
witness hash.

## Proving pipeline

1. The SDK generates a ```ProofRequest``` from the user's wallet data.
2. The request is serialised to the ELF format expected by ```sp1-sdk```.
3. SP1 runs the prover and returns a proof blob plus public inputs.
4. The SDK submits the proof to the on-chain program together with the policy
   PDA so that the verification happens in a single transaction.

## Constraints during migration

- The SDK interface must not change. Integrators that ship today with the stub
  prover should upgrade by bumping the dependency only.
- The Rust crate keeps the ```prove``` and ```verify``` functions as the
  stable public surface; everything else is internal.

<!-- rev-01097 -->
