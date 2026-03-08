// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Macros to derive chain spec extension traits implementation.

mod impls;

use proc_macro::TokenStream;

#[proc_macro_derive(ChainSpecGroup)]
pub fn group_derive(input: TokenStream) -> TokenStream {
	match syn::parse(input) {
		Ok(ast) => impls::group_derive(&ast),
		Err(e) => e.to_compile_error().into(),
	}
}

#[proc_macro_derive(ChainSpecExtension, attributes(forks))]
pub fn extensions_derive(input: TokenStream) -> TokenStream {
	match syn::parse(input) {
		Ok(ast) => impls::extension_derive(&ast),
		Err(e) => e.to_compile_error().into(),
	}
}
