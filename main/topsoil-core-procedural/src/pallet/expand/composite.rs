// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::pallet::Def;
use proc_macro2::TokenStream;

/// Expands `composite_enum` and adds the `VariantCount` implementation for it.
pub fn expand_composites(def: &mut Def) -> TokenStream {
	let mut expand = quote::quote!();
	let topsoil_core = &def.topsoil_core;

	for composite in &def.composites {
		let name = &composite.ident;
		let (impl_generics, ty_generics, where_clause) = composite.generics.split_for_impl();
		let variants_count = composite.variant_count;

		// add `VariantCount` implementation for `composite_enum`
		expand.extend(quote::quote_spanned!(composite.attr_span =>
			impl #impl_generics #topsoil_core::traits::VariantCount for #name #ty_generics #where_clause {
				const VARIANT_COUNT: u32 = #variants_count;
			}
		));
	}

	expand
}
