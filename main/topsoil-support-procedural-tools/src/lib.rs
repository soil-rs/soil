// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Proc macro helpers for procedural macros
// end::description[]

// reexport proc macros
pub use topsoil_support_procedural_tools_macros::*;

use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::parse::Error;

pub mod syn_ext;

// FIXME #1569, remove the following functions, which are copied from sp-api-macros
use proc_macro2::{Span, TokenStream};
use syn::Ident;

fn generate_hidden_includes_mod_name(unique_id: &str) -> Ident {
	Ident::new(&format!("sp_api_hidden_includes_{}", unique_id), Span::call_site())
}

/// Generates the access to the `topsoil-support` crate.
pub fn generate_crate_access(unique_id: &str, def_crate: &str) -> TokenStream {
	if std::env::var("CARGO_PKG_NAME").unwrap() == def_crate {
		let topsoil_support = match generate_access_from_frame_or_crate("topsoil-support") {
			Ok(c) => c,
			Err(e) => return e.into_compile_error().into(),
		};
		quote::quote!(#topsoil_support)
	} else {
		let mod_name = generate_hidden_includes_mod_name(unique_id);
		quote::quote!( self::#mod_name::hidden_include )
	}
}

/// Check if a path is using the `frame` crate or not.
///
/// This will usually check the output of [`generate_access_from_frame_or_crate`].
/// We want to know if whatever the `path` takes us to, is exported from `frame` or not. In that
/// case `path` would start with `frame`, something like `topsoil::x::y:z` or
/// topsoil::x::y:z.
pub fn is_using_frame_crate(path: &syn::Path) -> bool {
	path.segments.first().map(|s| s.ident == "topsoil").unwrap_or(false)
}

/// Generate the crate access for the crate using 2018 syntax.
///
/// If `frame` is in scope, it will use `topsoil::deps::<def_crate>`. Else, it will try
/// and find `<def_crate>` directly.
pub fn generate_access_from_frame_or_crate(def_crate: &str) -> Result<syn::Path, Error> {
	if let Some(path) = get_frame_crate_path(def_crate) {
		Ok(path)
	} else if let Some(path) = get_sdk_crate_path(def_crate) {
		Ok(path)
	} else {
		let ident = match crate_name(def_crate) {
			Ok(FoundCrate::Itself) => {
				let name = def_crate.to_string().replace("-", "_");
				Ok(syn::Ident::new(&name, Span::call_site()))
			},
			Ok(FoundCrate::Name(name)) => Ok(Ident::new(&name, Span::call_site())),
			Err(e) => Err(Error::new(Span::call_site(), e)),
		}?;

		Ok(syn::Path::from(ident))
	}
}

/// Generates the hidden includes that are required to make the macro independent from its scope.
pub fn generate_hidden_includes(unique_id: &str, def_crate: &str) -> TokenStream {
	let mod_name = generate_hidden_includes_mod_name(unique_id);

	if let Some(path) = get_frame_crate_path(def_crate) {
		quote::quote!(
			#[doc(hidden)]
			mod #mod_name {
				pub use #path as hidden_include;
			}
		)
	} else if let Some(path) = get_sdk_crate_path(def_crate) {
		quote::quote!(
			#[doc(hidden)]
			mod #mod_name {
				pub use #path as hidden_include;
			}
		)
	} else {
		match crate_name(def_crate) {
			Ok(FoundCrate::Itself) => quote!(),
			Ok(FoundCrate::Name(name)) => {
				let name = Ident::new(&name, Span::call_site());
				quote::quote!(
					#[doc(hidden)]
					mod #mod_name {
						pub use #name as hidden_include;
					}
				)
			},
			Err(e) => {
				let err = Error::new(Span::call_site(), e).to_compile_error();
				quote!( #err )
			},
		}
	}
}

/// Generates the path to the frame crate deps.
fn get_frame_crate_path(def_crate: &str) -> Option<syn::Path> {
	// This does not work if the frame crate is renamed.
	if let Ok(FoundCrate::Name(name)) = crate_name(&"topsoil") {
		let path = format!("{}::deps::{}", name, def_crate.to_string().replace("-", "_"));
		Some(syn::parse_str::<syn::Path>(&path).expect("is a valid path; qed"))
	} else {
		None
	}
}

fn get_sdk_crate_path(def_crate: &str) -> Option<syn::Path> {
	if let Ok(FoundCrate::Name(name)) = crate_name(&"polkadot-sdk") {
		let path = format!("{}::{}", name, def_crate.to_string()).replace("-", "_");
		Some(syn::parse_str::<syn::Path>(&path).expect("is a valid path; qed"))
	} else {
		None
	}
}

// fn to remove white spaces around string types
// (basically whitespaces around tokens)
pub fn clean_type_string(input: &str) -> String {
	input
		.replace(" ::", "::")
		.replace(":: ", "::")
		.replace(" ,", ",")
		.replace(" ;", ";")
		.replace(" [", "[")
		.replace("[ ", "[")
		.replace(" ]", "]")
		.replace(" (", "(")
		.replace("( ", "(")
		.replace(" )", ")")
		.replace(" <", "<")
		.replace("< ", "<")
		.replace(" >", ">")
}

/// Return all doc attributes literals found.
pub fn get_doc_literals(attrs: &[syn::Attribute]) -> Vec<syn::Expr> {
	attrs
		.iter()
		.filter_map(|attr| {
			if let syn::Meta::NameValue(meta) = &attr.meta {
				meta.path
					.get_ident()
					.filter(|ident| *ident == "doc")
					.map(|_| meta.value.clone())
			} else {
				None
			}
		})
		.collect()
}

/// Return all cfg attributes literals found.
pub fn get_cfg_attributes(attrs: &[syn::Attribute]) -> Vec<syn::Attribute> {
	attrs
		.iter()
		.filter_map(|attr| {
			if let syn::Meta::List(meta) = &attr.meta {
				meta.path.get_ident().filter(|ident| *ident == "cfg").map(|_| attr.clone())
			} else {
				None
			}
		})
		.collect()
}
