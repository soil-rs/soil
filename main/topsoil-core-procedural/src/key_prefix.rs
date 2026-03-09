// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{Ident, Result};
use topsoil_core_procedural_tools::generate_access_from_frame_or_crate;

const MAX_IDENTS: usize = 18;

pub fn impl_key_prefix_for_tuples(input: proc_macro::TokenStream) -> Result<TokenStream> {
	if !input.is_empty() {
		return Err(syn::Error::new(Span::call_site(), "No arguments expected"));
	}

	let mut all_trait_impls = TokenStream::new();
	let topsoil_core = generate_access_from_frame_or_crate("topsoil-core")?;

	for i in 2..=MAX_IDENTS {
		let current_tuple = (0..i)
			.map(|n| Ident::new(&format!("Tuple{}", n), Span::call_site()))
			.collect::<Vec<_>>();

		for prefix_count in 1..i {
			let (prefixes, suffixes) = current_tuple.split_at(prefix_count);

			let hashers = current_tuple
				.iter()
				.map(|ident| format_ident!("Hasher{}", ident))
				.collect::<Vec<_>>();
			let kargs =
				prefixes.iter().map(|ident| format_ident!("KArg{}", ident)).collect::<Vec<_>>();
			let partial_keygen = generate_keygen(prefixes);
			let suffix_keygen = generate_keygen(suffixes);
			let suffix_tuple = generate_tuple(suffixes);

			let trait_impls = quote! {
				impl<
					#(#current_tuple: FullCodec + StaticTypeInfo,)*
					#(#hashers: StorageHasher,)*
					#(#kargs: EncodeLike<#prefixes>),*
				> HasKeyPrefix<( #( #kargs, )* )> for ( #( Key<#hashers, #current_tuple>, )* ) {
					type Suffix = #suffix_tuple;

					fn partial_key(prefix: ( #( #kargs, )* )) -> Vec<u8> {
						<#partial_keygen>::final_key(prefix)
					}
				}

				impl<
					#(#current_tuple: FullCodec + StaticTypeInfo,)*
					#(#hashers: ReversibleStorageHasher,)*
					#(#kargs: EncodeLike<#prefixes>),*
				> HasReversibleKeyPrefix<( #( #kargs, )* )> for ( #( Key<#hashers, #current_tuple>, )* ) {
					fn decode_partial_key(key_material: &[u8]) -> Result<
						Self::Suffix,
						#topsoil_core::__private::codec::Error,
					> {
						<#suffix_keygen>::decode_final_key(key_material).map(|k| k.0)
					}
				}
			};

			all_trait_impls.extend(trait_impls);
		}
	}

	Ok(all_trait_impls)
}

fn generate_tuple(idents: &[Ident]) -> TokenStream {
	if idents.len() == 1 {
		idents[0].to_token_stream()
	} else {
		quote!((#(#idents),*))
	}
}

fn generate_keygen(idents: &[Ident]) -> TokenStream {
	if idents.len() == 1 {
		let key = &idents[0];
		let hasher = format_ident!("Hasher{}", key);

		quote!(Key<#hasher, #key>)
	} else {
		let hashers = idents.iter().map(|ident| format_ident!("Hasher{}", ident));

		quote!((#(Key<#hashers, #idents>),*))
	}
}
