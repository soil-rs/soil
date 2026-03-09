// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::pallet::parse::helper::MutItemAttrs;
use quote::ToTokens;

pub(crate) fn take_first_item_runtime_attr<Attr>(
	item: &mut impl MutItemAttrs,
) -> syn::Result<Option<Attr>>
where
	Attr: syn::parse::Parse,
{
	let attrs = if let Some(attrs) = item.mut_item_attrs() { attrs } else { return Ok(None) };

	if let Some(index) = attrs.iter().position(|attr| {
		attr.path().segments.first().map_or(false, |segment| segment.ident == "runtime")
	}) {
		let runtime_attr = attrs.remove(index);
		Ok(Some(syn::parse2(runtime_attr.into_token_stream())?))
	} else {
		Ok(None)
	}
}
