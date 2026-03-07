# Plan: Merge `soil-consensus` and Selectable Consensus Crates

Tracking the merge of the remaining std-only consensus support and selectable
consensus crates into the new flattened `soil-*` layout. See
`docs/specs/merge.md` for the architectural rationale.

## Goals

- Merge `sc-consensus`, `sc-consensus-slots`, and `soil-consensus-epochs` into
  `crates/soil-consensus`.
- Merge the selectable consensus engines into flattened crates:
  - `crates/soil-pow`
  - `crates/soil-aura`
  - `crates/soil-babe`
  - `crates/soil-grandpa`
  - `crates/soil-beefy`
- Merge the BABE, GRANDPA, and BEEFY RPC crates into their engine crates as
  `rpc` modules.
- Use hard renames only. Old crates are removed as soon as their replacements
  land.
- Keep this file updated in every merge commit so progress is accurate at
  `HEAD`.

## Sequence

1. Create this progress file.
2. Merge `sc-consensus` into `soil-consensus`.
3. Merge `sc-consensus-slots` into `soil-consensus::slots`.
4. Merge `soil-consensus-epochs` into `soil-consensus::epochs`.
5. Merge `sc-consensus-pow` into `soil-pow`.
6. Merge `sc-consensus-aura` into `soil-aura`.
7. Merge `sc-consensus-babe` and `soil-consensus-babe-rpc` into `soil-babe`.
8. Merge `sc-consensus-grandpa` and `soil-consensus-grandpa-rpc` into
   `soil-grandpa`.
9. Merge `sc-consensus-beefy` and `soil-consensus-beefy-rpc` into `soil-beefy`.
10. Final consistency pass and spec update.

## Validation Policy

- Per commit: run targeted checks/tests only.
- End of sequence: run `cargo test --all --release`.

## Progress

- [x] Step 1: Create this progress file.
- [x] Step 2: Merge `sc-consensus` into `soil-consensus`.
- [x] Step 3: Merge `sc-consensus-slots` into `soil-consensus::slots`.
- [x] Step 4: Merge `soil-consensus-epochs` into `soil-consensus::epochs`.
- [x] Step 5: Merge `sc-consensus-pow` into `soil-pow`.
- [x] Step 6: Merge `sc-consensus-aura` into `soil-aura`.
- [x] Step 7: Merge `sc-consensus-babe` and `soil-consensus-babe-rpc` into
      `soil-babe`.
- [ ] Step 8: Merge `sc-consensus-grandpa` and
      `soil-consensus-grandpa-rpc` into `soil-grandpa`.
- [ ] Step 9: Merge `sc-consensus-beefy` and `soil-consensus-beefy-rpc` into
      `soil-beefy`.
- [ ] Step 10: Final consistency pass and spec update.

## Validation Log

- Step 2:
  `cargo check -p soil-consensus -p soil-manual-seal -p soil-network-sync -p soil-service -p staging-node-cli -p substrate-test-runtime-client`
- Step 3:
  `cargo check -p soil-consensus -p soil-manual-seal -p soil-network-sync -p soil-service -p staging-node-cli -p substrate-test-runtime-client`
- Step 4:
  `cargo check -p soil-consensus -p soil-manual-seal -p soil-network-sync -p soil-service -p staging-node-cli -p substrate-test-runtime-client`
- Step 5:
  `cargo check -p soil-pow -p staging-node-cli`
- Step 6:
  `cargo check -p soil-aura -p soil-manual-seal -p staging-node-cli`
- Step 7:
  `cargo check -p soil-babe -p node-rpc -p soil-manual-seal -p soil-sync-state-rpc -p staging-node-cli`
