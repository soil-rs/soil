// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::{
	construct_runtime::parse::{Pallet, PalletPart, PalletPartKeyword, PalletPath},
	runtime::parse::PalletDeclaration,
};
use quote::ToTokens;
use syn::{punctuated::Punctuated, spanned::Spanned, token, Error};
use topsoil_core_procedural_tools::get_doc_literals;

impl Pallet {
	pub fn try_from(
		attr_span: proc_macro2::Span,
		item: &syn::ItemType,
		pallet_index: u8,
		disable_call: bool,
		disable_unsigned: bool,
		bounds: &Punctuated<syn::TypeParamBound, token::Plus>,
	) -> syn::Result<Self> {
		let name = item.ident.clone();

		let mut pallet_path = None;
		let mut pallet_parts = vec![];

		for (index, bound) in bounds.into_iter().enumerate() {
			if let syn::TypeParamBound::Trait(syn::TraitBound { path, .. }) = bound {
				if index == 0 {
					pallet_path = Some(PalletPath { inner: path.clone() });
				} else {
					let pallet_part = syn::parse2::<PalletPart>(bound.into_token_stream())?;
					pallet_parts.push(pallet_part);
				}
			} else {
				return Err(Error::new(
					attr_span,
					"Invalid pallet declaration, expected a path or a trait object",
				));
			};
		}

		let mut path = pallet_path.ok_or(Error::new(
			attr_span,
			"Invalid pallet declaration, expected a path or a trait object",
		))?;

		let PalletDeclaration { path: inner, instance, .. } =
			PalletDeclaration::try_from(attr_span, item, &path.inner)?;

		path = PalletPath { inner };

		pallet_parts = pallet_parts
			.into_iter()
			.filter(|part| {
				if let (true, &PalletPartKeyword::Call(_)) = (disable_call, &part.keyword) {
					false
				} else if let (true, &PalletPartKeyword::ValidateUnsigned(_)) =
					(disable_unsigned, &part.keyword)
				{
					false
				} else {
					true
				}
			})
			.collect();

		let cfg_pattern = item
			.attrs
			.iter()
			.filter(|attr| attr.path().segments.first().map_or(false, |s| s.ident == "cfg"))
			.map(|attr| {
				attr.parse_args_with(|input: syn::parse::ParseStream| {
					let input = input.parse::<proc_macro2::TokenStream>()?;
					cfg_expr::Expression::parse(&input.to_string())
						.map_err(|e| syn::Error::new(attr.span(), e.to_string()))
				})
			})
			.collect::<syn::Result<Vec<_>>>()?;

		let docs = get_doc_literals(&item.attrs);

		Ok(Pallet {
			is_expanded: true,
			name,
			index: pallet_index,
			path,
			instance,
			cfg_pattern,
			pallet_parts,
			docs,
		})
	}
}

#[test]
fn pallet_parsing_works() {
	use syn::{parse_quote, ItemType};

	let item: ItemType = parse_quote! {
		pub type System = topsoil_system + Call;
	};
	let ItemType { ty, .. } = item.clone();
	let syn::Type::TraitObject(syn::TypeTraitObject { bounds, .. }) = *ty else {
		panic!("Expected a trait object");
	};

	let index = 0;
	let pallet =
		Pallet::try_from(proc_macro2::Span::call_site(), &item, index, false, false, &bounds)
			.unwrap();

	assert_eq!(pallet.name.to_string(), "System");
	assert_eq!(pallet.index, index);
	assert_eq!(pallet.path.to_token_stream().to_string(), "topsoil_core :: system");
	assert_eq!(pallet.instance, None);
}

#[test]
fn pallet_parsing_works_with_instance() {
	use syn::{parse_quote, ItemType};

	let item: ItemType = parse_quote! {
		pub type System = topsoil_system<Instance1> + Call;
	};
	let ItemType { ty, .. } = item.clone();
	let syn::Type::TraitObject(syn::TypeTraitObject { bounds, .. }) = *ty else {
		panic!("Expected a trait object");
	};

	let index = 0;
	let pallet =
		Pallet::try_from(proc_macro2::Span::call_site(), &item, index, false, false, &bounds)
			.unwrap();

	assert_eq!(pallet.name.to_string(), "System");
	assert_eq!(pallet.index, index);
	assert_eq!(pallet.path.to_token_stream().to_string(), "topsoil_core :: system");
	assert_eq!(pallet.instance, Some(parse_quote! { Instance1 }));
}

#[test]
fn pallet_parsing_works_with_pallet() {
	use syn::{parse_quote, ItemType};

	let item: ItemType = parse_quote! {
		pub type System = topsoil_core::system::Pallet<Runtime> + Call;
	};
	let ItemType { ty, .. } = item.clone();
	let syn::Type::TraitObject(syn::TypeTraitObject { bounds, .. }) = *ty else {
		panic!("Expected a trait object");
	};

	let index = 0;
	let pallet =
		Pallet::try_from(proc_macro2::Span::call_site(), &item, index, false, false, &bounds)
			.unwrap();

	assert_eq!(pallet.name.to_string(), "System");
	assert_eq!(pallet.index, index);
	assert_eq!(pallet.path.to_token_stream().to_string(), "topsoil_core :: system");
	assert_eq!(pallet.instance, None);
}

#[test]
fn pallet_parsing_works_with_instance_and_pallet() {
	use syn::{parse_quote, ItemType};

	let item: ItemType = parse_quote! {
		pub type System = topsoil_core::system::Pallet<Runtime, Instance1> + Call;
	};
	let ItemType { ty, .. } = item.clone();
	let syn::Type::TraitObject(syn::TypeTraitObject { bounds, .. }) = *ty else {
		panic!("Expected a trait object");
	};

	let index = 0;
	let pallet =
		Pallet::try_from(proc_macro2::Span::call_site(), &item, index, false, false, &bounds)
			.unwrap();

	assert_eq!(pallet.name.to_string(), "System");
	assert_eq!(pallet.index, index);
	assert_eq!(pallet.path.to_token_stream().to_string(), "topsoil_core :: system");
	assert_eq!(pallet.instance, Some(parse_quote! { Instance1 }));
}
