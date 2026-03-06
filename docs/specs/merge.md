# Crate Merge Plan

## Motivation

The current workspace has 119 non-topsoil crates across `soil/`, `substrate/`,
`substrate-client/`, and `subsoil` (including subfolders like
`substrate-client/consensus/`). After the `subsoil` merge the dependency graph
contains 3 circular dependency clusters (SCCs), all through dev-dependencies
(there are zero production-only cycles). Many crates are always used together
(high Jaccard similarity) and share the same versioning cadence.

> **Note:** SCC analysis must include `dev-dependencies` — cycles through test crates
> are real compilation-order constraints. Also check subfolders (e.g.
> `substrate-client/consensus/{aura,babe,beefy,grandpa,pow,slots}/`).

### Guiding principles

- **Merge when no new feature flag is needed.** If no consumer ever needs one without the other, they belong together. Unused code is dead-code-eliminated by the linker.
- **Stop merging when transitive deps would grow for existing consumers.** If absorbing a crate would force new heavy third-party deps (e.g. libp2p) on consumers that don't need them, that's a real boundary — use a feature flag or keep it separate.
- **A crate should represent a decision boundary**, not a file boundary. The question is "would a consumer ever want this *without* that?"
- **Consensus engine primitives merge into `subsoil`.** The engines only depend on `codec`/`scale-info`/`serde` (already in subsoil) plus one or two lightweight crates (`finality-grandpa`, `strum`). Unused engines are dead-code-eliminated. Exception: `manual-seal` stays separate because it pulls in `tokio`/`jsonrpsee`/`futures`.

## Circular dependency clusters (SCCs)

These crates are already logically one unit. Each cycle collapses by merging
into a single crate. **All current cycles are through dev-dependencies only** —
there are no production-dep cycles remaining.

> **Note:** SCC analysis excludes topsoil (frame pallet) crates and
> `kitchensink-runtime`. These are _consumers_ of the infrastructure crates —
> they don't participate in infrastructure cycles. Including them inflates
> SCC 1 from 28 to 141 crates because `substrate-test-runtime` depends on
> `soil-service` and topsoil crates depend on `topsoil-support` which
> dev-depends on `substrate-test-runtime-client`. That's a test-infra artifact,
> not a real architectural coupling. Topsoil consolidation is a separate concern.

> **History:** Before the `subsoil` merge there were 5 SCCs. SCC 1 (Primitives,
> 10 crates) was eliminated by merging into `subsoil`. The old SCC 2
> (Client-Executor), SCC 4 (Statement), and SCC 5 (Service-RPC) collapsed into
> one mega-cluster (now SCC 1) because they are connected through shared
> dev-dependency edges (primarily via `substrate-test-runtime-client`). The old
> SCC 3 (Crypto-Hashing) evolved into SCC 2 now that both proc-macros live
> inside `subsoil`. After the consensus-primitives merge into `subsoil`,
> SCC 1 grew from 25 to 28 crates (added `soil-executor-wasmtime`,
> `soil-runtime-test`, `substrate-wasm-builder`).

### SCC 1: Client-Network-Service mega-cluster (28 crates)

All cycles go through dev-dependencies (`substrate-test-runtime-client`,
`substrate-test-runtime`, etc.). The production-dep subgraph is acyclic.

