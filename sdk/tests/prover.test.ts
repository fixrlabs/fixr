import BN from 'bn.js';

import { Prover, DisclosureBuilder, PredicateKind } from '../src';

describe('Prover', () => {
  const namespace = 'fixr-policy-v1';
  const prover = new Prover(namespace);
  const disclosure = new DisclosureBuilder(namespace);

  test('produces deterministic proofs for the same witness', () => {
    const predicate = disclosure.minSol(10);
    const blinding = new Uint8Array(32).fill(3);
    const request = {
      predicate,
      blinding,
      witness: { balanceLamports: new BN(100_000_000_000), saltHex: 'aa' },
    };
    const first = prover.prove(request);
    const second = prover.prove(request);
    expect(first.proofHash).toEqual(second.proofHash);
    expect(first.publicInputsHash).toEqual(second.publicInputsHash);
  });

  test('changes proof hash when the witness differs', () => {
    const predicate = disclosure.minSol(10);
    const blinding = new Uint8Array(32).fill(7);
    const a = prover.prove({
      predicate,
      blinding,
      witness: { balanceLamports: new BN(100_000_000_000), saltHex: '11' },
    });
    const b = prover.prove({
      predicate,
      blinding,
      witness: { balanceLamports: new BN(200_000_000_000), saltHex: '11' },
    });
    expect(a.proofHash).not.toEqual(b.proofHash);
  });

  test('predicate kind threads into the public inputs', () => {
    const predicate = disclosure.notBlacklisted();
    const proof = prover.prove({
      predicate,
      blinding: new Uint8Array(32),
      witness: { balanceLamports: new BN(0), saltHex: '00' },
    });
    expect(proof.predicateKind).toBe(PredicateKind.NotBlacklisted);
  });
});
// rev-01084
