// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use proc_macro2::Span;

use crate::pallet::Def;

pub fn expand_doc_only(def: &mut Def) -> proc_macro2::TokenStream {
	let dispatchables = if let Some(call_def) = &def.call {
		let type_impl_generics = def.type_impl_generics(Span::call_site());
		call_def
			.methods
			.iter()
			.map(|method| {
				let name = &method.name;
				let args = &method
					.args
					.iter()
					.map(|(_, arg_name, arg_type)| quote::quote!( #arg_name: #arg_type, ))
					.collect::<proc_macro2::TokenStream>();
				let docs = &method.docs;

				let real = format!(" [`Pallet::{}`].", name);
				quote::quote!(
					#( #[doc = #docs] )*
					///
					/// # Warning: Doc-Only
					///
					/// This function is an automatically generated, and is doc-only, uncallable
					/// stub. See the real version in
					#[ doc = #real ]
					pub fn #name<#type_impl_generics>(#args) { unreachable!(); }
				)
			})
			.collect::<proc_macro2::TokenStream>()
	} else {
		quote::quote!()
	};

	let storage_types = def
		.storages
		.iter()
		.map(|storage| {
			let storage_name = &storage.ident;
			let storage_type_docs = &storage.docs;
			let real = format!("[`pallet::{}`].", storage_name);
			quote::quote!(
				#( #[doc = #storage_type_docs] )*
				///
				/// # Warning: Doc-Only
				///
				/// This type is automatically generated, and is doc-only. See the real version in
				#[ doc = #real ]
				pub struct #storage_name();
			)
		})
		.collect::<proc_macro2::TokenStream>();

	quote::quote!(
		/// Auto-generated docs-only module listing all (public and private) defined storage types
		/// for this pallet.
		///
		/// # Warning: Doc-Only
		///
		/// Members of this module cannot be used directly and are only provided for documentation
		/// purposes.
		///
		/// To see the actual storage type, find a struct with the same name at the root of the
		/// pallet, in the list of [*Type Definitions*](../index.html#types).
		#[cfg(doc)]
		pub mod storage_types {
			use super::*;
			#storage_types
		}

		/// Auto-generated docs-only module listing all defined dispatchables for this pallet.
		///
		/// # Warning: Doc-Only
		///
		/// Members of this module cannot be used directly and are only provided for documentation
		/// purposes. To see the real version of each dispatchable, look for them in [`Pallet`] or
		/// [`Call`].
		#[cfg(doc)]
		pub mod dispatchables {
			use super::*;
			#dispatchables
		}
	)
}