```
Client core:
  soil-client-api        -[dep]-> soil-executor
  soil-executor          -[dev]-> sc-tracing, substrate-test-runtime
  soil-executor-wasmtime -[dep]-> soil-executor
  sc-tracing             -[dep]-> soil-client-api
  soil-client-db         -[dep]-> soil-client-api

Network:
  soil-network            -[dep]-> soil-client-api
  soil-network-sync       -[dep]-> sc-consensus, soil-client-api, soil-network
  soil-network-light      -[dep]-> soil-client-api, soil-network
  soil-network-statement  -[dep]-> soil-network, soil-network-sync
  soil-network-transactions -[dep]-> soil-network, soil-network-sync
  sc-mixnet               -[dep]-> soil-client-api, soil-network

Statement:
  sc-statement-store      -[dep]-> soil-client-api, soil-network-statement
  soil-network-statement  -[dev]-> sc-statement-store

RPC:
  soil-rpc-api     -[dep]-> sc-mixnet, soil-chain-spec
  soil-rpc-server  -[dep]-> soil-rpc-api
  sc-rpc           -[dep]-> sc-block-builder, sc-mixnet, sc-statement-store, sc-tracing,
                             soil-chain-spec, soil-client-api, soil-rpc-api
  soil-rpc-spec-v2 -[dep]-> sc-rpc, soil-chain-spec, soil-client-api

Service:
  soil-service    -[dep]-> sc-consensus, sc-rpc, sc-tracing, sc-transaction-pool,
                           soil-chain-spec, soil-client-api, soil-client-db,
                           soil-executor, soil-informant, soil-network,
                           soil-network-light, soil-network-sync,
                           soil-network-transactions, soil-rpc-server, soil-rpc-spec-v2
  soil-chain-spec -[dep]-> soil-client-api, soil-executor, soil-network
  soil-informant  -[dep]-> soil-client-api, soil-network, soil-network-sync

Consensus/Transaction client wrappers:
  sc-consensus         -[dep]-> soil-client-api
  sc-block-builder     -[dev]-> substrate-test-runtime-client
  sc-transaction-pool  -[dep]-> soil-client-api

Test/build infra (glue that creates most cycles):
  substrate-test-runtime              -[dep]-> soil-service
  substrate-test-runtime              -[dev]-> sc-block-builder, soil-chain-spec,
                                               soil-executor, substrate-test-runtime-client
  substrate-test-runtime-client       -[dep]-> sc-block-builder, sc-consensus,
                                               soil-client-api, substrate-test-client,
                                               substrate-test-runtime
  substrate-test-client               -[dep]-> sc-consensus, soil-client-api,
                                               soil-client-db, soil-executor, soil-service
  substrate-test-runtime-transaction-pool -[dep]-> sc-transaction-pool,
                                                   substrate-test-runtime-client
  soil-runtime-test                   -[dep]-> soil-executor, substrate-test-runtime
  substrate-wasm-builder              -[dev]-> substrate-test-runtime
```

### SCC 2: Subsoil core (3 crates)

```
subsoil              -[dep]-> subsoil-crypto-hashing, subsoil-derive
subsoil-crypto-hashing -[dev]-> subsoil-derive
subsoil-derive       -[dep]-> subsoil-crypto-hashing
subsoil-derive       -[dev]-> subsoil
```

### SCC 3: Node CLI (3 crates)

```
staging-node-cli        -[dev]-> node-testing, substrate-cli-test-utils
node-testing            -[dep]-> staging-node-cli
substrate-cli-test-utils -[dep]-> staging-node-cli
```

## Target structure

### `subsoil` — Primitives foundation (~30 crates → 1) ✅ COMPLETE

Merged. Eliminated the old SCC 1 (Primitives, 10 crates) and absorbed ~30 crates total.

### `subsoil` ← consensus primitives (~12 more crates)

Merged. 8 consensus engine primitive crates absorbed into `subsoil::consensus::*` and
`subsoil::block_builder`. Also merged `soil-mmr-primitives` into `subsoil::mmr` (needed
by beefy, was creating a cyclic dependency). Added `finality-grandpa` and `mmr-lib` as
new deps. Total: 9 crates absorbed.

Deferred: `soil-consensus-epochs` (→ `soil-client`), `soil-consensus-babe-rpc`,
`soil-consensus-grandpa-rpc`, `soil-consensus-beefy-rpc` (→ `soil-rpc`) — std-only crates
with client-side deps.

> **Note:** The current `soil-consensus` crate (client-side traits: `Environment`,
> `Proposer`, `SyncOracle`, `BlockStatus`, `BlockOrigin`) is std-only and has zero
> `no_std` consumers — all 33 dependents are client-side. It will be **renamed to
> `soil-consensus-common`** and ultimately absorbed into `soil-client`.

### `soil-manual-seal` — Kept separate (renamed from `soil-consensus-manual-seal`)

Pulls in `tokio`, `jsonrpsee`, `futures` — heavy async runtime deps that production
nodes shouldn't be forced to compile. Despite having `#![cfg_attr(not(feature = "std"), no_std)]`,
the crate has zero no_std content — every item is behind `#[cfg(feature = "std")]`.
No split needed; just rename and keep as a standalone std-only crate.

### `soil-client` — Client infrastructure (~24 crates → 1)

The client-executor core (part of SCC 1) plus tightly coupled crates and consensus engine wrappers.

