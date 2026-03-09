// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use syn::{Ident, PathArguments};

/// The declaration of a pallet.
#[derive(Debug, Clone)]
pub struct PalletDeclaration {
	/// The name of the pallet, e.g.`System` in `pub type System = topsoil_system`.
	pub name: Ident,
	/// The path of the pallet, e.g. `topsoil_system` in `pub type System = topsoil_system`.
	pub path: syn::Path,
	/// The segment of the pallet, e.g. `Pallet` in `pub type System = topsoil_core::system::Pallet`.
	pub pallet_segment: Option<syn::PathSegment>,
	/// The runtime parameter of the pallet, e.g. `Runtime` in
	/// `pub type System = topsoil_core::system::Pallet<Runtime>`.
	pub runtime_param: Option<Ident>,
	/// The instance of the pallet, e.g. `Instance1` in `pub type Council =
	/// plant_collective<Instance1>`.
	pub instance: Option<Ident>,
}

impl PalletDeclaration {
	pub fn try_from(
		_attr_span: proc_macro2::Span,
		item: &syn::ItemType,
		path: &syn::Path,
	) -> syn::Result<Self> {
		let name = item.ident.clone();

		let mut path = path.clone();

		let mut pallet_segment = None;
		let mut runtime_param = None;
		let mut instance = None;
		if let Some(segment) = path.segments.iter_mut().find(|seg| !seg.arguments.is_empty()) {
			if let PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
				args, ..
			}) = segment.arguments.clone()
			{
				if segment.ident == "Pallet" {
					let mut segment = segment.clone();
					segment.arguments = PathArguments::None;
					pallet_segment = Some(segment.clone());
				}
				let mut args_iter = args.iter();
				if let Some(syn::GenericArgument::Type(syn::Type::Path(arg_path))) =
					args_iter.next()
				{
					let ident = arg_path.path.require_ident()?.clone();
					if segment.ident == "Pallet" {
						runtime_param = Some(ident);
						if let Some(syn::GenericArgument::Type(syn::Type::Path(arg_path))) =
							args_iter.next()
						{
							instance = Some(arg_path.path.require_ident()?.clone());
						}
					} else {
						instance = Some(ident);
						segment.arguments = PathArguments::None;
					}
				}
			}
		}

		if pallet_segment.is_some() {
			path = syn::Path {
				leading_colon: None,
				segments: path
					.segments
					.iter()
					.filter(|seg| seg.arguments.is_empty())
					.cloned()
					.collect(),
			};
		}

		Ok(Self { name, path, pallet_segment, runtime_param, instance })
	}
}

#[test]
fn declaration_works() {
	use syn::parse_quote;

	let decl: PalletDeclaration = PalletDeclaration::try_from(
		proc_macro2::Span::call_site(),
		&parse_quote! { pub type System = topsoil_system; },
		&parse_quote! { topsoil_system },
	)
	.expect("Failed to parse pallet declaration");

	assert_eq!(decl.name, "System");
	assert_eq!(decl.path, parse_quote! { topsoil_system });
	assert_eq!(decl.pallet_segment, None);
	assert_eq!(decl.runtime_param, None);
	assert_eq!(decl.instance, None);
}

#[test]
fn declaration_works_with_instance() {
	use syn::parse_quote;

	let decl: PalletDeclaration = PalletDeclaration::try_from(
		proc_macro2::Span::call_site(),
		&parse_quote! { pub type System = topsoil_system<Instance1>; },
		&parse_quote! { topsoil_system<Instance1> },
	)
	.expect("Failed to parse pallet declaration");

	assert_eq!(decl.name, "System");
	assert_eq!(decl.path, parse_quote! { topsoil_system });
	assert_eq!(decl.pallet_segment, None);
	assert_eq!(decl.runtime_param, None);
	assert_eq!(decl.instance, Some(parse_quote! { Instance1 }));
}

#[test]
fn declaration_works_with_pallet() {
	use syn::parse_quote;

	let decl: PalletDeclaration = PalletDeclaration::try_from(
		proc_macro2::Span::call_site(),
		&parse_quote! { pub type System = topsoil_core::system::Pallet<Runtime>; },
		&parse_quote! { topsoil_core::system::Pallet<Runtime> },
	)
	.expect("Failed to parse pallet declaration");

	assert_eq!(decl.name, "System");
	assert_eq!(decl.path, parse_quote! { topsoil_system });

	let segment: syn::PathSegment =
		syn::PathSegment { ident: parse_quote! { Pallet }, arguments: PathArguments::None };
	assert_eq!(decl.pallet_segment, Some(segment));
	assert_eq!(decl.runtime_param, Some(parse_quote! { Runtime }));
	assert_eq!(decl.instance, None);
}

#[test]
fn declaration_works_with_pallet_and_instance() {
	use syn::parse_quote;

	let decl: PalletDeclaration = PalletDeclaration::try_from(
		proc_macro2::Span::call_site(),
		&parse_quote! { pub type System = topsoil_core::system::Pallet<Runtime, Instance1>; },
		&parse_quote! { topsoil_core::system::Pallet<Runtime, Instance1> },
	)
	.expect("Failed to parse pallet declaration");

	assert_eq!(decl.name, "System");
	assert_eq!(decl.path, parse_quote! { topsoil_system });

	let segment: syn::PathSegment =
		syn::PathSegment { ident: parse_quote! { Pallet }, arguments: PathArguments::None };
	assert_eq!(decl.pallet_segment, Some(segment));
	assert_eq!(decl.runtime_param, Some(parse_quote! { Runtime }));
	assert_eq!(decl.instance, Some(parse_quote! { Instance1 }));
}
