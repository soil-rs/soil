# Crate Merge Plan

## Motivation

The current workspace has 96 non-topsoil crates across `soil/`, `substrate/`,
`substrate-client/`, and `subsoil` (including subfolders like
`substrate-client/consensus/`). After the `subsoil` merge and the `soil-client`
consolidation, the dependency graph contains 3 circular dependency clusters
(SCCs), all through dev-dependencies (there are zero production-only cycles).
Many crates are always used together (high Jaccard similarity) and share the
same versioning cadence.

> **Note:** SCC analysis must include `dev-dependencies` — cycles through test crates
> are real compilation-order constraints. Also check subfolders (e.g.
> `substrate-client/consensus/{aura,babe,beefy,grandpa,pow,slots}/`).

### Guiding principles

- **Merge when no new feature flag is needed.** If no consumer ever needs one without the other, they belong together. Unused code is dead-code-eliminated by the linker.
- **Stop merging when transitive deps would grow for existing consumers.** If absorbing a crate would force new heavy third-party deps (e.g. libp2p) on consumers that don't need them, that's a real boundary — use a feature flag or keep it separate.
- **Prefer fewer crates over feature matrices when the extra deps are light.** Telemetry, fork-tree, or RPC stack deps are acceptable in exchange for simpler testing and a smaller crate graph. Reserve feature gating for genuinely heavy boundaries such as libp2p-class dependencies.
- **A crate should represent a decision boundary**, not a file boundary. The question is "would a consumer ever want this *without* that?"
- **Consensus engine primitives merge into `subsoil`.** The engines only depend on `codec`/`scale-info`/`serde` (already in subsoil) plus one or two lightweight crates (`finality-grandpa`, `strum`). Unused engines are dead-code-eliminated. Exception: `manual-seal` stays separate because it pulls in `tokio`/`jsonrpsee`/`futures`.

## Circular dependency clusters (SCCs)

These crates are already logically one unit. Each cycle collapses by merging
into a single crate. **All current cycles are through dev-dependencies only** —
there are no production-dep cycles remaining.

> **Note:** SCC analysis excludes topsoil (frame pallet) crates and
> `kitchensink-runtime`. These are _consumers_ of the infrastructure crates —
> they don't participate in infrastructure cycles. Including them mostly pulls in
> test-runtime edges and obscures the actual infrastructure boundaries. Topsoil
> consolidation is a separate concern.

> **History:** Before the `subsoil` merge there were 5 SCCs. SCC 1 (Primitives,
> 10 crates) was eliminated by merging into `subsoil`. The old Client-Executor,
> Statement, and Service-RPC SCCs collapsed into one larger dev-only cluster
> through shared test-runtime edges. Since then, the `soil-client` consolidation
> removed the client/executor half of that cluster, and the `soil-network`
> merge collapsed the split network/statement/mixnet crates. The remaining
> large SCC is now a smaller service/RPC/network/consensus test cluster rather
> than a client-network-service mega-cluster.

### SCC 1: Service-RPC-Network-Consensus test cluster (14 crates)

All cycles go through dev-dependencies (`substrate-test-runtime-client`,
`substrate-test-runtime`, etc.). The production-dep subgraph is acyclic.
`soil-client` is no longer part of this SCC, and the old split
network/statement/mixnet crates have been collapsed into `soil-network`.

```
Network / consensus:
  soil-consensus
  soil-network

RPC:
  soil-rpc-api
  soil-rpc-server
  soil-rpc-spec-v2
  sc-rpc

Service / assembly:
  soil-service
  soil-chain-spec
  soil-informant
  sc-transaction-pool

Test infrastructure:
  substrate-test-client
  substrate-test-runtime
  substrate-test-runtime-client
  substrate-test-runtime-transaction-pool
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

### `subsoil` ← consensus primitives and runtime-facing protocols (~13 more crates)

Merged. 8 consensus engine primitive crates absorbed into `subsoil::consensus::*` and
`subsoil::block_builder`. Also merged `soil-mmr-primitives` into `subsoil::mmr` (needed
by beefy, was creating a cyclic dependency). Added `finality-grandpa` and `mmr-lib` as
new deps. `soil-mixnet` also belongs here as a small runtime-facing protocol/types crate:
it is used by runtimes and topsoil pallets, and should not be pulled behind the heavy
`soil-network` boundary. Total: 10 crates absorbed.

Deferred from the primitives merge: client-side consensus support crates and
RPC-facing consensus crates. These stay out of `subsoil` and are handled as part
of the std-only client/service layer below.

### `soil-manual-seal` — Kept separate (renamed from `soil-consensus-manual-seal`) ✅ COMPLETE

Pulls in `tokio`, `jsonrpsee`, `futures` — heavy async runtime deps that production
nodes shouldn't be forced to compile. Despite having `#![cfg_attr(not(feature = "std"), no_std)]`,
the crate has zero no_std content — every item is behind `#[cfg(feature = "std")]`.
No split needed; just rename and keep as a standalone std-only crate.

