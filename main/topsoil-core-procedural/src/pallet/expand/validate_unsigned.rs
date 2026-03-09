// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::{pallet::Def, COUNTER};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Ident};

pub fn expand_validate_unsigned(def: &mut Def) -> TokenStream {
	let count = COUNTER.with(|counter| counter.borrow_mut().inc());
	let macro_ident =
		Ident::new(&format!("__is_validate_unsigned_part_defined_{}", count), def.item.span());

	let maybe_compile_error = if def.validate_unsigned.is_none() {
		quote! {
			compile_error!(concat!(
				"`",
				stringify!($pallet_name),
				"` does not have #[pallet::validate_unsigned] defined, perhaps you should \
				remove `ValidateUnsigned` from construct_runtime?",
			));
		}
	} else {
		TokenStream::new()
	};

	quote! {
		#[doc(hidden)]
		pub mod __substrate_validate_unsigned_check {
			#[macro_export]
			#[doc(hidden)]
			macro_rules! #macro_ident {
				($pallet_name:ident) => {
					#maybe_compile_error
				}
			}

			#[doc(hidden)]
			pub use #macro_ident as is_validate_unsigned_part_defined;
		}
	}
}
