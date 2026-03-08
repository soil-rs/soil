// This file is part of Soil.

// Copyright (C) Soil contributors.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{Error, Expr, ItemFn, Path, Result};

pub(crate) fn prefix_logs_with(arg: TokenStream, item: TokenStream) -> TokenStream {
	if arg.is_empty() {
		return Error::new(
			proc_macro2::Span::call_site(),
			"missing argument: prefix. Example: prefix_logs_with(\"Relaychain\")",
		)
		.to_compile_error()
		.into();
	}

	let prefix_expr = syn::parse_macro_input!(arg as Expr);
	let item_fn = syn::parse_macro_input!(item as ItemFn);

	let crate_name = match resolve_sc_tracing() {
		Ok(path) => path,
		Err(err) => return err.to_compile_error().into(),
	};

	let syn::ItemFn { attrs, vis, sig, block } = item_fn;

	if sig.asyncness.is_some() {
		(quote! {
			#(#attrs)*
			#vis #sig {
				let span = #crate_name::tracing::info_span!(
					#crate_name::logging::PREFIX_LOG_SPAN,
					name = #prefix_expr,
				);

				#crate_name::tracing::Instrument::instrument(async move {
					#block
				}, span).await
			}
		})
		.into()
	} else {
		(quote! {
			#(#attrs)*
			#vis #sig {
				let span = #crate_name::tracing::info_span!(
					#crate_name::logging::PREFIX_LOG_SPAN,
					name = #prefix_expr,
				);
				let _enter = span.enter();

				#block
			}
		})
		.into()
	}
}

fn resolve_sc_tracing() -> Result<Path> {
	match crate_name("polkadot-sdk") {
		Ok(FoundCrate::Itself) => syn::parse_str("polkadot_sdk::soil_client::tracing"),
		Ok(FoundCrate::Name(sdk_name)) => {
			syn::parse_str(&format!("{}::soil_client::tracing", sdk_name))
		},
		Err(_) => match crate_name("soil-client") {
			Ok(FoundCrate::Itself) => syn::parse_str("crate::tracing"),
			Ok(FoundCrate::Name(name)) => syn::parse_str(&format!("{}::tracing", name)),
			Err(_) => match crate_name("sc-tracing") {
				Ok(FoundCrate::Itself) => syn::parse_str("sc_tracing"),
				Ok(FoundCrate::Name(name)) => syn::parse_str(&name),
				Err(e) => Err(syn::Error::new(Span::call_site(), e)),
			},
		},
	}
}
