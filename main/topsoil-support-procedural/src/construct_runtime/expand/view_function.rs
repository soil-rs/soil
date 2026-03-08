// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::construct_runtime::Pallet;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};

/// Expands implementation of runtime level `DispatchViewFunction`.
pub fn expand_outer_query(
	runtime_name: &Ident,
	pallet_decls: &[Pallet],
	scrate: &TokenStream2,
) -> TokenStream2 {
	let runtime_view_function = syn::Ident::new("RuntimeViewFunction", Span::call_site());

	let prefix_conditionals = pallet_decls.iter().map(|pallet| {
		let pallet_name = &pallet.name;
		let attr = pallet.get_attributes();
		quote::quote! {
			#attr
			if id.prefix == <#pallet_name as #scrate::view_functions::ViewFunctionIdPrefix>::prefix() {
				return <#pallet_name as #scrate::view_functions::DispatchViewFunction>::dispatch_view_function(id, input, output)
			}
		}
	});

	quote::quote! {
		/// Runtime query type.
		#[derive(
			Clone, PartialEq, Eq,
			#scrate::__private::codec::Encode,
			#scrate::__private::codec::Decode,
			#scrate::__private::codec::DecodeWithMemTracking,
			#scrate::__private::scale_info::TypeInfo,
			#scrate::__private::Debug,
		)]
		pub enum #runtime_view_function {}

		const _: () = {
			impl #scrate::view_functions::DispatchViewFunction for #runtime_view_function {
				fn dispatch_view_function<O: #scrate::__private::codec::Output>(
					id: & #scrate::view_functions::ViewFunctionId,
					input: &mut &[u8],
					output: &mut O
				) -> Result<(), #scrate::view_functions::ViewFunctionDispatchError>
				{
					#( #prefix_conditionals )*
					Err(#scrate::view_functions::ViewFunctionDispatchError::NotFound(id.clone()))
				}
			}

			impl #runtime_name {
				/// Convenience function for view functions dispatching and execution from the runtime API.
				pub fn execute_view_function(
					id: #scrate::view_functions::ViewFunctionId,
					input: #scrate::__private::Vec<::core::primitive::u8>,
				) -> Result<#scrate::__private::Vec<::core::primitive::u8>, #scrate::view_functions::ViewFunctionDispatchError>
				{
					let mut output = #scrate::__private::vec![];
					<#runtime_view_function as #scrate::view_functions::DispatchViewFunction>::dispatch_view_function(&id, &mut &input[..], &mut output)?;
					Ok(output)
				}
			}
		};
	}
}
