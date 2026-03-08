// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! A crate that provides means of executing/dispatching calls into the runtime.
//!
//! There are a few responsibilities of this crate at the moment:
//!
//! - It provides an implementation of a common entrypoint for calling into the runtime, both
//! wasm and compiled.
//! - It defines the environment for the wasm execution, namely the host functions that are to be
//! provided into the wasm runtime module.
//! - It also provides the required infrastructure for executing the current wasm runtime (specified
//! by the current value of `:code` in the provided externalities), i.e. interfacing with
//! wasm engine used, instance cache.

#![warn(missing_docs)]

pub mod common;
pub mod polkavm;
pub mod wasmtime;

#[macro_use]
mod executor;
mod wasm_runtime;

#[allow(deprecated)]
pub use self::executor::NativeElseWasmExecutor;
pub use self::executor::{with_externalities_safe, NativeExecutionDispatch, WasmExecutor};
pub use codec::Codec;
#[doc(hidden)]
pub use subsoil::core::traits::Externalities;
pub use subsoil::version::{NativeVersion, RuntimeVersion};
#[doc(hidden)]
pub use subsoil::wasm_interface;
pub use subsoil::wasm_interface::HostFunctions;
pub use wasm_runtime::{read_embedded_version, WasmExecutionMethod};

pub use self::wasmtime::InstantiationStrategy as WasmtimeInstantiationStrategy;
pub use common::{
	error,
	wasm_runtime::{HeapAllocStrategy, DEFAULT_HEAP_ALLOC_PAGES, DEFAULT_HEAP_ALLOC_STRATEGY},
};

/// Extracts the runtime version of a given runtime code.
pub trait RuntimeVersionOf {
	/// Extract [`RuntimeVersion`] of the given `runtime_code`.
	fn runtime_version(
		&self,
		ext: &mut dyn Externalities,
		runtime_code: &subsoil::core::traits::RuntimeCode,
	) -> error::Result<RuntimeVersion>;
}