### `soil-client` — Client infrastructure (~16 crates → 1) ✅ COMPLETE

The client-executor core (part of SCC 1) plus tightly coupled std-only crates.

**Merged (16 crates):**

| Absorbed | Module path |
|---|---|
| former `soil-consensus` (client-side traits) | `soil_client::consensus` |
| soil-client-api | `soil_client::client_api` |
| soil-executor + common + polkavm + wasmtime | `soil_client::executor::{common,polkavm,wasmtime}` |
| soil-blockchain | `soil_client::blockchain` |
| soil-client-db + soil-state-db | `soil_client::db::{state_db}` |
| soil-transaction-pool-api | `soil_client::transaction_pool` |
| soil-storage-monitor | `soil_client::storage_monitor` |
| soil-utils | `soil_client::utils` |
| soil-maybe-compressed-blob | `soil_client::maybe_compressed_blob` |
| sc-tracing | `soil_client::tracing` |
| sc-block-builder | `soil_client::block_builder` |
| sc-keystore | `soil_client::keystore` |

> **Prerequisite:** RPC tracing types (BlockTrace, Event, Span, Data, TraceError,
> TraceBlockResponse) were moved from `soil-rpc::tracing` to `subsoil::tracing::rpc`
> to break the `sc-tracing → soil-rpc → soil-client` cycle.

`soil-client` is intentionally finished here. The remaining consensus-family
crates stay separate so that users can explicitly choose a consensus engine
(`soil-babe`, `soil-grandpa`, etc.) on top of the common client/network/service
stack. They are handled in the next sections, not deferred back into
`soil-client`.

**Not merged (no_std primitives used by WASM runtimes):**

| Crate | Reason |
|---|---|
| soil-offchain-primitives | `#![cfg_attr(not(feature = "std"), no_std)]`, used by topsoil crates |
| soil-genesis-builder | `#![cfg_attr(not(feature = "std"), no_std)]`, used by topsoil crates |
| soil-transaction-pool (primitives) | `#![cfg_attr(not(feature = "std"), no_std)]`, used by topsoil crates |

### `soil-consensus` — Shared consensus support (3 crates → 1) ✅ COMPLETE

Shared std-only support used by multiple consensus engines and by service/network
integration. This reuses the `soil-consensus` name after the former
client-traits crate was absorbed into `soil-client`.

| Absorb | Reason |
|---|---|
| sc-consensus | Shared client-side consensus traits and helpers |
| sc-consensus-slots | Slot machinery used by Aura/BABE |
| soil-consensus-epochs | Shared epoch logic used by BABE and related tooling |

This crate is intentionally slightly "fatter" than the old split. It avoids a
feature matrix and keeps the dependency graph simpler. None of these crates pull
in libp2p-class dependencies.

### Selectable consensus crates (8 crates → 5) ✅ COMPLETE

Users explicitly choose one or more consensus engines on top of
`soil-client`/`soil-network`/`soil-rpc`/`soil-service` (or the umbrella `soil`
crate). These remain separate decision boundaries.

| New crate | Absorbs | Reason |
|---|---|---|
| soil-aura | sc-consensus-aura | Standalone selectable engine |
| soil-babe | sc-consensus-babe + soil-consensus-babe-rpc | Keep engine and its RPC surface together |
| soil-beefy | sc-consensus-beefy + soil-consensus-beefy-rpc | Keep engine and its RPC surface together |
| soil-grandpa | sc-consensus-grandpa + soil-consensus-grandpa-rpc | Keep engine and its RPC surface together |
| soil-pow | sc-consensus-pow | Standalone selectable engine |

