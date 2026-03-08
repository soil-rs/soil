// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::pallet::Def;

/// * implement the trait `subsoil::runtime::BuildStorage`
pub fn expand_genesis_build(def: &mut Def) -> proc_macro2::TokenStream {
	let genesis_config = if let Some(genesis_config) = &def.genesis_config {
		genesis_config
	} else {
		return Default::default();
	};
	let genesis_build = def.genesis_build.as_ref().expect("Checked by def parser");

	let topsoil_support = &def.topsoil_support;
	let type_impl_gen = &genesis_config.gen_kind.type_impl_gen(genesis_build.attr_span);
	let gen_cfg_ident = &genesis_config.genesis_config;
	let gen_cfg_use_gen = &genesis_config.gen_kind.type_use_gen(genesis_build.attr_span);

	let where_clause = &genesis_build.where_clause;

	quote::quote_spanned!(genesis_build.attr_span =>
		#topsoil_support::std_enabled! {
			impl<#type_impl_gen> #topsoil_support::subsoil::runtime::BuildStorage for #gen_cfg_ident<#gen_cfg_use_gen> #where_clause
			{
				fn assimilate_storage(&self, storage: &mut #topsoil_support::subsoil::runtime::Storage) -> std::result::Result<(), std::string::String> {
					#topsoil_support::__private::BasicExternalities::execute_with_storage(storage, || {
						self.build();
						Ok(())
					})
				}
			}
		}
	)
}
