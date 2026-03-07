# Plan: Merge the Immediate Test Crates into `soil-test`

Tracking the consolidation of the smaller reusable test crates into the
single `crates/soil-test` package.

Deferred and explicitly out of scope for this pass:
- `soil-runtime-test`
- `sp-runtime-interface-test-wasm`
- `sp-runtime-interface-test-wasm-deprecated`
- all `substrate-*`, `node-*`, and `*runtime*` test crates not listed below

## Goals

- Absorb `soil-network-test` into `soil_test::network`.
- Absorb `soil-service-test` into `soil_test::service`.
- Absorb `soil-test-primitives` into `soil_test::primitives`.
- Move `sp-application-crypto-test` into `soil-test/tests/application_crypto.rs`.
- Move `sp-runtime-interface-test` into `soil-test/tests/runtime_interface.rs`.
- Move `sp-api-test` into `soil-test/tests/api_*`, `soil-test/tests/api/ui`, and
  `soil-test/benches/api_bench.rs`.
- Remove the old crate directories as soon as their replacements land.
- Keep this file updated in every merge commit so progress is accurate at
  `HEAD`.

## Sequence

1. Merge `soil-test-primitives`, `soil-network-test`, and `soil-service-test`.
2. Merge `sp-application-crypto-test`.
3. Merge `sp-runtime-interface-test`.
4. Merge `sp-api-test`.
5. Final consistency pass and spec update.

## Validation Policy

- Per commit: run targeted checks/tests only.
- End of sequence: run `cargo test -p soil-test --release` and
  `cargo test --all --release`.

## Progress

- [x] Step 1: Merge `soil-test-primitives`, `soil-network-test`, and
      `soil-service-test`.
- [x] Step 2: Merge `sp-application-crypto-test`.
- [x] Step 3: Merge `sp-runtime-interface-test`.
- [x] Step 4: Merge `sp-api-test`.
- [x] Step 5: Final consistency pass and spec update.

## Validation Log

- Step 1:
  `cargo check -p soil-test -p soil-service -p soil-network -p staging-node-cli`
- Step 2:
  `cargo test -p soil-test application_crypto --release`
- Step 3:
  `cargo test -p soil-test --test runtime_interface --release`
- Step 4:
  `cargo test -p soil-test --test api_decl_and_impl --release`
  `cargo test -p soil-test --test api_runtime_calls --release`
  `cargo bench -p soil-test --bench api_bench --no-run`
  `RUN_UI_TESTS=1 TRYBUILD=overwrite cargo test -p soil-test --test api_trybuild --release`
- Final:
  `cargo test -p soil-test --release`
  `cargo test -p soil-chain-spec --lib --release`
  `cargo check --tests --release -p soil-client -p soil-network -p soil-grandpa -p soil-beefy -p soil-aura -p soil-babe`
  `cargo test --all --release`
