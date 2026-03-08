// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Defines a `WasmRuntime` that uses the Wasmtime JIT to execute.
//!
//! You can choose a profiling strategy at runtime with
//! environment variable `WASMTIME_PROFILING_STRATEGY`:
//!
//! | `WASMTIME_PROFILING_STRATEGY` | Effect |
//! |-------------|-------------------------|
//! | undefined   | No profiling            |
//! | `"jitdump"` | jitdump profiling       |
//! | `"perfmap"` | perfmap profiling       |
//! | other value | No profiling (warning)  |

mod host;
mod imports;
mod instance_wrapper;
mod runtime;
mod util;

#[cfg(test)]
mod tests;

pub use crate::executor::common::{
	runtime_blob::RuntimeBlob,
	wasm_runtime::{HeapAllocStrategy, WasmModule},
};
pub use runtime::{
	create_runtime, create_runtime_from_artifact, create_runtime_from_artifact_bytes,
	prepare_runtime_artifact, Config, DeterministicStackLimit, InstantiationStrategy, Semantics,
	WasmtimeRuntime,
};
