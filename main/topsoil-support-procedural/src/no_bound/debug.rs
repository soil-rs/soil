// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use syn::spanned::Spanned;

/// Derive Debug but do not bound any generics.
pub fn derive_debug_no_bound(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as syn::DeriveInput);

	let input_ident = &input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let impl_ = match input.data {
		syn::Data::Struct(struct_) => match struct_.fields {
			syn::Fields::Named(named) => {
				let fields =
					named.named.iter().map(|i| &i.ident).map(
						|i| quote::quote_spanned!(i.span() => .field(stringify!(#i), &self.#i) ),
					);

				quote::quote!(
					fmt.debug_struct(stringify!(#input_ident))
						#( #fields )*
						.finish()
				)
			},
			syn::Fields::Unnamed(unnamed) => {
				let fields = unnamed
					.unnamed
					.iter()
					.enumerate()
					.map(|(i, _)| syn::Index::from(i))
					.map(|i| quote::quote_spanned!(i.span() => .field(&self.#i) ));

				quote::quote!(
					fmt.debug_tuple(stringify!(#input_ident))
						#( #fields )*
						.finish()
				)
			},
			syn::Fields::Unit => quote::quote!(fmt.write_str(stringify!(#input_ident))),
		},
		syn::Data::Enum(enum_) => {
			let variants = enum_.variants.iter().map(|variant| {
				let ident = &variant.ident;
				let full_variant_str = format!("{}::{}", input_ident, ident);
				match &variant.fields {
					syn::Fields::Named(named) => {
						let captured = named.named.iter().map(|i| &i.ident);
						let debugged = captured.clone().map(|i| {
							quote::quote_spanned!(i.span() =>
								.field(stringify!(#i), &#i)
							)
						});
						quote::quote!(
							Self::#ident { #( ref #captured, )* } => {
								fmt.debug_struct(#full_variant_str)
									#( #debugged )*
									.finish()
							}
						)
					},
					syn::Fields::Unnamed(unnamed) => {
						let captured = unnamed
							.unnamed
							.iter()
							.enumerate()
							.map(|(i, f)| syn::Ident::new(&format!("_{}", i), f.span()));
						let debugged = captured
							.clone()
							.map(|i| quote::quote_spanned!(i.span() => .field(&#i)));
						quote::quote!(
							Self::#ident ( #( ref #captured, )* ) => {
								fmt.debug_tuple(#full_variant_str)
									#( #debugged )*
									.finish()
							}
						)
					},
					syn::Fields::Unit => quote::quote!(
						Self::#ident => fmt.write_str(#full_variant_str)
					),
				}
			});

			quote::quote!(match *self {
				#( #variants, )*
			})
		},
		syn::Data::Union(_) => {
			let msg = "Union type not supported by `derive(DebugNoBound)`";
			return syn::Error::new(input.span(), msg).to_compile_error().into();
		},
	};

	quote::quote!(
		const _: () = {
			#[automatically_derived]
			#[allow(deprecated)]
			impl #impl_generics ::core::fmt::Debug for #input_ident #ty_generics #where_clause {
				fn fmt(&self, fmt: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
					#impl_
				}
			}
		};
	)
	.into()
}
