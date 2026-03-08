// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[cfg(all(feature = "std", not(feature = "metadata-hash")))]
fn main() {
	substrate_wasm_builder::WasmBuilder::build_using_defaults()
}

#[cfg(all(feature = "std", feature = "metadata-hash"))]
fn main() {
	substrate_wasm_builder::WasmBuilder::init_with_defaults()
		.enable_metadata_hash("Test", 14)
		.build()
}

#[cfg(not(feature = "std"))]
fn main() {}
