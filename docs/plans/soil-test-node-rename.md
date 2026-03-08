# Soil Test Node Rename Plan

## Summary
- Move the full leaf-shaped staging/kitchensink node cluster from `crates/substrate/...` into flattened `contrib/soil-test-staging-node-*` packages.
- Rename the full `substrate-test-*` family into flattened `crates/soil-test-node-*` packages, with `substrate-test-runtime-transaction-pool` specifically becoming `soil-test-node-runtime-txpool`.
- Apply the rename end-to-end for both families: package names, folder names, workspace dependency keys, and Rust crate IDs/imports all change to the new `soil-test-*` names.
- Maintain this file as the execution log, commit in logical increments during the work, and finish by running `cargo test --all --release` to green.

## Crate Mapping
| Old path | New path | Old package | New package | Notes |
| --- | --- | --- | --- | --- |
| `crates/substrate/staging-node-cli` | `contrib/soil-test-staging-node-cli` | `staging-node-cli` | `soil-test-staging-node-cli` | binary/default-run `soil-test-staging-node` |
| `crates/substrate/staging-chain-spec-builder` | `contrib/soil-test-staging-node-spec-builder` | `staging-chain-spec-builder` | `soil-test-staging-node-spec-builder` | binary `soil-test-staging-node-spec-builder` |
| `crates/substrate/staging-node-inspect` | `contrib/soil-test-staging-node-inspect` | `staging-node-inspect` | `soil-test-staging-node-inspect` | |
| `crates/substrate/node-primitives` | `contrib/soil-test-staging-node-primitives` | `node-primitives` | `soil-test-staging-node-primitives` | |
| `crates/substrate/node-rpc` | `contrib/soil-test-staging-node-rpc` | `node-rpc` | `soil-test-staging-node-rpc` | |
| `crates/substrate/node-testing` | `contrib/soil-test-staging-node-testing` | `node-testing` | `soil-test-staging-node-testing` | |
| `crates/substrate/node-bench` | `contrib/soil-test-staging-node-bench` | `node-bench` | `soil-test-staging-node-bench` | binary follows package |
| `crates/substrate/substrate-cli-test-utils` | `contrib/soil-test-staging-node-cli-test-utils` | `substrate-cli-test-utils` | `soil-test-staging-node-cli-test-utils` | |
| `crates/substrate/node-runtime-generate-bags` | `contrib/soil-test-staging-node-generate-bags` | `node-runtime-generate-bags` | `soil-test-staging-node-generate-bags` | binary follows package |
| `crates/substrate/kitchensink-runtime` | `contrib/soil-test-staging-node-runtime` | `kitchensink-runtime` | `soil-test-staging-node-runtime` | |
| `crates/substrate/substrate-test-client` | `crates/soil-test-node-client` | `substrate-test-client` | `soil-test-node-client` | |
| `crates/substrate/substrate-test-runtime` | `crates/soil-test-node-runtime` | `substrate-test-runtime` | `soil-test-node-runtime` | |
| `crates/substrate/substrate-test-runtime-client` | `crates/soil-test-node-runtime-client` | `substrate-test-runtime-client` | `soil-test-node-runtime-client` | |
| `crates/substrate/substrate-test-runtime-transaction-pool` | `crates/soil-test-node-runtime-txpool` | `substrate-test-runtime-transaction-pool` | `soil-test-node-runtime-txpool` | crate id `soil_test_node_runtime_txpool` |

## Phase Checklist
- [x] Create execution log before refactor edits.
- [ ] Update workspace members and dependency key scaffolding.
- [ ] Rename `substrate-test-*` crates and downstream uses.
- [ ] Move staging/kitchensink cluster to `contrib/soil-test-staging-node-*`.
- [ ] Rename command/doc/test surface to `soil-test-staging-node` names.
- [ ] Validate leaf boundary: no `crates/*` manifest depends on `contrib/soil-test-staging-node-*`.
- [ ] Run focused `cargo check` and command smoke checks.
- [ ] Run `cargo check --all`.
- [ ] Run `cargo test --all --release` and fix fallout until green.

## Progress Log
- 2026-03-08: Created plan and execution log. Verified the key workspace entries still use the old `staging-node-*`, `node-*`, and `substrate-test-*` paths and package names.

## Commit Log
- Pending.

## Validation Results
- Pending.
- 2026-03-08: Renamed `substrate-test-*` crates to `soil-test-node-*`, including `soil-test-node-runtime-txpool` for the former transaction-pool crate. Updated workspace members, workspace dependency keys, downstream manifests, Rust crate IDs/imports, and `Cargo.lock`.
- 2026-03-08: `cargo check -p soil-test-node-runtime --all-targets` passed after the rename.
