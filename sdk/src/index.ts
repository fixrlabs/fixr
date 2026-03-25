export * from './types';
export * from './prover';
export * from './disclosure';
export * from './solana';

import { Prover } from './prover';
import { DisclosureBuilder } from './disclosure';

/**
 * Convenience entry point that wires together a prover and a disclosure
 * builder backed by the same commitment namespace.
 */
export function createClient(namespace = 'fixr-policy-v1') {
  const prover = new Prover(namespace);
  const disclosure = new DisclosureBuilder(namespace);
  return { prover, disclosure };
}
// rev-01110
