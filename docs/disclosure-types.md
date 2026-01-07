# Disclosure types

The prover supports a fixed set of predicate kinds. Each predicate maps to a
```PredicateKind``` constant in both the Rust crate and the TypeScript SDK.

| Kind | Value | Threshold meaning |
|------|-------|-------------------|
| MinBalance | 0 | Minimum balance in lamports |
| MaxBalance | 1 | Maximum balance in lamports |
| NftOwnership | 2 | First 8 bytes of the collection hash |
| TradingVolume | 3 | Volume denominated in USD cents |
| NotBlacklisted | 4 | Reserved, threshold must be zero |
| CustomHash | 5 | Arbitrary commitment chosen by the caller |

## Choosing a predicate

Most integrations use ```MinBalance``` for tier gating, ```TradingVolume```
for airdrop eligibility, and ```NotBlacklisted``` for light-touch
compliance. Higher order predicates are built by composing commitments
off-chain before calling the SDK.

## Versioning

The namespace passed to ```createClient(``````fixr-policy-v1``````)``` is
mixed into every commitment. When the predicate catalogue grows, bump the
version to avoid colliding with policies registered under the old rules.

<!-- rev-01047 -->