The RPC companion crates move with their engine crates rather than into
`soil-rpc`. This matches how users configure nodes: consensus selection is an
explicit choice.

### `soil-network` — Networking stack (~10 crates → 1) ✅ COMPLETE

| Absorb | Reason |
|---|---|
| soil-network + soil-network-common + soil-network-types | Always together |
| soil-network-sync + soil-network-gossip + soil-network-light | Always with network |
| soil-network-transactions + soil-network-statement + sc-statement-store | SCC 1 pair collapses |
| sc-mixnet | Service-side mixnet integration over `soil-network` + `subsoil::mixnet` |

Merged. `soil-network` now owns the p2p/common/types/light/sync/gossip/
transactions/statement/statement_store/mixnet service stack. Runtime-facing
mixnet protocol types remain in `subsoil::mixnet`.

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

### `soil-service` — Node assembly (~10 crates → 1)

| Absorb | Reason |
|---|---|
| soil-service | SCC 1 with rpc-spec-v2 |
| soil-chain-spec + derive | Dep of service |
| soil-cli | Always with service |
| soil-basic-authorship | Block proposer, dep of service |
| soil-informant | Log output, dep of service |
| soil-sysinfo | System metrics |
| soil-telemetry | Infra utility |
| soil-maybe-compressed-blob | Blob helper |
| soil-proposer-metrics | Metrics |

### `soil-txpool` — Transaction pool service layer (1 crate)

| Absorb | Reason |
|---|---|
| sc-transaction-pool | Shared service-side infra used by service, manual seal, and node/CLI crates |

Kept separate from both `soil-consensus` and `soil-service`. It is reusable
service-side infrastructure, not a consensus-engine choice and not just service
assembly glue.

### Misc standalone (kept separate)

| Crate | Reason |
|---|---|
| soil-mmr-gadget | MMR gadget is opt-in (primitives merged into subsoil) |
| soil-staking, soil-session | Domain-specific primitives |
| soil-statement-store | Standalone feature |
| fork-tree | Generic data structure |
| substrate-bip39, substrate-prometheus-endpoint | Independent utilities |
| soil-test and all other test/bench crates | Stay separate |

### `soil` — Umbrella re-export

Re-exports everything. Consumers write `soil = { features = ["client", "aura", "grandpa"] }`.

## Summary

| New crate | Absorbs | ~Count | Status |
|---|---|---|---|
| **subsoil** | primitives + consensus engines (slots, aura, babe, grandpa, beefy, pow, sassafras, block-builder, mmr) + mixnet protocol | ~40 | Phase 1 ✅, Phase 2 ✅ |
| **soil-manual-seal** | renamed from soil-consensus-manual-seal (heavy async deps) | 1 | ✅ |
| **soil-client** | client-api, executor (4), blockchain, db (2), tx-pool-api, storage-monitor, utils, maybe-compressed-blob, sc-tracing, sc-block-builder, sc-keystore | ~16 | ✅ |
| **soil-consensus** | sc-consensus, sc-consensus-slots, soil-consensus-epochs | 3 | ✅ |
| **soil-{aura,babe,beefy,grandpa,pow}** | selectable consensus engines; babe/beefy/grandpa also absorb their RPC crates | 8 → 5 | ✅ |
| **soil-network** | p2p, common/types, light, sync, gossip, transactions, statements, mixnet service | ~10 | ✅ |
| **soil-rpc** | rpc server, spec, endpoints, rpc client/helpers | ~11 | Pending |
| **soil-service** | service, chain-spec, cli, infra | ~9 | Pending |
| **soil-txpool** | sc-transaction-pool | 1 | Pending |
| **misc standalone** | mmr, staking, fork-tree, test crates | ~12 | — |
| **soil** | umbrella re-export | 1 | Pending |

**96 non-topsoil crates → low-teens major crates plus a small set of intentional
standalones.** All 3 remaining circular dependency clusters are still
dev-dep-only. SCC 1 is now down to 14 crates after the `soil-network` merge;
the remaining work is concentrated in `soil-rpc`, `soil-service`, and
`soil-txpool`.
