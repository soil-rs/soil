// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use super::composite_helper;
use crate::construct_runtime::Pallet;
use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_outer_lock_id(pallet_decls: &[Pallet], scrate: &TokenStream) -> TokenStream {
	let mut conversion_fns = Vec::new();
	let mut lock_id_variants = Vec::new();
	for decl in pallet_decls {
		if let Some(_) = decl.find_part("LockId") {
			let variant_name = &decl.name;
			let path = &decl.path;
			let index = decl.index;
			let instance = decl.instance.as_ref();

			conversion_fns.push(composite_helper::expand_conversion_fn(
				"LockId",
				path,
				instance,
				variant_name,
			));

			lock_id_variants.push(composite_helper::expand_variant(
				"LockId",
				index,
				path,
				instance,
				variant_name,
			));
		}
	}

	quote! {
		/// An identifier for each lock placed on funds.
		#[derive(
			Copy, Clone, Eq, PartialEq,
			#scrate::__private::codec::Encode,
			#scrate::__private::codec::Decode,
			#scrate::__private::codec::DecodeWithMemTracking,
			#scrate::__private::codec::MaxEncodedLen,
			#scrate::__private::scale_info::TypeInfo,
			#scrate::__private::Debug,
		)]
		pub enum RuntimeLockId {
			#( #lock_id_variants )*
		}

		#( #conversion_fns )*
	}
}
