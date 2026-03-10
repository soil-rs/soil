// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use super::helper;
use syn::spanned::Spanned;

/// Definition for pallet genesis build implementation.
pub struct GenesisBuildDef {
	/// A set of usage of instance, must be check for consistency with trait.
	pub instances: Option<Vec<helper::InstanceUsage>>,
	/// The where_clause used.
	pub where_clause: Option<syn::WhereClause>,
	/// The span of the pallet::genesis_build attribute.
	pub attr_span: proc_macro2::Span,
}

impl GenesisBuildDef {
	pub fn try_from(attr_span: proc_macro2::Span, item: &mut syn::Item) -> syn::Result<Self> {
		let item = if let syn::Item::Impl(item) = item {
			item
		} else {
			let msg = "Invalid pallet::genesis_build, expected item impl";
			return Err(syn::Error::new(item.span(), msg));
		};

		let item_trait = &item
			.trait_
			.as_ref()
			.ok_or_else(|| {
				let msg = "Invalid pallet::genesis_build, expected impl<..> GenesisBuild<..> \
					for GenesisConfig<..>";
				syn::Error::new(item.span(), msg)
			})?
			.1;

		let instances =
			helper::check_genesis_builder_usage(item_trait)?.map(|instances| vec![instances]);

		Ok(Self { attr_span, instances, where_clause: item.generics.where_clause.clone() })
	}
}
