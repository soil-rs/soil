// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use syn::spanned::Spanned;
pub struct RuntimeStructDef {
	pub ident: syn::Ident,
}

impl RuntimeStructDef {
	pub fn try_from(item: &mut syn::Item) -> syn::Result<Self> {
		let item = if let syn::Item::Struct(item) = item {
			item
		} else {
			let msg = "Invalid runtime::runtime, expected struct definition";
			return Err(syn::Error::new(item.span(), msg));
		};

		Ok(Self { ident: item.ident.clone() })
	}
}
