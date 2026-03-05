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
- [ ] 9. `soil-storage`

### Tier 2
- [ ] 10. `soil-externalities`
- [ ] 11. `soil-weights`

### Tier 3
- [ ] 12. `soil-core`

### Tier 4
- [ ] 13. `soil-allocator`
- [ ] 14. `soil-keystore`

### Tier 5 — Former SCC
- [ ] 15. `soil-runtime-interface`
- [ ] 16. `soil-trie`
- [ ] 17. `soil-state-machine`
- [ ] 18. `soil-io`
- [ ] 19. `soil-application-crypto`

### Tier 6
- [ ] 20. `soil-runtime`

### Tier 7
- [ ] 21. `soil-version`
- [ ] 22. `soil-api`

### Tier 8
- [ ] 23. `soil-keyring`
- [ ] 24. `soil-crypto-ec-utils`
- [ ] 25. `soil-npos-elections`
- [ ] 26. `soil-inherents`

### Tier 9
- [ ] 27. `soil-timestamp`
