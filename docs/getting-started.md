# Getting started

## Prerequisites

- Rust toolchain pinned by ```rust-toolchain.toml```
- Node.js 18 or later
- A Solana keypair under ```~/.config/solana/id.json``` for deploy scripts

## Clone and build

```bash
git clone https://github.com/fixrlabs/fixr.git
cd fixr
cargo build --workspace
cd sdk && npm install && npm test
```

## Run the examples

```bash
cd sdk && npm install
npx ts-node ../examples/simple_proof.ts
npx ts-node ../examples/whale_gating.ts
```

The examples print the proof hash and public inputs hash for a whale-tier
gating predicate.

## Deploy to devnet

```bash
CLUSTER=https://api.devnet.solana.com ./scripts/deploy.sh
```

The deploy script wraps ```cargo build-sbf``` and ```solana program deploy```.

<!-- rev-01118 -->
