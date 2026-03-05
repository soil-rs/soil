# Crate Merge Plan

## Motivation

The current workspace has 152 crates across `soil/`, `substrate/`, and `substrate-client/`
(including subfolders like `substrate-client/consensus/`).
The dependency graph contains 5 circular dependency clusters (SCCs) and 17 topological layers.
Many crates are always used together (high Jaccard similarity) and share the same versioning cadence.

> **Note:** SCC analysis must include `dev-dependencies` — cycles through test crates
> are real compilation-order constraints. Also check subfolders (e.g.
> `substrate-client/consensus/{aura,babe,beefy,grandpa,pow,slots}/`).

### Guiding principles

- **Merge when no new feature flag is needed.** If no consumer ever needs one without the other, they belong together. Unused code is dead-code-eliminated by the linker.
- **Stop merging when transitive deps would grow for existing consumers.** If absorbing a crate would force new heavy third-party deps (e.g. libp2p) on consumers that don't need them, that's a real boundary — use a feature flag or keep it separate.
- **A crate should represent a decision boundary**, not a file boundary. The question is "would a consumer ever want this *without* that?"
- **Consensus engines merge into the base crate.** The engines only depend on `codec`/`scale-info`/`serde` (already universal) plus one or two lightweight crates (`finality-grandpa`, `strum`). Unused engines are dead-code-eliminated. Exception: `manual-seal` stays separate because it pulls in `tokio`/`jsonrpsee`/`futures`.

## Circular dependency clusters (SCCs)

These crates are already logically one unit. Each cycle collapses by merging into a single crate.

### SCC 1: Primitives (10 crates)

```
soil-api ↔ soil-runtime ↔ soil-io ↔ soil-runtime-interface ↔ soil-state-machine ↔ soil-trie
soil-version ↔ soil-version-proc-macro
soil-application-crypto → soil-io
binary-merkle-tree → soil-runtime
```

### SCC 2: Client-Executor (3 crates)

```
sc-tracing → soil-client-api → soil-executor → sc-tracing
```

### SCC 3: Crypto-Hashing (2 crates)

```
soil-crypto-hashing ↔ soil-crypto-hashing-proc-macro
```

### SCC 4: Statement (2 crates)

```
sc-statement-store ↔ soil-network-statement
```

### SCC 5: Service-RPC (2 crates)

```
soil-rpc-spec-v2 ↔ soil-service
```

## Target structure

### `subsoil` — Primitives foundation (~30 crates → 1)

The SCC:primitives nucleus plus everything it depends on at depth ≤ 7.

| Absorb | Current depth | Reason |
|---|---|---|
| soil-core | 3 | Heart of primitives |
| soil-io | 5 (SCC) | Cyclic with runtime/trie |
| soil-runtime | 5 (SCC) | Cyclic with io/state-machine |
| soil-state-machine | 5 (SCC) | Cyclic with runtime/trie |
| soil-trie | 5 (SCC) | Cyclic with runtime |
| soil-api + proc-macro | 5 (SCC) | Cyclic with runtime |
| soil-version + proc-macro | 5 (SCC) | Cyclic with runtime |
| soil-runtime-interface + proc-macro | 5 (SCC) | Cyclic with io |
| soil-application-crypto | 5 (SCC) | Cyclic with io |
| binary-merkle-tree | 5 (SCC) | Cyclic with runtime |
| soil-crypto-hashing + proc-macro | 0 (SCC) | Cyclic pair, dep of core |
| soil-std | 0 | Leaf, dep of core |
| soil-debug-derive | 0 | Leaf, dep of core |
| soil-tracing + proc-macro | 0 | Leaf, 21 reverse deps |
| soil-panic-handler | 0 | Leaf, dep of state-machine |
| soil-wasm-interface | 0 | Leaf, dep of allocator/executor |
| soil-storage | 1 | Dep of core/externalities |
| soil-externalities | 2 | Dep of core/io/state-machine |
| soil-arithmetic | 1 | Dep of runtime, used broadly |
| soil-weights | 2 | Dep of runtime |
| soil-metadata-ir | 0 | Dep of api |
| soil-allocator | 4 | Tiny, wasm-only |
| soil-inherents | 6 | Tiny trait, dep of consensus/timestamp |
| soil-timestamp | 7 | Tiny, just inherents+runtime |
| soil-keystore | 4 | Dep of io, 19 reverse deps |
| soil-database | 0 | Trait-only, 4 reverse deps |
| soil-keyring | 6 | Test/dev identities, only deps are core+runtime |
| soil-crypto-ec-utils | 6 | EC helpers, only dep is runtime-interface |
| soil-npos-elections | 6 | Election math, only deps are arithmetic+core+runtime |

### `soil-consensus` — Consensus framework + engines (~11 crates → 1)

Base consensus traits, shared helpers, and all engine implementations. The engines only add
lightweight external deps (`codec`, `scale-info`, `serde`, `finality-grandpa`, `strum`) that
consumers already have or that the linker strips if unused.

