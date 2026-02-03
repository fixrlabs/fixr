import BN from 'bn.js';

import { DisclosureBuilder, PredicateKind } from '../src';

describe('DisclosureBuilder', () => {
  const builder = new DisclosureBuilder('fixr-policy-v1');

  test('minSol sets the threshold in lamports', () => {
    const predicate = builder.minSol(25);
    expect(predicate.kind).toBe(PredicateKind.MinBalance);
    expect(predicate.threshold.eq(new BN(25_000_000_000))).toBe(true);
    expect(predicate.label).toBe('min_sol_25');
  });

  test('ownsNft requires a 64 character hex string', () => {
    expect(() => builder.ownsNft('abc')).toThrow();
  });

  test('asPolicy computes a future expiry', () => {
    const predicate = builder.maxSol(10);
    const commitment = new Uint8Array(32).fill(1);
    const now = Math.floor(Date.now() / 1000);
    const policy = builder.asPolicy(predicate, commitment, 3600);
    expect(policy.expiresAt).toBeGreaterThanOrEqual(now + 3599);
    expect(policy.predicateKind).toBe(PredicateKind.MaxBalance);
  });
});
