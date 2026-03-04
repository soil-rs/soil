# Migration Plan: Substrate to Soil

See [specs/migration.md](../specs/migration.md) for the design rationale.

## Strategy

Bottom-up migration following the dependency graph. Rename crates tier by tier,
starting from leaf crates with zero internal dependencies. Each tier is only
started once all its dependencies in prior tiers are fully migrated.

## Dependency Tiers (sp-* crates)

The `sp-*` crates form a clean DAG with no circular non-dev dependencies.

```
Tier 0  (13 crates)  ── no internal deps
Tier 1  ( 3 crates)  ── depends on T0 only
Tier 2  ( 1 crate )  ── depends on T0-1
Tier 3  ( 2 crates)  ── depends on T0-2        ← sp-core, sp-runtime-interface
Tier 4  ( 4 crates)  ── depends on T0-3        ← sp-keystore, sp-trie
Tier 5  ( 1 crate )  ── depends on T0-4        ← sp-state-machine
Tier 6  ( 1 crate )  ── depends on T0-5        ← sp-io
Tier 7  ( 1 crate )  ── depends on T0-6        ← sp-application-crypto
Tier 8  ( 1 crate )  ── depends on T0-7        ← sp-runtime  (41 consumers)
Tier 9  ( 6 crates)  ── depends on T0-8        ← sp-inherents, sp-version, ...
Tier 10 ( 2 crates)  ── depends on T0-9        ← sp-api, sp-timestamp
Tier 11 (14 crates)  ── depends on T0-10       ← consensus, domain primitives
Tier 12 ( 5 crates)  ── depends on T0-11       ← sp-blockchain, sp-consensus-babe, ...
```

Critical path (longest chain):
```
sp-debug-derive (T0) → sp-storage (T1) → sp-externalities (T2) → sp-core (T3)
→ sp-trie (T4) → sp-state-machine (T5) → sp-io (T6) → sp-application-crypto (T7)
→ sp-runtime (T8) → sp-api (T10)
```

## Phases

### Phase 1 — Tier 0 leaf crates (sp-* → soil-*)

Rename, move to `soil/` directory, add `std` feature where missing. These 13
crates can be migrated in parallel since they have no internal dependencies.

| Old Crate                          | New Crate                          | Status |
|------------------------------------|------------------------------------|--------|
| `sp-api-proc-macro`               | `soil-api-proc-macro`              | DONE   |
| `sp-arithmetic`                   | `soil-arithmetic`                  | DONE   |
| `sp-crypto-hashing`               | `soil-crypto-hashing`              | DONE   |
| `sp-database`                     | `soil-database`                    | DONE   |
| `sp-debug-derive`                 | `soil-debug-derive`                | DONE   |
| `sp-maybe-compressed-blob`        | `soil-maybe-compressed-blob`       | DONE   |
| `sp-metadata-ir`                  | `soil-metadata-ir`                 | DONE   |
| `sp-panic-handler`                | `soil-panic-handler`               | DONE   |
| `sp-runtime-interface-proc-macro` | `soil-runtime-interface-proc-macro` | DONE   |
| `sp-std`                          | `soil-std`                         | DONE   |
| `sp-tracing`                      | `soil-tracing`                     | DONE   |
| `sp-version-proc-macro`           | `soil-version-proc-macro`          | DONE   |
| `sp-wasm-interface`               | `soil-wasm-interface`              | DONE   |

### Phase 2 — Tiers 1-8 (sp-* → soil-*, critical path)

Migrate one tier at a time up the critical path. Each tier unlocks the next.

| Tier | Crates | Status |
|------|--------|--------|
| 1 | `sp-crypto-hashing-proc-macro`, `sp-storage`, `sp-weights` | DONE |
| 2 | `sp-externalities` | DONE |
| 3 | `sp-core`, `sp-runtime-interface` | DONE |
| 4 | `sp-crypto-ec-utils`, `sp-keystore`, `sp-rpc`, `sp-trie` | DONE |
| 5 | `sp-state-machine` | DONE |
| 6 | `sp-io` | DONE |
| 7 | `sp-application-crypto` | DONE |
| 8 | `sp-runtime` | DONE |

