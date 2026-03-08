# Plan: Move the Client Import Pipeline from `soil-consensus` to `soil-client`

Tracking the hard move of the non-engine block import and import queue API from
`soil-consensus` into `soil-client::import`. This keeps the consensus-family
crates as a leaf dependency group while moving client/network sync plumbing to
the client layer where it belongs.

## Goals

- Move the block import and import queue modules from `soil-consensus` into
  `crates/soil-client/src/import/`.
- Expose the moved API as a flat `soil_client::import::*` surface.
- Remove the moved exports from `soil-consensus`.
- Rewrite all downstream consumers to import from `soil-client`.
- Remove the `soil-network -> soil-consensus` dependency edge.
- Keep this file updated with progress and validation at `HEAD`.

## Sequence

1. Create `soil_client::import` and move the import pipeline modules.
2. Remove the moved modules and reexports from `soil-consensus`.
3. Rewrite all downstream consumers to use `soil_client::import::*`.
4. Clean up dependencies so `soil-network` no longer depends on
   `soil-consensus`.
5. Run targeted validation and final workspace validation.

## Validation Policy

- Run targeted tests/checks after the move lands.
- End with `cargo test --all --release`.

## Progress

- [x] Step 1: Create `soil_client::import` and move the import pipeline
      modules.
- [x] Step 2: Remove the moved modules and reexports from `soil-consensus`.
- [x] Step 3: Rewrite all downstream consumers to use
      `soil_client::import::*`.
- [x] Step 4: Clean up dependencies so `soil-network` no longer depends on
      `soil-consensus`.
- [x] Step 5: Run targeted validation and final workspace validation.

## Validation Log

- `cargo test -p soil-client --lib`
- `cargo check -p soil-network -p soil-service -p soil-manual-seal`
- `cargo check -p soil-aura -p soil-babe -p soil-pow -p soil-grandpa -p soil-beefy`
- `cargo check -p soil-test-staging-node-cli -p soil-test-node-client -p soil-test-node-runtime-client`
- `cargo test -p soil-network-test --release`
- `cargo test -p soil-network --release`
- `cargo test -p soil-test-staging-node-cli --release`
- `cargo test --all --release`
