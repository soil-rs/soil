// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::construct_runtime::Pallet;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;

/// Expands aggregate `RuntimeTask` enum.
pub fn expand_outer_task(
	runtime_name: &Ident,
	pallet_decls: &[Pallet],
	scrate: &TokenStream2,
) -> TokenStream2 {
	let mut from_impls = Vec::new();
	let mut task_variants = Vec::new();
	let mut variant_names = Vec::new();
	let mut task_types = Vec::new();
	let mut cfg_attrs = Vec::new();
	for decl in pallet_decls {
		if decl.find_part("Task").is_none() {
			continue;
		}

		let variant_name = &decl.name;
		let path = &decl.path;
		let index = decl.index;
		let instance = decl.instance.as_ref().map(|instance| quote!(, #path::#instance));
		let task_type = quote!(#path::Task<#runtime_name #instance>);

		let attr = decl.get_attributes();

		from_impls.push(quote! {
			#attr
			impl From<#task_type> for RuntimeTask {
				fn from(hr: #task_type) -> Self {
					RuntimeTask::#variant_name(hr)
				}
			}

			#attr
			impl TryInto<#task_type> for RuntimeTask {
				type Error = ();

				fn try_into(self) -> Result<#task_type, Self::Error> {
					match self {
						RuntimeTask::#variant_name(hr) => Ok(hr),
						_ => Err(()),
					}
				}
			}
		});

		task_variants.push(quote! {
			#attr
			#[codec(index = #index)]
			#variant_name(#task_type),
		});

		variant_names.push(quote!(#variant_name));

		task_types.push(task_type);

		cfg_attrs.push(attr);
	}

	let prelude = quote!(#scrate::traits::tasks::__private);

	const INCOMPLETE_MATCH_QED: &'static str =
		"cannot have an instantiated RuntimeTask without some Task variant in the runtime. QED";

	let output = quote! {
		/// An aggregation of all `Task` enums across all pallets included in the current runtime.
		#[derive(
			Clone, Eq, PartialEq,
			#scrate::__private::codec::Encode,
			#scrate::__private::codec::Decode,
			#scrate::__private::codec::DecodeWithMemTracking,
			#scrate::__private::scale_info::TypeInfo,
			#scrate::__private::Debug,
		)]
		pub enum RuntimeTask {
			#( #task_variants )*
		}

		#[automatically_derived]
		impl #scrate::traits::Task for RuntimeTask {
			type Enumeration = #prelude::IntoIter<RuntimeTask>;

			fn is_valid(&self) -> bool {
				match self {
					#(
						#cfg_attrs
						RuntimeTask::#variant_names(val) => val.is_valid(),
					)*
					_ => unreachable!(#INCOMPLETE_MATCH_QED),
				}
			}

			fn run(&self) -> Result<(), #scrate::traits::tasks::__private::DispatchError> {
				match self {
					#(
						#cfg_attrs
						RuntimeTask::#variant_names(val) => val.run(),
					)*
					_ => unreachable!(#INCOMPLETE_MATCH_QED),
				}
			}

			fn weight(&self) -> #scrate::pallet_prelude::Weight {
				match self {
					#(
						#cfg_attrs
						RuntimeTask::#variant_names(val) => val.weight(),
					)*
					_ => unreachable!(#INCOMPLETE_MATCH_QED),
				}
			}

			fn task_index(&self) -> u32 {
				match self {
					#(
						#cfg_attrs
						RuntimeTask::#variant_names(val) => val.task_index(),
					)*
					_ => unreachable!(#INCOMPLETE_MATCH_QED),
				}
			}

			fn iter() -> Self::Enumeration {
				let mut all_tasks = Vec::new();
				#(
					#cfg_attrs
					all_tasks.extend(<#task_types>::iter().map(RuntimeTask::from).collect::<Vec<_>>());
				)*
				all_tasks.into_iter()
			}
		}

		#( #from_impls )*
	};

	output
}