### Phase 3 — Tiers 9-12 (remaining sp-* → soil-*)

| Tier | Crates | Status |
|------|--------|--------|
| 9 | `sp-inherents`, `sp-keyring`, `sp-npos-elections`, `sp-staking`, `sp-test-primitives`, `sp-version` | DONE |
| 10 | `sp-api`, `sp-timestamp` | DONE |
| 11 | `sp-authority-discovery`, `sp-block-builder`, `sp-consensus`, `sp-consensus-grandpa`, `sp-consensus-pow`, `sp-consensus-slots`, `sp-genesis-builder`, `sp-mixnet`, `sp-mmr-primitives`, `sp-offchain`, `sp-session`, `sp-statement-store`, `sp-transaction-pool`, `sp-transaction-storage-proof` | DONE |
| 12 | `sp-blockchain`, `sp-consensus-aura`, `sp-consensus-babe`, `sp-consensus-beefy`, `sp-consensus-sassafras` | DONE |

### Phase 4 — Merge sp-*/sc-* pairs into soil-*

Merge the `sc-*` implementation into its corresponding `soil-*` crate behind
`#[cfg(feature = "std")]`. Start with thin trait+impl pairs.

| Merged Crate | sp-* Source | sc-* Source | Complexity | Status |
|---|---|---|---|---|
| `soil-keystore` | `sp-keystore` | `sc-keystore` | Low | TODO |
| `soil-offchain` | `sp-offchain` | `sc-offchain` | Low | TODO |
| `soil-tracing` | `sp-tracing` | `sc-tracing` | Low | TODO |
| `soil-authority-discovery` | `sp-authority-discovery` | `sc-authority-discovery` | Medium | TODO |
| `soil-block-builder` | `sp-block-builder` | `sc-block-builder` | Medium | TODO |
| `soil-consensus` | `sp-consensus` | `sc-consensus` | Medium | TODO |
| `soil-consensus-aura` | `sp-consensus-aura` | `sc-consensus-aura` | Medium | TODO |
| `soil-consensus-babe` | `sp-consensus-babe` | `sc-consensus-babe` | Medium | TODO |
| `soil-consensus-beefy` | `sp-consensus-beefy` | `sc-consensus-beefy` | Medium | TODO |
| `soil-consensus-grandpa` | `sp-consensus-grandpa` | `sc-consensus-grandpa` | Medium | TODO |
| `soil-consensus-pow` | `sp-consensus-pow` | `sc-consensus-pow` | Medium | TODO |
| `soil-consensus-slots` | `sp-consensus-slots` | `sc-consensus-slots` | Medium | TODO |
| `soil-mixnet` | `sp-mixnet` | `sc-mixnet` | Medium | TODO |
| `soil-rpc` | `sp-rpc` | `sc-rpc` | Medium | TODO |
| `soil-statement-store` | `sp-statement-store` | `sc-statement-store` | Medium | TODO |
| `soil-transaction-pool` | `sp-transaction-pool` | `sc-transaction-pool` | Medium | TODO |

### Phase 5 — Standalone sc-* crates (sc-* → soil-*)

Rename `sc-*` crates that have no `sp-*` counterpart. Add `std` feature (no_std
build produces empty library).

