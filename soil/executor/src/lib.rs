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

#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
#[cfg(feature = "std")]
mod executor;
#[cfg(test)]
#[cfg(feature = "std")]
mod integration_tests;
#[cfg(feature = "std")]
mod wasm_runtime;

#[cfg(feature = "std")]
pub use codec::Codec;
#[allow(deprecated)]
#[cfg(feature = "std")]
pub use executor::NativeElseWasmExecutor;
#[cfg(feature = "std")]
pub use executor::{with_externalities_safe, NativeExecutionDispatch, WasmExecutor};
#[doc(hidden)]
#[cfg(feature = "std")]
pub use soil_core::traits::Externalities;
#[cfg(feature = "std")]
pub use soil_version::{NativeVersion, RuntimeVersion};
#[doc(hidden)]
#[cfg(feature = "std")]
pub use soil_wasm_interface;
#[cfg(feature = "std")]
pub use soil_wasm_interface::HostFunctions;
#[cfg(feature = "std")]
pub use wasm_runtime::{read_embedded_version, WasmExecutionMethod};

#[cfg(feature = "std")]
pub use soil_executor_common::{
	error,
	wasm_runtime::{HeapAllocStrategy, DEFAULT_HEAP_ALLOC_PAGES, DEFAULT_HEAP_ALLOC_STRATEGY},
};
#[cfg(feature = "std")]
pub use soil_executor_wasmtime::InstantiationStrategy as WasmtimeInstantiationStrategy;

/// Extracts the runtime version of a given runtime code.
#[cfg(feature = "std")]
pub trait RuntimeVersionOf {
	/// Extract [`RuntimeVersion`] of the given `runtime_code`.
	fn runtime_version(
		&self,
		ext: &mut dyn Externalities,
		runtime_code: &soil_core::traits::RuntimeCode,
	) -> error::Result<RuntimeVersion>;
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
	use super::*;
	use soil_executor_common::runtime_blob::RuntimeBlob;
	use soil_runtime_test::wasm_binary_unwrap;
	use soil_io::TestExternalities;

	#[test]
	fn call_in_interpreted_wasm_works() {
		let mut ext = TestExternalities::default();
		let mut ext = ext.ext();

		let executor = WasmExecutor::<soil_io::SubstrateHostFunctions>::builder().build();
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
