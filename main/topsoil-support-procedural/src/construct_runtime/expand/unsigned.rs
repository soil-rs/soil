// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::construct_runtime::Pallet;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn expand_outer_validate_unsigned(
	runtime: &Ident,
	pallet_decls: &[Pallet],
	scrate: &TokenStream,
) -> TokenStream {
	let mut pallet_names = Vec::new();
	let mut pallet_attrs = Vec::new();
	let mut query_validate_unsigned_part_macros = Vec::new();

	for pallet_decl in pallet_decls {
		if pallet_decl.exists_part("ValidateUnsigned") {
			let name = &pallet_decl.name;
			let path = &pallet_decl.path;
			let attr = pallet_decl.get_attributes();

			pallet_names.push(name);
			pallet_attrs.push(attr);
			query_validate_unsigned_part_macros.push(quote! {
				#path::__substrate_validate_unsigned_check::is_validate_unsigned_part_defined!(#name);
			});
		}
	}

	quote! {
		#( #query_validate_unsigned_part_macros )*

		impl #scrate::unsigned::ValidateUnsigned for #runtime {
			type Call = RuntimeCall;

			fn pre_dispatch(call: &Self::Call) -> Result<(), #scrate::unsigned::TransactionValidityError> {
				#[allow(unreachable_patterns)]
				match call {
					#(
						#pallet_attrs
						RuntimeCall::#pallet_names(inner_call) => #pallet_names::pre_dispatch(inner_call),
					)*
					// pre-dispatch should not stop inherent extrinsics, validation should prevent
					// including arbitrary (non-inherent) extrinsics to blocks.
					_ => Ok(()),
				}
			}

			fn validate_unsigned(
				#[allow(unused_variables)]
				source: #scrate::unsigned::TransactionSource,
				call: &Self::Call,
			) -> #scrate::unsigned::TransactionValidity {
				#[allow(unreachable_patterns)]
				match call {
					#(
						#pallet_attrs
						RuntimeCall::#pallet_names(inner_call) => #pallet_names::validate_unsigned(source, inner_call),
					)*
					_ => #scrate::unsigned::UnknownTransaction::NoUnsignedValidator.into(),
				}
			}
		}
	}
}
