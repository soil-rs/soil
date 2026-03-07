# Plan: Merge the RPC stack into `soil-rpc`

Tracking the hard merge of the RPC-family crates into a single flattened
`crates/soil-rpc` crate. This keeps RPC API, handlers, server, v2 surface,
endpoint helpers, and client helpers in one crate while leaving the topsoil-tied
frame RPC helpers separate.

## Goals

- Move the existing `soil-rpc` crate to the flattened path `crates/soil-rpc`.
- Merge `soil-rpc-api`, `sc-rpc`, `soil-rpc-server`, `soil-rpc-spec-v2`,
  `soil-mmr-rpc`, `soil-sync-state-rpc`,
  `substrate-state-trie-migration-rpc`, and `substrate-rpc-client` into
  `soil-rpc`.
- Keep `substrate-frame-rpc-system` and `substrate-frame-rpc-support`
  separate.
- Use hard renames only. Remove each old crate as soon as its replacement lands.
- Keep this file updated in every merge commit so progress is accurate at
  `HEAD`.

## Sequence

1. Create this progress file.
2. Flatten the existing `soil-rpc` crate to `crates/soil-rpc`.
3. Merge `soil-rpc-api` into `soil_rpc::api`.
4. Merge `sc-rpc` into the root handler modules of `soil-rpc`.
5. Merge `soil-rpc-server` into `soil_rpc::server`.
6. Merge `soil-rpc-spec-v2` into `soil_rpc::v2`.
7. Merge `soil-mmr-rpc` into `soil_rpc::mmr`.
8. Merge `soil-sync-state-rpc` into `soil_rpc::sync_state`.
9. Merge `substrate-state-trie-migration-rpc` into
   `soil_rpc::state_trie_migration`.
10. Merge `substrate-rpc-client` into `soil_rpc::client`.
11. Final consistency pass and spec update.

## Validation Policy

- Per commit: run targeted checks/tests only.
- End of sequence: run `cargo test --all --release`.

## Progress

- [x] Step 1: Create this progress file.
- [x] Step 2: Flatten the existing `soil-rpc` crate to `crates/soil-rpc`.
- [ ] Step 3: Merge `soil-rpc-api` into `soil_rpc::api`.
- [ ] Step 4: Merge `sc-rpc` into the root handler modules of `soil-rpc`.
- [ ] Step 5: Merge `soil-rpc-server` into `soil_rpc::server`.
- [ ] Step 6: Merge `soil-rpc-spec-v2` into `soil_rpc::v2`.
- [ ] Step 7: Merge `soil-mmr-rpc` into `soil_rpc::mmr`.
- [ ] Step 8: Merge `soil-sync-state-rpc` into `soil_rpc::sync_state`.
- [ ] Step 9: Merge `substrate-state-trie-migration-rpc` into
      `soil_rpc::state_trie_migration`.
- [ ] Step 10: Merge `substrate-rpc-client` into `soil_rpc::client`.
- [ ] Step 11: Final consistency pass and spec update.

## Validation Log

- Step 2:
  `cargo test -p soil-rpc`
- Step 3:
  `cargo check -p soil-rpc -p substrate-frame-rpc-system -p substrate-frame-rpc-support -p soil-babe -p substrate-rpc-client`
- Step 4:
  `cargo check -p soil-rpc -p soil-service -p node-rpc -p staging-node-cli`
- Step 5:
  `cargo check -p soil-rpc -p soil-service -p staging-node-cli`
- Step 6:
  `cargo test -p soil-rpc --release`
  `cargo check -p soil-service -p staging-node-cli`
- Step 7:
  `cargo check -p soil-rpc -p node-rpc -p staging-node-cli`
- Step 8:
  `cargo check -p soil-rpc -p node-rpc -p staging-node-cli`
- Step 9:
  `cargo check -p soil-rpc -p node-rpc -p staging-node-cli -p topsoil-state-trie-migration`
- Step 10:
  `cargo check -p soil-rpc -p substrate-cli-test-utils -p frame-remote-externalities -p staging-node-cli`
- Step 11:
  `cargo test --all --release`