| Absorb | Reason |
|---|---|
| soil-consensus | Base traits (depth 7, 25 reverse deps) |
| soil-consensus-slots | Slot-based helper (depth 8, 7 reverse deps) |
| soil-consensus-epochs | Epoch tracking (depth 12, deps are blockchain+client-api+runtime) |
| soil-block-builder | Block building (depth 7, 6 reverse deps) |
| soil-consensus-aura | Engine, only adds `codec`/`scale-info` |
| soil-consensus-babe + `-rpc` | Engine, only adds `codec`/`scale-info`/`serde` |
| soil-consensus-grandpa + `-rpc` | Engine, adds `finality-grandpa` (lightweight) |
| soil-consensus-beefy + `-rpc` | Engine, adds `strum` (trivial) |
| soil-consensus-sassafras | Engine, only adds `codec`/`scale-info`/`serde` |
| soil-consensus-pow | Engine, only adds `codec` |

### `soil-consensus-manual-seal` — Kept separate

Pulls in `tokio`, `jsonrpsee`, `futures` — heavy async runtime deps that production
nodes shouldn't be forced to compile.

### `soil-client` — Client infrastructure (~22 crates → 1)

The SCC:client-executor cluster plus tightly coupled crates and consensus engine wrappers.

| Absorb | Reason |
|---|---|
| soil-client-api | 39 reverse deps, SCC with executor |
| soil-executor + common + polkavm + wasmtime | SCC with client-api via sc-tracing |
| soil-blockchain | Depth 8, 34 reverse deps, tight with client-api |
| soil-client-db + soil-state-db | Storage backend |
| soil-transaction-pool + -api | Always together |
| soil-offchain | Always with client |
| soil-genesis-builder | Client-side |
| soil-storage + soil-storage-monitor | Trivial |
| sc-tracing | In SCC with client-api/executor |
| sc-block-builder | Wraps soil-block-builder (14 reverse deps) |
| sc-keystore | Wraps soil-keystore (6 reverse deps) |
| sc-consensus | Wraps soil-consensus (12 reverse deps) |
| sc-transaction-pool | Wraps soil-transaction-pool (9 reverse deps) |
| sc-consensus-aura | Wraps soil-consensus-aura, wires engine into client |
| sc-consensus-babe | Wraps soil-consensus-babe, wires engine into client |
| sc-consensus-beefy | Wraps soil-consensus-beefy, also needs network |
| sc-consensus-grandpa | Wraps soil-consensus-grandpa, also needs network + chain-spec + client-db |
| sc-consensus-pow | Wraps soil-consensus-pow |
| sc-consensus-slots | Wraps soil-consensus-slots |

### `soil-network` — Networking stack (~10 crates → 1)

| Absorb | Reason |
|---|---|
| soil-network + soil-network-common + soil-network-types | Always together |
| soil-network-sync + soil-network-gossip + soil-network-light | Always with network |
| soil-network-transactions + soil-network-statement + sc-statement-store | SCC pair collapses |
| sc-mixnet | Wraps soil-mixnet |

### `soil-rpc` — RPC layer (~8 crates → 1)

| Absorb | Reason |
|---|---|
| soil-rpc + soil-rpc-api + soil-rpc-server | Always together |
| soil-rpc-spec-v2 | SCC with service, logically RPC |
| soil-mmr-rpc, soil-sync-state-rpc | RPC endpoints |
| sc-rpc | Wraps soil-rpc |
| substrate-frame-rpc-system, substrate-frame-rpc-support | RPC helpers |
| substrate-state-trie-migration-rpc | RPC endpoint |
| substrate-rpc-client | RPC client |

### `soil-service` — Node assembly (~10 crates → 1)

| Absorb | Reason |
|---|---|
| soil-service | SCC with rpc-spec-v2 |
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
| soil-mmr-primitives + soil-mmr-gadget | MMR is opt-in |
| soil-mixnet | Opt-in protocol |
| soil-staking, soil-session | Domain-specific primitives |
| soil-statement-store | Standalone feature |
| fork-tree | Generic data structure |
| substrate-bip39, substrate-prometheus-endpoint | Independent utilities |
| All test/bench crates | Stay separate |

### `soil` — Umbrella re-export

Re-exports everything. Consumers write `soil = { features = ["client", "aura", "grandpa"] }`.

## Summary

| New crate | Absorbs | ~Count |
|---|---|---|
| **subsoil** | primitives, types, runtime, state-machine, trie, api, crypto utils | ~33 |
| **soil-consensus** | base consensus + slots + epochs + block-builder + all engines | ~11 |
| **soil-consensus-manual-seal** | kept separate (heavy async deps) | 1 |
| **soil-client** | client-api, executor, blockchain, db, tx-pool, sc-* wrappers, sc-consensus-* engines | ~22 |
| **soil-network** | p2p, sync, gossip, statements | ~10 |
| **soil-rpc** | rpc server, spec, endpoints | ~8 |
| **soil-service** | service, chain-spec, cli, infra | ~10 |
| **misc standalone** | mmr, mixnet, staking, fork-tree, test crates | ~12 |
| **soil** | umbrella re-export | 1 |

**152 crates → ~15.** All 5 circular dependency clusters become crate-internal.