| Absorb | Reason |
|---|---|
| soil-consensus-common (née soil-consensus) | Client-side consensus traits (`Proposer`, `SyncOracle`, etc.), std-only, 33 reverse deps — all client-side |
| soil-client-api | 39 reverse deps, SCC 1 with executor |
| soil-executor + common + polkavm + wasmtime | SCC 1 with client-api via sc-tracing |
| soil-blockchain | Depth 8, 34 reverse deps, tight with client-api |
| soil-client-db + soil-state-db | Storage backend |
| soil-transaction-pool + -api | Always together |
| soil-offchain | Always with client |
| soil-genesis-builder | Client-side |
| soil-storage + soil-storage-monitor | Trivial |
| sc-tracing | SCC 1 with client-api/executor |
| sc-block-builder | Wraps soil-block-builder (14 reverse deps) |
| sc-keystore | Wraps soil-keystore (6 reverse deps) |
| sc-consensus | Wraps soil-consensus-common (12 reverse deps) |
| sc-transaction-pool | Wraps soil-transaction-pool (9 reverse deps) |
| sc-consensus-aura | Wraps soil-consensus-aura, wires engine into client |
| sc-consensus-babe | Wraps soil-consensus-babe, wires engine into client |
| sc-consensus-beefy | Wraps soil-consensus-beefy, also needs network |
| sc-consensus-grandpa | Wraps soil-consensus-grandpa, also needs network + chain-spec + client-db |
| sc-consensus-pow | Wraps soil-consensus-pow |
| sc-consensus-slots | Wraps soil-consensus-slots |
| soil-consensus-epochs | Epoch tracking, used by sc-consensus-babe/slots |

### `soil-network` — Networking stack (~10 crates → 1)

| Absorb | Reason |
|---|---|
| soil-network + soil-network-common + soil-network-types | Always together |
| soil-network-sync + soil-network-gossip + soil-network-light | Always with network |
| soil-network-transactions + soil-network-statement + sc-statement-store | SCC 1 pair collapses |
| sc-mixnet | Wraps soil-mixnet |

### `soil-rpc` — RPC layer (~11 crates → 1)

| Absorb | Reason |
|---|---|
| soil-rpc + soil-rpc-api + soil-rpc-server | Always together |
| soil-rpc-spec-v2 | SCC 1 with service, logically RPC |
| soil-mmr-rpc, soil-sync-state-rpc | RPC endpoints |
| sc-rpc | Wraps soil-rpc |
| substrate-frame-rpc-system, substrate-frame-rpc-support | RPC helpers |
| substrate-state-trie-migration-rpc | RPC endpoint |
| substrate-rpc-client | RPC client |
| soil-consensus-babe-rpc | BABE RPC endpoint |
| soil-consensus-grandpa-rpc | GRANDPA RPC endpoint |
| soil-consensus-beefy-rpc | BEEFY RPC endpoint |

### `soil-service` — Node assembly (~10 crates → 1)

| Absorb | Reason |
|---|---|
| soil-service | SCC 1 with rpc-spec-v2 |
| soil-chain-spec + derive | Dep of service |
| soil-cli | Always with service |
| soil-basic-authorship | Block proposer, dep of service |
| soil-informant | Log output, dep of service |
| soil-sysinfo | System metrics |
| soil-telemetry + soil-utils | Infra utilities |
| soil-maybe-compressed-blob | Blob helper |
| soil-proposer-metrics | Metrics |

### Misc standalone (kept separate)

| Crate | Reason |
|---|---|
| soil-mmr-gadget | MMR gadget is opt-in (primitives merged into subsoil) |
| soil-mixnet | Opt-in protocol |
| soil-staking, soil-session | Domain-specific primitives |
| soil-statement-store | Standalone feature |
| fork-tree | Generic data structure |
| substrate-bip39, substrate-prometheus-endpoint | Independent utilities |
| All test/bench crates | Stay separate |

### `soil` — Umbrella re-export

Re-exports everything. Consumers write `soil = { features = ["client", "aura", "grandpa"] }`.

## Summary

| New crate | Absorbs | ~Count | Status |
|---|---|---|---|
| **subsoil** | primitives + consensus engines (slots, aura, babe, grandpa, beefy, pow, sassafras, block-builder, mmr) | ~39 | Phase 1 ✅, Phase 2 ✅ |
| **soil-manual-seal** | renamed from soil-consensus-manual-seal (heavy async deps) | 1 | Pending |
| **soil-client** | consensus-common, client-api, executor, blockchain, db, tx-pool, epochs, sc-* wrappers, sc-consensus-* engines | ~24 | Pending |
| **soil-network** | p2p, sync, gossip, statements | ~10 | Pending |
| **soil-rpc** | rpc server, spec, endpoints, consensus-*-rpc | ~11 | Pending |
| **soil-service** | service, chain-spec, cli, infra | ~10 | Pending |
| **misc standalone** | mmr, mixnet, staking, fork-tree, test crates | ~12 | — |
| **soil** | umbrella re-export | 1 | Pending |

**119 non-topsoil crates → ~14.** All 3 remaining circular dependency clusters
(all dev-dep-only) become crate-internal. SCC 1 (25 crates) spans client,
network, RPC, and service — those merges will collapse it entirely.