| Old Crate | New Crate | Status |
|---|---|---|
| `sc-allocator` | `soil-allocator` | DONE |
| `sc-basic-authorship` | `soil-basic-authorship` | DONE |
| `sc-chain-spec` | `soil-chain-spec` | DONE |
| `sc-chain-spec-derive` | `soil-chain-spec-derive` | DONE |
| `sc-cli` | `soil-cli` | DONE |
| `sc-client-api` | `soil-client-api` | DONE |
| `sc-client-db` | `soil-client-db` | DONE |
| `sc-consensus-babe-rpc` | `soil-consensus-babe-rpc` | DONE |
| `sc-consensus-beefy-rpc` | `soil-consensus-beefy-rpc` | DONE |
| `sc-consensus-epochs` | `soil-consensus-epochs` | DONE |
| `sc-consensus-grandpa-rpc` | `soil-consensus-grandpa-rpc` | DONE |
| `sc-consensus-manual-seal` | `soil-consensus-manual-seal` | DONE |
| `sc-executor` | `soil-executor` | DONE |
| `sc-executor-common` | `soil-executor-common` | DONE |
| `sc-executor-polkavm` | `soil-executor-polkavm` | DONE |
| `sc-executor-wasmtime` | `soil-executor-wasmtime` | DONE |
| `sc-informant` | `soil-informant` | DONE |
| `sc-network` | `soil-network` | DONE |
| `sc-network-common` | `soil-network-common` | DONE |
| `sc-network-gossip` | `soil-network-gossip` | DONE |
| `sc-network-light` | `soil-network-light` | DONE |
| `sc-network-statement` | `soil-network-statement` | DONE |
| `sc-network-sync` | `soil-network-sync` | DONE |
| `sc-network-test` | `soil-network-test` | DONE |
| `sc-network-transactions` | `soil-network-transactions` | DONE |
| `sc-network-types` | `soil-network-types` | DONE |
| `sc-proposer-metrics` | `soil-proposer-metrics` | DONE |
| `sc-rpc-api` | `soil-rpc-api` | DONE |
| `sc-rpc-server` | `soil-rpc-server` | DONE |
| `sc-rpc-spec-v2` | `soil-rpc-spec-v2` | DONE |
| `sc-runtime-test` | `soil-runtime-test` | DONE |
| `sc-runtime-utilities` | `soil-runtime-utilities` | DONE |
| `sc-service` | `soil-service` | DONE |
| `sc-service-test` | `soil-service-test` | DONE |
| `sc-state-db` | `soil-state-db` | DONE |
| `sc-storage-monitor` | `soil-storage-monitor` | DONE |
| `sc-sync-state-rpc` | `soil-sync-state-rpc` | DONE |
| `sc-sysinfo` | `soil-sysinfo` | DONE |
| `sc-telemetry` | `soil-telemetry` | DONE |
| `sc-tracing-proc-macro` | `soil-tracing-proc-macro` | DONE |
| `sc-transaction-pool-api` | `soil-transaction-pool-api` | DONE |
| `sc-utils` | `soil-utils` | DONE |
| `mmr-gadget` | `soil-mmr-gadget` | DONE |
| `mmr-rpc` | `soil-mmr-rpc` | DONE |

Status: DONE

### Phase 6 — FRAME / pallets (frame-*, pallet-* → topsoil-*)

Rename and move to `topsoil/` directory. Verify the dependency invariant:
`topsoil → soil` is allowed, `soil → topsoil` is forbidden.

- 140 crates renamed: `frame-*` → `topsoil-*`, `pallet-*` → `topsoil-*`
- `polkadot-sdk-frame` umbrella crate → `topsoil`
- Directory: `frame/` → `topsoil/`
- Proc macro string literals updated for crate resolution
- All source references updated (`frame_support` → `topsoil_support`, `pallet_*` → `topsoil_*`)

Status: DONE

## Known Blockers

| Issue | Affected | Resolution |
|---|---|---|
| `sp-statement-store` depends on `frame-support` | Phase 3 (T11) | Break the dep or defer to Phase 6 |
| Proc-macro crates cannot be `no_std` | All phases | Exempt from `std` feature requirement (build-time only) |
| `sp-runtime` has 41 sc-* consumers | Phase 2 (T8) | Expect large cascade of Cargo.toml updates |

## Per-Crate Migration Checklist

For each crate:

1. Rename `Cargo.toml` package name (`sp-foo` → `soil-foo`)
2. Move directory (`primitives/foo` → `soil/foo`)
3. Ensure `std` feature exists and crate compiles under `no_std`
4. Update workspace `Cargo.toml`: member path, `[workspace.dependencies]` entry
5. Update all downstream `Cargo.toml` references
6. Update all `use sp_foo::` → `use soil_foo::` in source files
7. Verify `cargo check --workspace` passes
8. Commit
