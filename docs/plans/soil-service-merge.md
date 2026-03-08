# Plan: Finish the Remaining Merge Work with `soil-service` and `soil-txpool`

Tracking the hard rename of `sc-transaction-pool` to `soil-txpool` and the
final `soil-service` merge into a flattened `main/soil-service` crate.

Locked standalones for this phase:
- `soil-cli`
- `soil-telemetry`
- `soil-chain-spec`
- `soil-chain-spec-macros`

## Goals

- Rename `sc-transaction-pool` to `soil-txpool` at `main/soil-txpool`.
- Flatten `soil-service` to `main/soil-service`.
- Merge `soil-basic-authorship`, `soil-proposer-metrics`, `soil-informant`,
  and `soil-sysinfo` into `soil-service`.
- Keep `soil-cli`, `soil-telemetry`, and `soil-chain-spec + derive` separate.
- Remove old crates as soon as their replacements land.
- Keep this file updated in every merge commit so progress is accurate at
  `HEAD`.

## Sequence

1. Create this progress file.
2. Update `docs/specs/merge.md` to the final remaining scope.
3. Rename `sc-transaction-pool` to `soil-txpool`.
4. Flatten the existing `soil-service` crate to `main/soil-service`.
5. Merge `soil-proposer-metrics` and `soil-basic-authorship`.
6. Merge `soil-informant` into `soil_service::informant`.
7. Merge `soil-sysinfo` into `soil_service::sysinfo`.
8. Final consistency pass and spec update.

## Validation Policy

- Per commit: run targeted checks/tests only.
- End of sequence: run `cargo test --all --release`.

## Progress

- [x] Step 1: Create this progress file.
- [x] Step 2: Update `docs/specs/merge.md` to the final remaining scope.
- [x] Step 3: Rename `sc-transaction-pool` to `soil-txpool`.
- [x] Step 4: Flatten the existing `soil-service` crate to `main/soil-service`.
- [x] Step 5: Merge `soil-proposer-metrics` and `soil-basic-authorship`.
- [x] Step 6: Merge `soil-informant` into `soil_service::informant`.
- [x] Step 7: Merge `soil-sysinfo` into `soil_service::sysinfo`.
- [x] Step 8: Final consistency pass and spec update.

## Validation Log

- Step 3:
  `cargo check -p soil-txpool -p soil-service -p soil-manual-seal -p soil-test-staging-node-cli -p soil-test-staging-node-bench -p substrate-frame-rpc-system -p soil-test-node-runtime-txpool`
- Step 4:
  `cargo check -p soil-service -p soil-service-test -p soil-test-staging-node-testing -p soil-cli -p subkey -p soil-test-staging-node-inspect -p soil-test-staging-node-cli-test-utils`
- Step 5:
  `cargo check -p soil-service -p soil-manual-seal -p soil-test-staging-node-bench -p soil-test-staging-node-cli`
- Step 6:
  `cargo check -p soil-service -p soil-test-staging-node-cli`
- Step 7:
  `cargo check -p soil-service -p soil-test-staging-node-cli`
- Step 8:
  `cargo test --all --release`
