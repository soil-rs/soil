// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::{pallet::Def, NUMBER_OF_INSTANCE};
use proc_macro2::Span;

/// * Provide inherent instance to be used by construct_runtime
/// * Provide Instance1 ..= Instance16 for instantiable pallet
pub fn expand_instances(def: &mut Def) -> proc_macro2::TokenStream {
	let topsoil_support = &def.topsoil_support;
	let inherent_ident = syn::Ident::new(crate::INHERENT_INSTANCE_NAME, Span::call_site());
	let instances = if def.config.has_instance {
		(1..=NUMBER_OF_INSTANCE)
			.map(|i| syn::Ident::new(&format!("Instance{}", i), Span::call_site()))
			.collect()
	} else {
		vec![]
	};

	quote::quote!(
		/// Hidden instance generated to be internally used when module is used without
		/// instance.
		#[doc(hidden)]
		pub type #inherent_ident = ();

		#( pub use #topsoil_support::instances::#instances; )*
	)
}
