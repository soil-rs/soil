// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use super::composite_helper;
use crate::construct_runtime::Pallet;
use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_outer_slash_reason(pallet_decls: &[Pallet], scrate: &TokenStream) -> TokenStream {
	let mut conversion_fns = Vec::new();
	let mut slash_reason_variants = Vec::new();
	for decl in pallet_decls {
		if let Some(_) = decl.find_part("SlashReason") {
			let variant_name = &decl.name;
			let path = &decl.path;
			let index = decl.index;
			let instance = decl.instance.as_ref();

			conversion_fns.push(composite_helper::expand_conversion_fn(
				"SlashReason",
				path,
				instance,
				variant_name,
			));

			slash_reason_variants.push(composite_helper::expand_variant(
				"SlashReason",
				index,
				path,
				instance,
				variant_name,
			));
		}
	}

	quote! {
		/// A reason for slashing funds.
		#[derive(
			Copy, Clone, Eq, PartialEq,
			#scrate::__private::codec::Encode,
			#scrate::__private::codec::Decode,
			#scrate::__private::codec::DecodeWithMemTracking,
			#scrate::__private::codec::MaxEncodedLen,
			#scrate::__private::scale_info::TypeInfo,
			#scrate::__private::Debug,
		)]
		pub enum RuntimeSlashReason {
			#( #slash_reason_variants )*
		}

		#( #conversion_fns )*
	}
}
