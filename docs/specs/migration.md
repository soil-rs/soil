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

### Dependency invariant

```
topsoil-* ──depends-on──► soil-*
soil-*    ──depends-on──► soil-*
topsoil-* ──depends-on──► topsoil-*

soil-*  ──NEVER──► topsoil-*
```
