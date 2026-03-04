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

//! Substrate client runtime utilities.
//!
//! Provides convenient APIs to ease calling functions contained by a FRAME
//! runtime WASM blob.
#![warn(missing_docs)]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use codec::{Decode, Encode};
#[cfg(feature = "std")]
use error::{Error, Result};
#[cfg(feature = "std")]
use soil_executor::WasmExecutor;
#[cfg(feature = "std")]
use soil_core::{
	traits::{CallContext, CodeExecutor, FetchRuntimeCode, RuntimeCode},
	OpaqueMetadata,
};
#[cfg(feature = "std")]
use soil_state_machine::BasicExternalities;
#[cfg(feature = "std")]
use soil_wasm_interface::HostFunctions;
#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(feature = "std")]
pub mod error;

/// Fetches the latest metadata from the given runtime blob.
#[cfg(feature = "std")]
pub fn fetch_latest_metadata_from_code_blob<HF: HostFunctions>(
	executor: &WasmExecutor<HF>,
	code_bytes: Cow<[u8]>,
) -> Result<OpaqueMetadata> {
	let runtime_caller = RuntimeCaller::new(executor, code_bytes);
	let version_result = runtime_caller.call("Metadata_metadata_versions", ());

	match version_result {
		Ok(supported_versions) => {
			let supported_versions = Vec::<u32>::decode(&mut supported_versions.as_slice())?;
			let latest_stable = supported_versions
				.into_iter()
				// TODO: Subxt doesn't support V16 metadata until v0.42.0, so don't try
				// to fetch it here until we update to that version.
				.filter(|v| *v != u32::MAX && *v < 16)
				.max()
				.ok_or(Error::StableMetadataVersionNotFound)?;

			let encoded = runtime_caller.call("Metadata_metadata_at_version", latest_stable)?;

			Option::<OpaqueMetadata>::decode(&mut encoded.as_slice())?
				.ok_or(Error::OpaqueMetadataNotFound)
		},
		Err(_) => {
			let encoded = runtime_caller.call("Metadata_metadata", ())?;
			Decode::decode(&mut encoded.as_slice()).map_err(Into::into)
		},
	}
}

#[cfg(feature = "std")]
struct BasicCodeFetcher<'a> {
	code: Cow<'a, [u8]>,
	hash: Vec<u8>,
}

#[cfg(feature = "std")]
impl<'a> FetchRuntimeCode for BasicCodeFetcher<'a> {
	fn fetch_runtime_code(&self) -> Option<Cow<'_, [u8]>> {
		Some(self.code.as_ref().into())
	}
}

#[cfg(feature = "std")]
impl<'a> BasicCodeFetcher<'a> {
	fn new(code: Cow<'a, [u8]>) -> Self {
		Self { hash: soil_crypto_hashing::blake2_256(&code).to_vec(), code }
	}

	fn runtime_code(&'a self) -> RuntimeCode<'a> {
		RuntimeCode {
			code_fetcher: self as &'a dyn FetchRuntimeCode,
			heap_pages: None,
			hash: self.hash.clone(),
		}
	}
}

/// Simple utility that is used to call into the runtime.
#[cfg(feature = "std")]
pub struct RuntimeCaller<'a, 'b, HF: HostFunctions> {
	executor: &'b WasmExecutor<HF>,
	code_fetcher: BasicCodeFetcher<'a>,
}

#[cfg(feature = "std")]
impl<'a, 'b, HF: HostFunctions> RuntimeCaller<'a, 'b, HF> {
	/// Instantiate a new runtime caller.
	pub fn new(executor: &'b WasmExecutor<HF>, code_bytes: Cow<'a, [u8]>) -> Self {
		Self { executor, code_fetcher: BasicCodeFetcher::new(code_bytes) }
	}

	/// Calls a runtime function represented by a `method` name and `parity-scale-codec`
	/// encodable arguments that will be passed to it.
	pub fn call(&self, method: &str, data: impl Encode) -> Result<Vec<u8>> {
		let mut ext = BasicExternalities::default();
		self.executor
			.call(
				&mut ext,
				&self.code_fetcher.runtime_code(),
				method,
				&data.encode(),
				CallContext::Offchain,
			)
			.0
			.map_err(Into::into)
	}
}
