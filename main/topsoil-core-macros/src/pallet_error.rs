// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use quote::ToTokens;
use crate::tools::generate_access_from_frame_or_crate;

// Derive `PalletError`
pub fn derive_pallet_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let syn::DeriveInput { ident: name, generics, data, .. } = match syn::parse(input) {
		Ok(input) => input,
		Err(e) => return e.to_compile_error().into(),
	};

	let topsoil_core = match generate_access_from_frame_or_crate("topsoil-core") {
		Ok(c) => c,
		Err(e) => return e.into_compile_error().into(),
	};
	let topsoil_core = &topsoil_core;
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let max_encoded_size = match data {
		syn::Data::Struct(syn::DataStruct { fields, .. }) => match fields {
			syn::Fields::Named(syn::FieldsNamed { named: fields, .. })
			| syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed: fields, .. }) => {
				let maybe_field_tys = fields
					.iter()
					.map(|f| generate_field_types(f, &topsoil_core))
					.collect::<syn::Result<Vec<_>>>();
				let field_tys = match maybe_field_tys {
					Ok(tys) => tys.into_iter().flatten(),
					Err(e) => return e.into_compile_error().into(),
				};
				quote::quote! {
					0_usize
					#(
						.saturating_add(<
							#field_tys as #topsoil_core::traits::PalletError
						>::MAX_ENCODED_SIZE)
					)*
				}
			},
			syn::Fields::Unit => quote::quote!(0),
		},
		syn::Data::Enum(syn::DataEnum { variants, .. }) => {
			let field_tys = variants
				.iter()
				.map(|variant| generate_variant_field_types(variant, &topsoil_core))
				.collect::<Result<Vec<Option<Vec<proc_macro2::TokenStream>>>, syn::Error>>();

			let field_tys = match field_tys {
				Ok(tys) => tys.into_iter().flatten().collect::<Vec<_>>(),
				Err(e) => return e.to_compile_error().into(),
			};

			// We start with `1`, because the discriminant of an enum is stored as u8
			if field_tys.is_empty() {
				quote::quote!(1)
			} else {
				let variant_sizes = field_tys.into_iter().map(|variant_field_tys| {
					quote::quote! {
						1_usize
						#(.saturating_add(<
							#variant_field_tys as #topsoil_core::traits::PalletError
						>::MAX_ENCODED_SIZE))*
					}
				});

				quote::quote! {{
					let mut size = 1_usize;
					let mut tmp = 0_usize;
					#(
						tmp = #variant_sizes;
						size = if tmp > size { tmp } else { size };
						tmp = 0_usize;
					)*
					size
				}}
			}
		},
		syn::Data::Union(syn::DataUnion { union_token, .. }) => {
			let msg = "Cannot derive `PalletError` for union; please implement it directly";
			return syn::Error::new(union_token.span, msg).into_compile_error().into();
		},
	};

	quote::quote!(
		#[allow(deprecated)]
		const _: () = {
			impl #impl_generics #topsoil_core::traits::PalletError
				for #name #ty_generics #where_clause
			{
				const MAX_ENCODED_SIZE: usize = #max_encoded_size;
			}
		};
	)
	.into()
}

fn generate_field_types(
	field: &syn::Field,
	scrate: &syn::Path,
) -> syn::Result<Option<proc_macro2::TokenStream>> {
	let attrs = &field.attrs;

	for attr in attrs {
		if attr.path().is_ident("codec") {
			let mut res = None;

			attr.parse_nested_meta(|meta| {
				if meta.path.is_ident("skip") {
					res = Some(None);
				} else if meta.path.is_ident("compact") {
					let field_ty = &field.ty;
					res = Some(Some(quote::quote!(#scrate::__private::codec::Compact<#field_ty>)));
				} else if meta.path.is_ident("compact") {
					res = Some(Some(meta.value()?.parse()?));
				}

				Ok(())
			})?;

			if let Some(v) = res {
				return Ok(v);
			}
		}
	}

	Ok(Some(field.ty.to_token_stream()))
}

fn generate_variant_field_types(
	variant: &syn::Variant,
	scrate: &syn::Path,
) -> syn::Result<Option<Vec<proc_macro2::TokenStream>>> {
	let attrs = &variant.attrs;

	for attr in attrs {
		if attr.path().is_ident("codec") {
			let mut skip = false;

			// We ignore the error intentionally as this isn't `codec(skip)` when
			// `parse_nested_meta` fails.
			let _ = attr.parse_nested_meta(|meta| {
				skip = meta.path.is_ident("skip");
				Ok(())
			});

			if skip {
				return Ok(None);
			}
		}
	}

	match &variant.fields {
		syn::Fields::Named(syn::FieldsNamed { named: fields, .. })
		| syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed: fields, .. }) => {
			let field_tys = fields
				.iter()
				.map(|field| generate_field_types(field, scrate))
				.collect::<syn::Result<Vec<_>>>()?;
			Ok(Some(field_tys.into_iter().flatten().collect()))
		},
		syn::Fields::Unit => Ok(None),
	}
}
