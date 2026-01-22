# Contributing to fixr

Thanks for taking the time to contribute. This document outlines the workflow
for getting changes merged.

## Development setup

1. Clone the repository

   ```bash
   git clone https://github.com/velyadotdev/fixr.git
   cd fixr
   ```

2. Install the Rust toolchain pinned by ```rust-toolchain.toml```, then build
   the workspace:

   ```bash
   cargo build --workspace
   ```

3. Install SDK dependencies:

   ```bash
   cd sdk
   npm install
   npm test
   ```

## Branching

- ```main``` is always releasable. Direct pushes are discouraged.
- Feature branches: ```feat/<short-description>```
- Fix branches: ```fix/<short-description>```
- Chore branches: ```chore/<short-description>```

## Commit style

We follow Conventional Commits. The allowed types are:
```feat```, ```fix```, ```docs```, ```chore```, ```refactor```,
```test```, ```perf```, ```style```, ```build```, ```ci```.

Scopes should match the crate or package touched, for example
```feat(core): add disclosure predicate``` or ```fix(sdk): handle empty proofs```.

## Pull requests

- Keep pull requests focused. Split large changes into smaller units.
- Include tests for new behaviour. The CI pipeline runs ```cargo test```
  on the workspace and ```npm test``` in ```sdk/```.
- Update the relevant document under ```docs/``` when behaviour changes.

## Reporting issues

Use the issue tracker with one of the labels:
```bug```, ```enhancement```, ```question```, ```documentation```.

Please include a minimum reproduction, the expected behaviour, and the actual
behaviour.

<!-- rev-01061 -->
