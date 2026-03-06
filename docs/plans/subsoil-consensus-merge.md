# Plan: Merge consensus primitives into `subsoil`

## Context

Continuing the crate consolidation (119 non-topsoil crates → ~14). The `subsoil` Phase 1 merge is complete (~30 crates absorbed). Now we merge 12 consensus primitive crates into `subsoil` (Phase 2), per `docs/specs/merge.md` section "subsoil ← consensus primitives (~12 more crates)".

**Note:** `soil-consensus` (the current crate with `Environment`, `Proposer`, `SyncOracle`) is NOT part of this merge — it's std-only client code that will become `soil-consensus-common` and merge into `soil-client` later.

## Crates to merge (8 total)

Consensus engine crates go under `subsoil::consensus::*`. Block builder stays at top level as `subsoil::block_builder`.

| # | Crate | Module path | Source | New deps for subsoil |
|---|---|---|---|---|
| 1 | soil-consensus-slots | `consensus::slots` | `crates/soil/consensus-slots/` | — (codec, scale-info, serde already in subsoil) |
| 2 | soil-block-builder | `block_builder` | `crates/soil/block-builder/` | — |
| 3 | soil-consensus-pow | `consensus::pow` | `crates/soil/consensus-pow/` | — |
| 4 | soil-consensus-grandpa | `consensus::grandpa` | `crates/soil/consensus-grandpa/` | `finality-grandpa` (new) |
| 5 | soil-consensus-beefy | `consensus::beefy` | `crates/soil/consensus-beefy/` | `soil-mmr-primitives` (new) |
| 6 | soil-consensus-aura | `consensus::aura` | `crates/soil/consensus-aura/` | — (uses slots internally) |
| 7 | soil-consensus-babe | `consensus::babe` | `crates/soil/consensus-babe/` | — (uses slots internally) |
| 8 | soil-consensus-sassafras | `consensus::sassafras` | `crates/soil/consensus-sassafras/` | — (uses slots, needs `bandersnatch-experimental`) |

**File structure:**
- `crates/subsoil/src/consensus/mod.rs` declares consensus submodules (`slots`, `pow`, `grandpa`, `beefy`, `aura`, `babe`, `sassafras`). `lib.rs` adds `pub mod consensus;`.
- `crates/subsoil/src/block_builder/` stays at top level. `lib.rs` adds `pub mod block_builder;`.

**Deferred:** `soil-consensus-epochs`, `soil-consensus-babe-rpc`, `soil-consensus-grandpa-rpc`, `soil-consensus-beefy-rpc` — these are std-only crates with client-side deps. Their destination (subsoil vs soil-client) will be decided later.

## Merge order and rationale

Leaf crates first, then crates that depend on slots.

1. `soil-consensus-slots` — leaf, no consensus deps. Merged first because aura/babe/sassafras depend on it. Creates the `consensus/` module dir.
2. `soil-block-builder` — leaf, only depends on subsoil. 2 source files (lib.rs + client_side.rs).
3. `soil-consensus-pow` — leaf, only adds codec (already in subsoil).
4. `soil-consensus-grandpa` — leaf, adds `finality-grandpa` dep. Single file.
5. `soil-consensus-beefy` — leaf, adds `soil-mmr-primitives`. 6 source files + test-res/.
6. `soil-consensus-aura` — depends on slots (now `super::slots`). 3 source files.
7. `soil-consensus-babe` — depends on slots. 3 source files.
8. `soil-consensus-sassafras` — depends on slots. 4 source files. Gated behind `bandersnatch-experimental`.

## Per-commit procedure (template)

For each crate:

### 1. Copy source files
- Consensus crates → `crates/subsoil/src/consensus/$MOD.rs` (or `consensus/$MOD/mod.rs` if multi-file)
- Block builder → `crates/subsoil/src/block_builder/mod.rs` (top level, multi-file)
- Copy any test resources (e.g. beefy's `test-res/`)

### 2. Add module declaration
- Consensus: add `pub mod $MOD;` to `consensus/mod.rs`
- Block builder: add `pub mod block_builder;` to `lib.rs`
- First consensus commit creates `consensus/mod.rs` and adds `pub mod consensus;` to `lib.rs`

### 3. Update `crates/subsoil/Cargo.toml`
- Add any new dependencies (with appropriate feature gating)
- Add `/std` feature propagation for new deps

### 4. Rewrite internal imports in the moved source
- `use subsoil::` → `use crate::` (now inside subsoil)
- Cross-references to sibling modules: `use soil_consensus_slots::` → `use super::slots::` (or `use crate::consensus::slots::`)

### 5. Update workspace `Cargo.toml`
- Remove from `[workspace] members`
- Remove from `[workspace.dependencies]`

### 6. Update all consumer crates
- **Cargo.toml**: Remove old crate dep; ensure `subsoil` dep exists
- **Source files**: e.g. `use soil_consensus_slots::` → `use subsoil::consensus::slots::`, `use soil_block_builder::` → `use subsoil::block_builder::`
- **Feature flags**: e.g. `soil-consensus-babe = { features = ["serde"] }` → `subsoil = { features = ["serde"] }`

### 7. Delete old crate directory

### 8. Verify and commit
- `cargo check -p subsoil` then `cargo check --workspace`
- `git commit`

## Key consumers (need updates across multiple merges)

| Consumer | Crates it depends on |
|---|---|
| `kitchensink-runtime` | block-builder, aura, babe, beefy, grandpa, pow, slots |
| `substrate-test-runtime` | block-builder, aura, babe, grandpa |
| `staging-node-cli` | babe, beefy, grandpa |
| `node-rpc` | block-builder, babe, beefy |
| `sc-consensus-babe` | babe, slots, block-builder |
| `sc-consensus-aura` | aura, slots, block-builder |
| `sc-consensus-grandpa` | grandpa |
| `sc-consensus-beefy` | beefy |
| `sc-consensus-pow` | pow, block-builder |
| `sc-consensus-slots` | slots |
| `sc-block-builder` | block-builder |
| `soil-consensus-manual-seal` | aura, babe, slots |
| `topsoil-*` (aura, babe, grandpa, beefy, sassafras, beefy-mmr, topsoil) | various |
| `soil-mmr-gadget` | beefy |
| `soil-chain-spec` | babe (dev-dep) |

## Critical files

- `Cargo.toml` (workspace root) — members + deps (every commit)
- `crates/subsoil/Cargo.toml` — accumulates new deps
- `crates/subsoil/src/lib.rs` — adds `pub mod` declarations
- ~30 consumer `Cargo.toml` files
- ~87 consumer `.rs` files for import rewrites

## Progress

- [ ] Commit 1: Merge `soil-consensus-slots`
- [ ] Commit 2: Merge `soil-block-builder`
- [ ] Commit 3: Merge `soil-consensus-pow`
- [ ] Commit 4: Merge `soil-consensus-grandpa`
- [ ] Commit 5: Merge `soil-consensus-beefy`
- [ ] Commit 6: Merge `soil-consensus-aura`
- [ ] Commit 7: Merge `soil-consensus-babe`
- [ ] Commit 8: Merge `soil-consensus-sassafras`
- [ ] Update `docs/specs/merge.md` — mark Phase 2a complete

## Verification

After each commit:
1. `cargo check -p subsoil` — merged crate compiles
2. `cargo check --workspace` — all consumers compile

After all merges:
3. `cargo test -p subsoil` — subsoil tests pass
4. Verify `docs/specs/merge.md` status updated
