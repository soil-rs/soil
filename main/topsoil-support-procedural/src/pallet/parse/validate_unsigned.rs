// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use super::helper;
use syn::spanned::Spanned;

/// The definition of the pallet validate unsigned implementation.
pub struct ValidateUnsignedDef {}

impl ValidateUnsignedDef {
	pub fn try_from(item: &mut syn::Item) -> syn::Result<Self> {
		let item = if let syn::Item::Impl(item) = item {
			item
		} else {
			let msg = "Invalid pallet::validate_unsigned, expected item impl";
			return Err(syn::Error::new(item.span(), msg));
		};

		if item.trait_.is_none() {
			let msg = "Invalid pallet::validate_unsigned, expected impl<..> ValidateUnsigned for \
				Pallet<..>";
			return Err(syn::Error::new(item.span(), msg));
		}

		if let Some(last) = item.trait_.as_ref().unwrap().1.segments.last() {
			if last.ident != "ValidateUnsigned" {
				let msg = "Invalid pallet::validate_unsigned, expected trait ValidateUnsigned";
				return Err(syn::Error::new(last.span(), msg));
			}
		} else {
			let msg = "Invalid pallet::validate_unsigned, expected impl<..> ValidateUnsigned for \
				Pallet<..>";
			return Err(syn::Error::new(item.span(), msg));
		}

		helper::check_pallet_struct_usage(&item.self_ty)?;
		helper::check_impl_gen(&item.generics, item.impl_token.span())?;

		Ok(ValidateUnsignedDef {})
	}
}
