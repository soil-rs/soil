// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use syn::spanned::Spanned;

/// Derive PartialOrd but do not bound any generic.
pub fn derive_partial_ord_no_bound(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input: syn::DeriveInput = match syn::parse(input) {
		Ok(input) => input,
		Err(e) => return e.to_compile_error().into(),
	};

	let name = &input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let impl_ =
		match input.data {
			syn::Data::Struct(struct_) => match struct_.fields {
				syn::Fields::Named(named) => {
					let fields =
						named.named.iter().map(|i| &i.ident).map(
							|i| quote::quote_spanned!(i.span() => self.#i.partial_cmp(&other.#i)),
						);

					quote::quote!(
						Some(core::cmp::Ordering::Equal)
							#(
								.and_then(|order| {
									let next_order = #fields?;
									Some(order.then(next_order))
								})
							)*
					)
				},
				syn::Fields::Unnamed(unnamed) => {
					let fields =
						unnamed.unnamed.iter().enumerate().map(|(i, _)| syn::Index::from(i)).map(
							|i| quote::quote_spanned!(i.span() => self.#i.partial_cmp(&other.#i)),
						);

					quote::quote!(
						Some(core::cmp::Ordering::Equal)
							#(
								.and_then(|order| {
									let next_order = #fields?;
									Some(order.then(next_order))
								})
							)*
					)
				},
				syn::Fields::Unit => {
					quote::quote!(Some(core::cmp::Ordering::Equal))
				},
			},
			syn::Data::Enum(_) => {
				let msg = "Enum type not supported by `derive(PartialOrdNoBound)`";
				return syn::Error::new(input.span(), msg).to_compile_error().into();
			},
			syn::Data::Union(_) => {
				let msg = "Union type not supported by `derive(PartialOrdNoBound)`";
				return syn::Error::new(input.span(), msg).to_compile_error().into();
			},
		};

	quote::quote!(
		const _: () = {
			#[allow(deprecated)]
			impl #impl_generics core::cmp::PartialOrd for #name #ty_generics #where_clause {
				fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
					#impl_
				}
			}
		};
	)
	.into()
}
