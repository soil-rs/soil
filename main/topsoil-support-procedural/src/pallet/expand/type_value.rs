// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::pallet::Def;

/// * Generate the struct
/// * implement the `Get<..>` on it
/// * Rename the name of the function to internal name
pub fn expand_type_values(def: &mut Def) -> proc_macro2::TokenStream {
	let mut expand = quote::quote!();
	let topsoil_support = &def.topsoil_support;

	for type_value in &def.type_values {
		let fn_name_str = &type_value.ident.to_string();
		let fn_name_snakecase = inflector::cases::snakecase::to_snake_case(fn_name_str);
		let fn_ident_renamed = syn::Ident::new(
			&format!("__type_value_for_{}", fn_name_snakecase),
			type_value.ident.span(),
		);

		let type_value_item = {
			let item = &mut def.item.content.as_mut().expect("Checked by def").1[type_value.index];
			if let syn::Item::Fn(item) = item {
				item
			} else {
				unreachable!("Checked by error parser")
			}
		};

		// Rename the type_value function name
		type_value_item.sig.ident = fn_ident_renamed.clone();

		let vis = &type_value.vis;
		let ident = &type_value.ident;
		let type_ = &type_value.type_;
		let where_clause = &type_value.where_clause;

		let (struct_impl_gen, struct_use_gen) = if type_value.is_generic {
			(
				def.type_impl_generics(type_value.attr_span),
				def.type_use_generics(type_value.attr_span),
			)
		} else {
			(Default::default(), Default::default())
		};

		let docs = &type_value.docs;

		expand.extend(quote::quote_spanned!(type_value.attr_span =>
			#( #[doc = #docs] )*
			#vis struct #ident<#struct_use_gen>(core::marker::PhantomData<((), #struct_use_gen)>);
			impl<#struct_impl_gen> #topsoil_support::traits::Get<#type_> for #ident<#struct_use_gen>
			#where_clause
			{
				fn get() -> #type_ {
					#fn_ident_renamed::<#struct_use_gen>()
				}
			}
		));
	}
	expand
}
