import { sha256 } from 'js-sha256';
import BN from 'bn.js';

import { Proof, ProofRequest, PredicateKind } from './types';

/**
 * Deterministic prover that mirrors the Rust crate `zk_prover`.
 *
 * The implementation is a placeholder for the SP1 prover. It derives
 * reproducible hashes so that SDK users can exercise the same interface
 * and tests they will use when the real prover is wired in.
 */
export class Prover {
  constructor(private readonly namespace: string) {}

  prove(request: ProofRequest): Proof {
    const commitment = this.commitment(request.predicate.kind, request.predicate.threshold, request.blinding);
    const publicInputsHash = this.publicInputsHash(request.predicate.kind, request.predicate.threshold, commitment);
    const witnessBytes = this.witnessBytes(request);
    const proofHash = this.proofHash(publicInputsHash, witnessBytes);

    return {
      proofHash,
      publicInputsHash,
      predicateKind: request.predicate.kind,
      threshold: request.predicate.threshold,
      commitment,
    };
  }

  commitment(kind: PredicateKind, threshold: BN, blinding: Uint8Array): Uint8Array {
    const hasher = sha256.create();
    hasher.update(this.namespace);
    hasher.update(new Uint8Array([kind]));
    hasher.update(this.bnToLeBytes(threshold, 8));
    hasher.update(blinding);
    return new Uint8Array(hasher.arrayBuffer());
  }

  publicInputsHash(kind: PredicateKind, threshold: BN, commitment: Uint8Array): Uint8Array {
    const hasher = sha256.create();
    hasher.update(new Uint8Array([kind]));
    hasher.update(this.bnToLeBytes(threshold, 8));
    hasher.update(commitment);
    return new Uint8Array(hasher.arrayBuffer());
  }

  private proofHash(publicInputsHash: Uint8Array, witness: Uint8Array): Uint8Array {
    const hasher = sha256.create();
    hasher.update(publicInputsHash);
    hasher.update(witness);
    return new Uint8Array(hasher.arrayBuffer());
  }

  private witnessBytes(request: ProofRequest): Uint8Array {
    const hasher = sha256.create();
    hasher.update(this.bnToLeBytes(request.witness.balanceLamports, 8));
    hasher.update(request.witness.saltHex);
    return new Uint8Array(hasher.arrayBuffer());
  }

  private bnToLeBytes(value: BN, byteLength: number): Uint8Array {
    const hex = value.toString(16, byteLength * 2);
    const buffer = new Uint8Array(byteLength);
    for (let i = 0; i < byteLength; i += 1) {
      const byte = hex.slice((byteLength - 1 - i) * 2, (byteLength - i) * 2);
      buffer[i] = parseInt(byte, 16);
    }
    return buffer;
  }
}
