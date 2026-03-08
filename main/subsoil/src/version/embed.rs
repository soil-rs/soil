// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Provides functionality to embed a [`RuntimeVersion`](super::RuntimeVersion) as custom section
//! into a WASM file.

use codec::Encode;
use parity_wasm::elements::{deserialize_buffer, serialize, Module};

#[derive(Clone, Copy, Eq, PartialEq, Debug, thiserror::Error)]
pub enum Error {
	#[error("Deserializing wasm failed")]
	Deserialize,
	#[error("Serializing wasm failed")]
	Serialize,
}

/// Embed the given `version` to the given `wasm` blob.
///
/// If there was already a runtime version embedded, this will be overwritten.
///
/// Returns the new WASM blob.
pub fn embed_runtime_version(
	wasm: &[u8],
	mut version: super::RuntimeVersion,
) -> Result<Vec<u8>, Error> {
	let mut module: Module = deserialize_buffer(wasm).map_err(|_| Error::Deserialize)?;

	let apis = version
		.apis
		.iter()
		.map(Encode::encode)
		.flat_map(|v| v.into_iter())
		.collect::<Vec<u8>>();

	module.set_custom_section("runtime_apis", apis);

	version.apis.to_mut().clear();
	module.set_custom_section("runtime_version", version.encode());

	serialize(module).map_err(|_| Error::Serialize)
}
