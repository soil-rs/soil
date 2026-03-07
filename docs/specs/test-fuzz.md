# Merge `core/fuzz` and `npos-elections/fuzzer` into `soil-test-fuzz`

## Summary
- Replace `crates/soil/core/fuzz` and `crates/soil/npos-elections/fuzzer` with one new package at `crates/soil-test-fuzz`.
- Convert the `sp-core-fuzz` target from `cargo-fuzz`/`libfuzzer-sys` to `honggfuzz`.
- Keep the crate as a normal workspace member; do not change `default-members` or create a separate fuzz workspace.
- Normalize all runnable target names to domain-prefixed underscore names:
  - `fuzz_address_uri` -> `core_address_uri`
  - `phragmen_balancing` -> `npos_phragmen_balancing`
  - `phragmms_balancing` -> `npos_phragmms_balancing`
  - `phragmen_pjr` -> `npos_phragmen_pjr`
  - `reduce` -> `npos_reduce`

## Implementation Changes
- Create `crates/soil-test-fuzz/Cargo.toml` as the only fuzz package under `crates/`, with:
  - package name `soil-test-fuzz`
  - version `0.1.0` to match the `soil-test` namespace convention
  - `publish = false`, workspace authors/homepage/repository/license/edition, and workspace lints
  - `honggfuzz`, `subsoil`, `regex`, `rand`, and `clap` as dependencies
  - no `libfuzzer-sys`
  - no `cargo-fuzz` metadata
  - no library target; only explicit `[[bin]]` entries

- Update the workspace manifest in `Cargo.toml`:
  - remove members `crates/soil/core/fuzz` and `crates/soil/npos-elections/fuzzer`
  - add member `crates/soil-test-fuzz`
  - remove workspace dependency `libfuzzer-sys`, since it is no longer used anywhere
  - keep `honggfuzz` unchanged
  - do not add `default-members`

- Use a flat source layout in `crates/soil-test-fuzz/src/` to minimize churn:
  - `src/core_address_uri.rs`
  - `src/npos_phragmen_balancing.rs`
  - `src/npos_phragmms_balancing.rs`
  - `src/npos_phragmen_pjr.rs`
  - `src/npos_reduce.rs`
  - `src/npos_common.rs`
  - `.gitignore` with `hfuzz_target` and `hfuzz_workspace`

- Port the former `sp-core-fuzz` target to honggfuzz by rewriting the harness behavior, not the assertions:
  - switch from `fuzz_target!(|input: &str| ...)` to `honggfuzz::fuzz!(|data: &[u8]| ...)`
  - convert input with `std::str::from_utf8(data)` and skip invalid UTF-8
  - keep the existing regex-vs-`AddressUri::parse` assertions unchanged for valid UTF-8 input
  - do not introduce lossy UTF-8 conversion

- Preserve the existing npos fuzz logic with minimal algorithm changes:
  - move `common.rs` to `npos_common.rs`
  - update imports in the npos bins to use the renamed shared module
  - keep `npos_phragmen_pjr`’s current non-fuzzing local-run mode and CLI behavior, only updating names/docs
  - keep the other npos bins as plain honggfuzz loop-based binaries

- Add a short `crates/soil-test-fuzz/README.md` for developer experience:
  - list the five target names
  - show canonical commands with `cargo hfuzz run <target>`
  - note that `npos_phragmen_pjr` also supports plain `cargo run -p soil-test-fuzz --bin npos_phragmen_pjr -- ...`
  - include a brief old-name -> new-name mapping so existing users can translate commands

- Remove the old directories:
  - `crates/soil/core/fuzz`
  - `crates/soil/npos-elections/fuzzer`
  - refresh `Cargo.lock` so the old package names disappear and `soil-test-fuzz` is recorded instead

## Public Interface Changes
- Package names removed:
  - `sp-core-fuzz`
  - `sp-npos-elections-fuzzer`
- Package name added:
  - `soil-test-fuzz`
- Bin names change exactly to:
  - `core_address_uri`
  - `npos_phragmen_balancing`
  - `npos_phragmms_balancing`
  - `npos_phragmen_pjr`
  - `npos_reduce`
- No compatibility aliases or duplicate legacy bin names should be kept; all inline run/debug comments must be updated to the new names.

## Test Plan
- Structural checks:
  - `cargo metadata --no-deps`
  - `cargo check -p soil-test-fuzz --bins`
- Command-surface checks:
  - `cargo run -p soil-test-fuzz --bin npos_phragmen_pjr -- --help`
  - verify the README commands and inline source comments use the new package/bin names only
- Fuzz-run smoke checks:
  - `cargo hfuzz run core_address_uri`
  - `cargo hfuzz run npos_phragmen_balancing`
  - stop after a short smoke run; this is to verify harness startup and runner wiring, not to perform long fuzzing
- Cleanup checks:
  - confirm no remaining repo references to `sp-core-fuzz`, `sp-npos-elections-fuzzer`, `cargo-fuzz`, `libfuzzer-sys`, `crates/soil/core/fuzz`, or `crates/soil/npos-elections/fuzzer`

## Assumptions
- Scope is limited to the two fuzz crates under `crates/`; `contrib/` fuzzers remain untouched.
- `soil-test-fuzz` should follow the `soil-test-*` namespace convention, even though it is fuzz infrastructure rather than integration tests.
- The workspace behavior stays as-is; this refactor reduces crate count but does not change workspace selection semantics.
- Normalized target names are intentional command-breaking changes in exchange for a cleaner long-term namespace.
