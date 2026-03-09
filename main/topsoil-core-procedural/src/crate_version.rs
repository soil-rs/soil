// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Implementation of macros related to crate versioning.

use super::get_cargo_env_var;
use proc_macro2::{Span, TokenStream};
use syn::{Error, Result};
use topsoil_core_procedural_tools::generate_access_from_frame_or_crate;

/// Create an error that will be shown by rustc at the call site of the macro.
fn create_error(message: &str) -> Error {
	Error::new(Span::call_site(), message)
}

/// Implementation of the `crate_to_crate_version!` macro.
pub fn crate_to_crate_version(input: proc_macro::TokenStream) -> Result<TokenStream> {
	if !input.is_empty() {
		return Err(create_error("No arguments expected!"));
	}

	let major_version = get_cargo_env_var::<u16>("CARGO_PKG_VERSION_MAJOR")
		.map_err(|_| create_error("Major version needs to fit into `u16`"))?;

	let minor_version = get_cargo_env_var::<u8>("CARGO_PKG_VERSION_MINOR")
		.map_err(|_| create_error("Minor version needs to fit into `u8`"))?;

	let patch_version = get_cargo_env_var::<u8>("CARGO_PKG_VERSION_PATCH")
		.map_err(|_| create_error("Patch version needs to fit into `u8`"))?;

	let crate_ = generate_access_from_frame_or_crate("topsoil-core")?;

	Ok(quote::quote! {
		#crate_::traits::CrateVersion {
			major: #major_version,
			minor: #minor_version,
			patch: #patch_version,
		}
	})
}
