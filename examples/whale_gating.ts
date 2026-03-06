import BN from 'bn.js';

import { createClient, PredicateKind } from '../sdk/src';

const { prover, disclosure } = createClient();

interface GatedPool {
  name: string;
  requireMinSol: number;
}

function requireProof(pool: GatedPool, userBalanceLamports: BN): boolean {
  const predicate = disclosure.minSol(pool.requireMinSol);
  const blinding = new Uint8Array(32).fill(11);
  const proof = prover.prove({
    predicate,
    blinding,
    witness: { balanceLamports: userBalanceLamports, saltHex: 'dead' },
  });

  process.stdout.write(`pool=${pool.name} threshold=${proof.threshold.toString()} lamports\n`);
  process.stdout.write(`kind=${PredicateKind[proof.predicateKind]}\n`);

  return userBalanceLamports.gte(proof.threshold);
}

function main() {
  const pools: GatedPool[] = [
    { name: 'crimson', requireMinSol: 100 },
    { name: 'obsidian', requireMinSol: 1000 },
  ];

  const userBalance = new BN(250).mul(new BN(1_000_000_000));
  for (const pool of pools) {
    const ok = requireProof(pool, userBalance);
    process.stdout.write(`access=${ok ? 'granted' : 'denied'}\n`);
  }
}

main();
// rev-01102
