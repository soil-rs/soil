// This file is part of Soil.

// Copyright (C) Soil contributors.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use syn::{
	parse::{Parse, ParseStream},
	parse_macro_input, ItemTrait, Result, Token,
};

use utils::generate_runtime_interface_include;

use proc_macro2::{Span, TokenStream as TokenStream2};

use inflector::Inflector;

use quote::quote;

use syn::Ident;

mod bare_function_interface;
mod host_function_interface;
mod trait_decl_impl;
mod utils;

/// Custom keywords supported by the `runtime_interface` attribute.
pub(crate) mod keywords {
	syn::custom_keyword!(wasm_only);
	syn::custom_keyword!(no_tracing);
}

struct Options {
	wasm_only: bool,
	tracing: bool,
}

impl Options {
	fn unpack(self) -> (bool, bool) {
		(self.wasm_only, self.tracing)
	}
}
impl Default for Options {
	fn default() -> Self {
		Options { wasm_only: false, tracing: true }
	}
}

impl Parse for Options {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut res = Self::default();
		while !input.is_empty() {
			let lookahead = input.lookahead1();
			if lookahead.peek(keywords::wasm_only) {
				let _ = input.parse::<keywords::wasm_only>();
				res.wasm_only = true;
			} else if lookahead.peek(keywords::no_tracing) {
				let _ = input.parse::<keywords::no_tracing>();
				res.tracing = false;
			} else if lookahead.peek(Token![,]) {
				let _ = input.parse::<Token![,]>();
			} else {
				return Err(lookahead.error());
			}
		}
		Ok(res)
	}
}

pub(crate) fn runtime_interface(
	attrs: proc_macro::TokenStream,
	input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let trait_def = parse_macro_input!(input as ItemTrait);
	let (wasm_only, tracing) = parse_macro_input!(attrs as Options).unpack();

	runtime_interface_impl(trait_def, wasm_only, tracing)
		.unwrap_or_else(|e| e.to_compile_error())
		.into()
}

/// Implementation of the `runtime_interface` attribute.
fn runtime_interface_impl(
	trait_def: ItemTrait,
	is_wasm_only: bool,
	tracing: bool,
) -> Result<TokenStream2> {
	let bare_functions = bare_function_interface::generate(&trait_def, is_wasm_only, tracing)?;
	let crate_include = generate_runtime_interface_include();
	let mod_name = Ident::new(&trait_def.ident.to_string().to_snake_case(), Span::call_site());
	let trait_decl_impl = trait_decl_impl::process(&trait_def, is_wasm_only)?;
	let host_functions = host_function_interface::generate(&trait_def, is_wasm_only)?;
	let vis = trait_def.vis;
	let attrs = &trait_def.attrs;

	let res = quote! {
		#( #attrs )*
		#vis mod #mod_name {
			use super::*;
			#crate_include

			#bare_functions

			#trait_decl_impl

			#host_functions
		}
	};

	let res = expander::Expander::new("runtime_interface")
		.dry(std::env::var("EXPAND_MACROS").is_err())
		.verbose(true)
		.write_to_out_dir(res)
		.expect("Does not fail because of IO in OUT_DIR; qed");

	Ok(res)
}
