// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! A set of common definitions that are needed for defining execution engines.

pub mod error;
pub mod runtime_blob;
pub mod util;
pub mod wasm_runtime;

pub(crate) fn is_polkavm_enabled() -> bool {
	std::env::var_os("SUBSTRATE_ENABLE_POLKAVM").map_or(false, |value| value == "1")
}
