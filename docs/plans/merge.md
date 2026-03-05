# Subsoil Merge Progress

Tracking the merge of ~33 primitives crates into a single `subsoil` crate.
See `docs/specs/merge.md` for the full spec.

## Phase 0: Scaffolding

- [x] Step 0.1: Move & rename `soil-crypto-hashing` → `subsoil-crypto-hashing`
- [x] Step 0.2: Move & rename 6 proc-macro crates → `subsoil-*` prefix
- [x] Step 0.3: Create empty `subsoil` crate

## Phase 1: Merge Crates

### Tier 0 — Leaves
- [x] 1. `soil-std`
- [x] 2. `soil-panic-handler`
- [x] 3. `soil-wasm-interface`
- [x] 4. `soil-metadata-ir`
- [x] 5. `soil-database`
- [x] 6. `soil-tracing`
- [x] 7. `soil-arithmetic`
- [x] 8. `binary-merkle-tree`

### Tier 1
- [x] 9. `soil-storage`

### Tier 2
- [x] 10. `soil-externalities`
- [x] 11. `soil-weights`

### Tier 3
- [x] 12. `soil-core`

### Tier 4
- [x] 13. `soil-allocator`
- [x] 14. `soil-keystore`

### Tier 5 — Former SCC
- [x] 15. `soil-runtime-interface`
- [x] 16. `soil-trie`
- [x] 17. `soil-state-machine`
- [x] 18. `soil-io`
- [x] 19. `soil-application-crypto`

### Tier 6
- [x] 20. `soil-runtime`

### Tier 7
- [x] 21. `soil-version`
- [x] 22. `soil-api`

### Tier 8
- [x] 23. `soil-keyring`
- [x] 24. `soil-crypto-ec-utils`
- [x] 25. `soil-npos-elections`
- [x] 26. `soil-inherents`

### Tier 9
- [x] 27. `soil-timestamp`
