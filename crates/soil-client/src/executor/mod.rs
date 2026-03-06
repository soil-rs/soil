// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

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

#[cfg(test)]
mod tests {
	use super::*;
	use common::runtime_blob::RuntimeBlob;
	use soil_runtime_test::wasm_binary_unwrap;
	use subsoil::io::TestExternalities;

	#[test]
	fn call_in_interpreted_wasm_works() {
		let mut ext = TestExternalities::default();
		let mut ext = ext.ext();

		let executor = WasmExecutor::<subsoil::io::SubstrateHostFunctions>::builder().build();
		let res = executor
			.uncached_call(
				RuntimeBlob::uncompress_if_needed(wasm_binary_unwrap()).unwrap(),
				&mut ext,
				true,
				"test_empty_return",
				&[],
			)
			.unwrap();
		assert_eq!(res, vec![0u8; 0]);
	}
}
