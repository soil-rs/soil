// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, Result};
use crate::tools::generate_access_from_frame_or_crate;

pub fn transactional(_attr: TokenStream, input: TokenStream) -> Result<TokenStream> {
	let ItemFn { attrs, vis, sig, block } = syn::parse(input)?;

	let crate_ = generate_access_from_frame_or_crate("topsoil-core")?;
	let output = quote! {
		#(#attrs)*
		#vis #sig {
			use #crate_::storage::{with_transaction, TransactionOutcome};
			with_transaction(|| {
				let r = (|| { #block })();
				if r.is_ok() {
					TransactionOutcome::Commit(r)
				} else {
					TransactionOutcome::Rollback(r)
				}
			})
		}
	};

	Ok(output.into())
}

pub fn require_transactional(_attr: TokenStream, input: TokenStream) -> Result<TokenStream> {
	let ItemFn { attrs, vis, sig, block } = syn::parse(input)?;

	let crate_ = generate_access_from_frame_or_crate("topsoil-core")?;
	let output = quote! {
		#(#attrs)*
		#vis #sig {
			if !#crate_::storage::transactional::is_transactional() {
				return Err(#crate_::subsoil::runtime::TransactionalError::NoLayer.into());
			}
			#block
		}
	};

	Ok(output.into())
}
