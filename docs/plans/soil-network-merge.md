# Plan: Merge `soil-mixnet` into `subsoil` and consolidate the `soil-network` stack

Tracking the merge of the runtime-facing mixnet protocol and the remaining
service-side networking crates into the flattened `soil-network` layout. See
`docs/specs/merge.md` for the architectural rationale.

## Goals

- Merge `soil-mixnet` into `subsoil::mixnet`.
- Merge the service-side network crates into `crates/soil/network`:
  - `soil-network-common`
  - `soil-network-types`
  - `soil-network-light`
  - `soil-network-sync`
  - `soil-network-gossip`
  - `soil-network-transactions`
  - `soil-network-statement`
  - `sc-statement-store`
  - `sc-mixnet`
- Use hard renames only. Old crates are removed as soon as their replacements
  land.
- Keep `soil-network-test` and `soil-statement-store` standalone in this phase.
- Keep this file updated in every merge commit so progress is accurate at
  `HEAD`.

## Public API Targets

- `subsoil::mixnet`
- `soil_network::common`
- `soil_network::types`
- `soil_network::light`
- `soil_network::sync`
- `soil_network::gossip`
- `soil_network::transactions`
- `soil_network::statement`
- `soil_network::statement_store`
- `soil_network::mixnet`

## Sequence

1. Create this progress file.
2. Merge `soil-mixnet` into `subsoil`.
3. Merge `soil-network-common` into `soil_network::common`.
4. Merge `soil-network-types` into `soil_network::types`.
5. Merge `soil-network-light` into `soil_network::light`.
6. Merge `soil-network-sync` into `soil_network::sync`.
7. Merge `soil-network-gossip` into `soil_network::gossip`.
8. Merge `soil-network-transactions` into `soil_network::transactions`.
9. Merge `soil-network-statement` and `sc-statement-store` into
   `soil_network::{statement,statement_store}`.
10. Merge `sc-mixnet` into `soil_network::mixnet`.
11. Final consistency pass and spec update.

## Validation Policy

- Per commit: run targeted checks/tests only.
- End of sequence: run `cargo test --all --release`.

## Progress

- [x] Step 1: Create this progress file.
- [x] Step 2: Merge `soil-mixnet` into `subsoil`.
- [x] Step 3: Merge `soil-network-common` into `soil_network::common`.
- [x] Step 4: Merge `soil-network-types` into `soil_network::types`.
- [x] Step 5: Merge `soil-network-light` into `soil_network::light`.
- [x] Step 6: Merge `soil-network-sync` into `soil_network::sync`.
- [ ] Step 7: Merge `soil-network-gossip` into `soil_network::gossip`.
- [ ] Step 8: Merge `soil-network-transactions` into `soil_network::transactions`.
- [ ] Step 9: Merge `soil-network-statement` and `sc-statement-store` into
      `soil_network::{statement,statement_store}`.
- [ ] Step 10: Merge `sc-mixnet` into `soil_network::mixnet`.
- [ ] Step 11: Final consistency pass and spec update.

## Validation Log

- Step 2:
  `cargo check -p subsoil -p topsoil-mixnet -p kitchensink-runtime -p sc-mixnet -p staging-node-cli`
- Step 3:
  `cargo check -p soil-network -p soil-network-sync -p soil-grandpa -p soil-service -p soil-network-test`
- Step 4:
  `cargo check -p soil-network -p soil-network-sync -p soil-grandpa -p soil-beefy -p sc-authority-discovery -p sc-offchain -p sc-mixnet -p soil-consensus -p staging-node-cli`
- Step 5:
  `cargo check -p soil-network -p soil-network-light -p soil-service -p soil-network-test -p staging-node-cli`
- Step 6:
  `cargo check -p soil-network -p soil-service -p soil-informant -p soil-grandpa -p soil-beefy -p soil-network-test -p staging-node-cli`
- Step 7:
  `cargo check -p soil-network -p soil-network-gossip -p soil-grandpa -p soil-beefy -p staging-node-cli`
- Step 8:
  `cargo check -p soil-network -p soil-network-transactions -p soil-service -p staging-node-cli`
- Step 9:
  `cargo check -p soil-network -p sc-rpc -p staging-node-cli -p soil-network-test`
- Step 10:
  `cargo check -p soil-network -p soil-cli -p soil-rpc-api -p sc-rpc -p node-rpc -p staging-node-cli`
- Step 11:
  `cargo test --all --release`
