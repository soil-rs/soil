// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use super::helper;
use syn::spanned::Spanned;

/// Definition for pallet genesis config type.
///
/// Either:
/// * `struct GenesisConfig`
/// * `enum GenesisConfig`
pub struct GenesisConfigDef {
	/// The index of item in pallet module.
	pub index: usize,
	/// The kind of generic the type `GenesisConfig` has.
	pub gen_kind: super::GenericKind,
	/// A set of usage of instance, must be check for consistency with trait.
	pub instances: Vec<helper::InstanceUsage>,
	/// The ident of genesis_config, can be used for span.
	pub genesis_config: syn::Ident,
}

impl GenesisConfigDef {
	pub fn try_from(index: usize, item: &mut syn::Item) -> syn::Result<Self> {
		let item_span = item.span();
		let (vis, ident, generics) = match &item {
			syn::Item::Enum(item) => (&item.vis, &item.ident, &item.generics),
			syn::Item::Struct(item) => (&item.vis, &item.ident, &item.generics),
			_ => {
				let msg = "Invalid pallet::genesis_config, expected enum or struct";
				return Err(syn::Error::new(item.span(), msg));
			},
		};

		let mut instances = vec![];
		// NOTE: GenesisConfig is not allowed to be only generic on I because it is not supported
		// by construct_runtime.
		if let Some(u) = helper::check_type_def_optional_gen(generics, ident.span())? {
			instances.push(u);
		}

		let has_instance = generics.type_params().any(|t| t.ident == "I");
		let has_config = generics.type_params().any(|t| t.ident == "T");
		let gen_kind = super::GenericKind::from_gens(has_config, has_instance)
			.expect("Checked by `helper::check_type_def_optional_gen` above");

		if !matches!(vis, syn::Visibility::Public(_)) {
			let msg = "Invalid pallet::genesis_config, GenesisConfig must be public";
			return Err(syn::Error::new(item_span, msg));
		}

		if ident != "GenesisConfig" {
			let msg = "Invalid pallet::genesis_config, ident must `GenesisConfig`";
			return Err(syn::Error::new(ident.span(), msg));
		}

		Ok(GenesisConfigDef { index, genesis_config: ident.clone(), instances, gen_kind })
	}
}
