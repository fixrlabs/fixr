<p align="center">
  <img src="./assets/banner.png" alt="fixr banner" width="100%" />
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.75.0-ca7e4b?style=flat-square&logo=rust" alt="Rust" />
  <img src="https://img.shields.io/badge/TypeScript-5.4-3178c6?style=flat-square&logo=typescript" alt="TypeScript" />
  <img src="https://img.shields.io/badge/Solana-mainnet-14f195?style=flat-square&logo=solana" alt="Solana" />
  <img src="https://img.shields.io/badge/license-MIT-8b0000?style=flat-square" alt="License" />
  <img src="https://img.shields.io/badge/version-0.3.1-d4a017?style=flat-square" alt="Version" />
  <img src="https://img.shields.io/badge/build-passing-0a0a0a?style=flat-square" alt="Build" />
</p>

<p align="center">
  <a href="https://fixr.red"><img src="https://img.shields.io/badge/website-fixr.red-8b0000?style=for-the-badge" alt="Website" /></a>
  <a href="https://x.com/fixrprotocol"><img src="https://img.shields.io/badge/x-@fixrprotocol-0a0a0a?style=for-the-badge&logo=x" alt="X" /></a>
</p>

# fixr --- In the red light, only what matters appears.

ZK Selective Disclosure Protocol on Solana — develop only what matters.

`fixr` is a selective disclosure toolkit for Solana. Users publish commitments
instead of raw data, generate proofs that satisfy a predicate, and let consumer
programs gate access without ever reading the underlying balance or transaction
history. The protocol combines an SP1-style prover with Token-2022 Confidential
Transfer so that gated actions can stay encrypted end to end.

## Architecture

```mermaid
graph LR
  User([User Wallet]) --> UI[fixr UI]
  UI --> Prover[SP1 Prover]
  Prover --> Proof{{Selective Disclosure Proof}}
  Proof --> Consumer[Consumer Protocol]
  UI -.-> T22[Token-2022 Confidential Transfer]
  Proof -.-> Registry[(On-chain Registry)]
```

The on-chain registry lives in `programs/fixr_core`. It stores a commitment for
every policy, anchors proof hashes into PDAs, and exposes CPI entrypoints that
other Solana programs can call to verify a predicate before acting on a user.

## Features

| Capability | Crate / package | Status |
|------------|-----------------|--------|
| Selective disclosure policies | `programs/fixr_core` | Stable |
| Deterministic prover stub | `crates/zk_prover` | Stable, SP1 backend is queued |
| TypeScript client | `sdk` | Stable |
| Token-2022 CPI wrapper | `programs/fixr_core` | Stable |
| CLI examples | `examples/` | Stable |

## Installation

```bash
git clone https://github.com/velyadotdev/fixr.git
cd fixr
cargo build --workspace
cd sdk && npm install && npm test
```

## Usage

```ts
import BN from 'bn.js';
import { createClient, solToLamports } from './sdk/src';

const { prover, disclosure } = createClient();
const predicate = disclosure.minSol(100);
const blinding = new Uint8Array(32).fill(42);

const proof = prover.prove({
  predicate,
  blinding,
  witness: {
    balanceLamports: solToLamports(250),
    saltHex: 'c0ffee',
  },
});

console.log(Buffer.from(proof.proofHash).toString("hex"));
```

The same call pattern compiles against the stub prover today and against the SP1
backend when it is wired in.

## Repository layout

```
fixr/
  programs/fixr_core/        anchor program and instructions
  crates/zk_prover/          deterministic prover skeleton
  sdk/                       typescript client and tests
  examples/                  simple proof and whale gating demos
  scripts/                   deploy and proof inspection helpers
  docs/                      architecture, getting started, disclosure types
```

## Links

- Website: https://fixr.red
- X: https://x.com/fixrprotocol

## License

Released under the MIT License. See `LICENSE` for details.

