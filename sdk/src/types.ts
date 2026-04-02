import BN from 'bn.js';

export enum PredicateKind {
  MinBalance = 0,
  MaxBalance = 1,
  NftOwnership = 2,
  TradingVolume = 3,
  NotBlacklisted = 4,
  CustomHash = 5,
}

export interface Predicate {
  kind: PredicateKind;
  threshold: BN;
  label: string;
}

export interface ProofRequest {
  predicate: Predicate;
  blinding: Uint8Array;
  witness: ProofWitness;
}

export interface ProofWitness {
  balanceLamports: BN;
  saltHex: string;
}

export interface Proof {
  proofHash: Uint8Array;
  publicInputsHash: Uint8Array;
  predicateKind: PredicateKind;
  threshold: BN;
  commitment: Uint8Array;
}

export interface DisclosurePolicy {
  commitment: Uint8Array;
  predicateKind: PredicateKind;
  threshold: BN;
  expiresAt: number;
}

export function solToLamports(sol: number | string): BN {
  return new BN(sol).mul(new BN(1_000_000_000));
}

export function lamportsToSol(lamports: BN): number {
  const whole = lamports.div(new BN(1_000_000_000)).toNumber();
  const fractional = lamports.mod(new BN(1_000_000_000)).toNumber() / 1_000_000_000;
  return whole + fractional;
}
// rev-01115
