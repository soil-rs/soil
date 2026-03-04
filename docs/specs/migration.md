# Crate Renaming and Migration: Substrate to Soil

## Background

In the original Substrate design, crates are split into three categories based on
`std` support:

| Prefix | Directory     | Description                          | `no_std` |
|--------|---------------|--------------------------------------|----------|
| `sp-`  | `primitives/` | Primitive types and logic            | Yes      |
| —      | `frame/`      | Pallet / FRAME framework             | Yes      |
| `sc-`  | `client/`     | Node (client) code                   | No       |

The intended dependency direction is:

```
sc-* ← frame ← sp-*
```

The `sp-` / `sc-` split existed because native and WASM compilation shared a
single feature-resolution pass — if `std`-only client code leaked into the WASM
pipeline, the build would fail. This constraint **no longer applies**: the native
and WASM pipelines are fully separate and perform independent feature resolution.

### Problems with the current design

1. **Doubled crate count.** Many logical units are artificially split into an
   `sp-` half and an `sc-` half solely because of the `std` feature boundary.
2. **Broken logical separation.** Crates should represent logical boundaries, but
   the `sp-` / `sc-` split forces a second axis of separation (std support) that
   crosscuts the logical one.

## Migration plan

### `soil-` crates (primitives + client)

| Aspect         | Before                                   | After                              |
|----------------|------------------------------------------|-------------------------------------|
| Prefix         | `sp-*` (primitives), `sc-*` (client)     | `soil-*`                            |
| Directory      | `primitives/`, `client/`                 | `soil/`                             |
| `no_std`       | `sp-` yes, `sc-` no                     | **All** `soil-` crates support `no_std` |
| Umbrella crate | —                                        | `soil` (re-exports all `soil-*`)    |

Rules:

- Every `soil-` crate **must** declare a `std` feature and compile under
  `no_std`. For crates whose functionality is inherently `std`-only (e.g.
  networking, database), the `no_std` build produces an empty library.
- The umbrella crate `soil` lives at `soil/soil` and re-exports all public API.
- Paired `sp-` / `sc-` crates that share a logical boundary are merged into a
  single `soil-` crate.

### `topsoil-` crates (FRAME / pallets)

| Aspect         | Before                        | After                                |
|----------------|-------------------------------|---------------------------------------|
| Prefix         | `pallet-*`, `frame-*`        | `topsoil-*`                           |
| Directory      | `frame/`                      | `topsoil/`                            |
| `no_std`       | Yes                           | Yes                                   |

Rules:

- `topsoil-*` crates may depend on `soil-*` crates.
- `soil-*` crates **must not** depend on `topsoil-*` crates.
- The dependency direction is strictly: `topsoil → soil`, never the reverse.

### Summary of prefix mapping

| Old prefix / pattern       | New prefix    | New directory |
|----------------------------|---------------|---------------|
| `sp-*`                     | `soil-`       | `soil/`       |
| `sc-*`                     | `soil-`       | `soil/`       |
| `pallet-*`                 | `topsoil-`    | `topsoil/`    |
| `frame-*`                  | `topsoil-`    | `topsoil/`    |
| `frame` (umbrella)         | `topsoil`     | `topsoil/`    |

### `no_std` design for `sc-*` → `soil-*` migrations

Every `soil-*` crate must compile under `no_std`. For crates originating from
`sc-*` (inherently std-only), the `no_std` build produces an empty library. The
`#[cfg(feature = "std")]` boundary is placed at the **module declaration level**
in `lib.rs`, not scattered throughout interior code.

#### Merged pairs (`sp-*` + `sc-*` → single `soil-*`)

The primitives half (former `sp-*`) is always available. The client half (former
`sc-*`) is feature-gated per module:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

// Always available (former sp-*)
mod error;
mod traits;
pub use error::*;
pub use traits::*;

// std-only (former sc-*)
#[cfg(feature = "std")]
mod import_queue;
#[cfg(feature = "std")]
pub use import_queue::*;

#[cfg(feature = "std")]
mod verifier;
#[cfg(feature = "std")]
pub use verifier::*;
```

#### Standalone `sc-*` → `soil-*`

Every existing `mod` declaration in `lib.rs` gets `#[cfg(feature = "std")]`.
Top-level items (structs, functions, traits, impls) that lived directly in
`lib.rs` are moved into a new `mod client`, also feature-gated. Other modules
remain as top-level siblings — they are **not** nested under `client`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
mod some_mod;
#[cfg(feature = "std")]
pub use some_mod::*;

#[cfg(feature = "std")]
mod another_mod;
#[cfg(feature = "std")]
pub use another_mod::*;

// Top-level items from old lib.rs moved here
#[cfg(feature = "std")]
mod client;
#[cfg(feature = "std")]
pub use client::*;
```

Under `no_std`, the crate compiles as an empty library.

### Dependency invariant

```
topsoil-* ──depends-on──► soil-*
soil-*    ──depends-on──► soil-*
topsoil-* ──depends-on──► topsoil-*

soil-*  ──NEVER──► topsoil-*
```
