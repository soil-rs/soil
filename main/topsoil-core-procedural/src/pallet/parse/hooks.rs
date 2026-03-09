// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use super::helper;
use syn::spanned::Spanned;

/// Implementation of the pallet hooks.
pub struct HooksDef {
	/// A set of usage of instance, must be check for consistency with trait.
	pub instances: Vec<helper::InstanceUsage>,
	/// The where_clause used.
	pub where_clause: Option<syn::WhereClause>,
	/// The span of the pallet::hooks attribute.
	pub attr_span: proc_macro2::Span,
	/// Boolean flag, set to true if the `on_runtime_upgrade` method of hooks was implemented.
	pub has_runtime_upgrade: bool,
}

impl HooksDef {
	pub fn try_from(attr_span: proc_macro2::Span, item: &mut syn::Item) -> syn::Result<Self> {
		let item = if let syn::Item::Impl(item) = item {
			item
		} else {
			let msg = "Invalid pallet::hooks, expected item impl";
			return Err(syn::Error::new(item.span(), msg));
		};

		let instances = vec![
			helper::check_pallet_struct_usage(&item.self_ty)?,
			helper::check_impl_gen(&item.generics, item.impl_token.span())?,
		];

		let item_trait = &item
			.trait_
			.as_ref()
			.ok_or_else(|| {
				let msg = "Invalid pallet::hooks, expected impl<..> Hooks \
					for Pallet<..>";
				syn::Error::new(item.span(), msg)
			})?
			.1;

		if item_trait.segments.len() != 1 || item_trait.segments[0].ident != "Hooks" {
			let msg = format!(
				"Invalid pallet::hooks, expected trait to be `Hooks` found `{}`\
				, you can import from `topsoil_core::pallet_prelude`",
				quote::quote!(#item_trait)
			);

			return Err(syn::Error::new(item_trait.span(), msg));
		}

		let has_runtime_upgrade = item.items.iter().any(|i| match i {
			syn::ImplItem::Fn(method) => method.sig.ident == "on_runtime_upgrade",
			_ => false,
		});

		Ok(Self {
			attr_span,
			instances,
			has_runtime_upgrade,
			where_clause: item.generics.where_clause.clone(),
		})
	}
}
