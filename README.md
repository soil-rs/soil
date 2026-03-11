# Soil

Soil is a blockchain framework for building application-specific blockchains (sometimes called "solo chains" or "app chains"). It is a fork of [Substrate](https://github.com/parity-tech/polkadot-sdk/tree/master/substrate), the framework originally developed by Parity Technologies and used as the foundation of the Polkadot ecosystem.

Substrate is a powerful and battle-tested framework, but its tight coupling to Polkadot has introduced complexity that is unnecessary for projects operating outside that ecosystem. Soil aims to provide a cleaner, more focused alternative for teams that want Substrate's proven architecture without the Polkadot-specific overhead.

## Design Goals

- **Lightweight and stable core.** Strip away layers of abstraction and indirection that accumulated over years of Polkadot-driven development, resulting in a smaller and more approachable codebase.

- **Swappable components.** Make it possible to replace components that were historically fixed in Substrate — in particular, the executor (beyond Wasm or PVM) and the trie storage backend.

- **Reduced technical debt.** Redesign crate boundaries to be clearer and more consistent, eliminating circular dependencies and other structural issues inherited from upstream.

- **Independent of Polkadot.** Clearly separate genuinely generic blockchain primitives from Polkadot-specific functionality, so that solo chain builders only pay for what they use.

## Codebase Organization

The workspace is organized into five top-level directories:

### `main/`

Core crates that make up the framework itself. These are the crates most users will depend on directly.

Crate names follow a layered naming convention:

| Prefix | Role | Substrate equivalent |
|---|---|---|
| `subsoil` | Low-level primitives, types, and traits | `sp-*` |
| `topsoil` | Runtime framework (FRAME) | `frame-*` |
| `soil` | Client-side / node services | `sc-*` |

Key crates include `subsoil` (core primitives), `topsoil` and `topsoil-core` (the runtime framework for building pallets), `topsoil-executive` (block execution logic), and client-side services such as `soil-network`, `soil-consensus`, `soil-rpc`, and `soil-service`.

### `runtime/`

Pallets that ship with the framework. These use the `plant-*` naming convention (equivalent to `pallet-*` in Substrate) and cover common blockchain functionality: account balances, staking, consensus integration (Aura, BABE, GRANDPA), session management, transaction payments, assets, and more.

### `contrib/`

Community-contributed and optional pallets that are not part of the core framework but are maintained within the repository. This includes governance pallets (democracy, referenda, treasury), NFTs, nomination pools, identity, and various example pallets that demonstrate how to build with Soil.

### `harness/`

Test utilities and test-only crates. These provide mock runtimes, test nodes, and support crates used by the framework's test suite. They are not intended for use in production.

### `library/`

Standalone tools and utilities, including `subkey` (key management), `substrate-wasm-builder` (Wasm compilation helper), and RPC support crates.

## License

Soil is licensed under [GPL-3.0-or-later with the Classpath Exception 2.0](LICENSE).
