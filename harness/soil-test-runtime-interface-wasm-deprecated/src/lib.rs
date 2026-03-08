// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Tests for the runtime interface traits and proc macros.

#![cfg_attr(not(feature = "std"), no_std)]

use subsoil::runtime_interface::runtime_interface;
use subsoil::wasm_export_functions;

// Include the WASM binary
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

/// Wasm binary unwrapped. If built with `SKIP_WASM_BUILD`, the function panics.
#[cfg(feature = "std")]
pub fn wasm_binary_unwrap() -> &'static [u8] {
	WASM_BINARY.expect(
		"Development wasm binary is not available. Testing is only supported with the flag \
		 disabled.",
	)
}

/// This function is not used, but we require it for the compiler to include `sp-io`.
/// `sp-io` is required for its panic and oom handler.
#[cfg(not(feature = "std"))]
#[no_mangle]
pub fn import_sp_io() {
	subsoil::io::misc::print_utf8(&[]);
}

#[runtime_interface]
pub trait TestApi {
	fn test_versioning(&self, _data: u32) -> bool {
		// should not be called
		unimplemented!()
	}
}

wasm_export_functions! {
	fn test_versioning_works() {
		// old api allows only 42 and 50
		assert!(test_api::test_versioning(42));
		assert!(test_api::test_versioning(50));

		assert!(!test_api::test_versioning(142));
		assert!(!test_api::test_versioning(0));
	}
}
