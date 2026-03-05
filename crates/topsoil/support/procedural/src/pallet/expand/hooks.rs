// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::pallet::Def;

/// * implement the individual traits using the Hooks trait
pub fn expand_hooks(def: &mut Def) -> proc_macro2::TokenStream {
	let (where_clause, span, has_runtime_upgrade) = match def.hooks.as_ref() {
		Some(hooks) => {
			let where_clause = hooks.where_clause.clone();
			let span = hooks.attr_span;
			let has_runtime_upgrade = hooks.has_runtime_upgrade;
			(where_clause, span, has_runtime_upgrade)
		},
		None => (def.config.where_clause.clone(), def.pallet_struct.attr_span, false),
	};

	let topsoil_support = &def.topsoil_support;
	let type_impl_gen = &def.type_impl_generics(span);
	let type_use_gen = &def.type_use_generics(span);
	let pallet_ident = &def.pallet_struct.pallet;
	let topsoil_system = &def.topsoil_system;
	let pallet_name = quote::quote! {
		<
			<T as #topsoil_system::Config>::PalletInfo
			as
			#topsoil_support::traits::PalletInfo
		>::name::<Self>().unwrap_or("<unknown pallet name>")
	};

	let initialize_on_chain_storage_version = if let Some(in_code_version) =
		&def.pallet_struct.storage_version
	{
		quote::quote! {
			#topsoil_support::__private::log::info!(
				target: #topsoil_support::LOG_TARGET,
				"🐥 New pallet {:?} detected in the runtime. Initializing the on-chain storage version to match the storage version defined in the pallet: {:?}",
				#pallet_name,
				#in_code_version
			);
			#in_code_version.put::<Self>();
		}
	} else {
		quote::quote! {
			let default_version = #topsoil_support::traits::StorageVersion::new(0);
			#topsoil_support::__private::log::info!(
				target: #topsoil_support::LOG_TARGET,
				"🐥 New pallet {:?} detected in the runtime. The pallet has no defined storage version, so the on-chain version is being initialized to {:?}.",
				#pallet_name,
				default_version
			);
			default_version.put::<Self>();
		}
	};

	let log_runtime_upgrade = if has_runtime_upgrade {
		// a migration is defined here.
		quote::quote! {
			#topsoil_support::__private::log::info!(
				target: #topsoil_support::LOG_TARGET,
				"⚠️ {} declares internal migrations (which *might* execute). \
				 On-chain `{:?}` vs in-code storage version `{:?}`",
				#pallet_name,
				<Self as #topsoil_support::traits::GetStorageVersion>::on_chain_storage_version(),
				<Self as #topsoil_support::traits::GetStorageVersion>::in_code_storage_version(),
			);
		}
	} else {
		// default.
		quote::quote! {
			#topsoil_support::__private::log::debug!(
				target: #topsoil_support::LOG_TARGET,
				"✅ no migration for {}",
				#pallet_name,
			);
		}
	};

	let hooks_impl = if def.hooks.is_none() {
		let topsoil_system = &def.topsoil_system;
		quote::quote! {
			impl<#type_impl_gen>
				#topsoil_support::traits::Hooks<#topsoil_system::pallet_prelude::BlockNumberFor::<T>>
				for #pallet_ident<#type_use_gen> #where_clause {}
		}
	} else {
		proc_macro2::TokenStream::new()
	};

	// If a storage version is set, we should ensure that the storage version on chain matches the
	// in-code storage version. This assumes that `Executive` is running custom migrations before
	// the pallets are called.
	let post_storage_version_check = if def.pallet_struct.storage_version.is_some() {
		quote::quote! {
			let on_chain_version = <Self as #topsoil_support::traits::GetStorageVersion>::on_chain_storage_version();
			let in_code_version = <Self as #topsoil_support::traits::GetStorageVersion>::in_code_storage_version();

			if on_chain_version != in_code_version {
				#topsoil_support::__private::log::error!(
					target: #topsoil_support::LOG_TARGET,
					"{}: On chain storage version {:?} doesn't match in-code storage version {:?}.",
					#pallet_name,
					on_chain_version,
					in_code_version,
				);

				return Err("On chain and in-code storage version do not match. Missing runtime upgrade?".into());
			}
		}
	} else {
		quote::quote! {
			let on_chain_version = <Self as #topsoil_support::traits::GetStorageVersion>::on_chain_storage_version();

			if on_chain_version != #topsoil_support::traits::StorageVersion::new(0) {
				#topsoil_support::__private::log::error!(
					target: #topsoil_support::LOG_TARGET,
					"{}: On chain storage version {:?} is set to non zero, \
					 while the pallet is missing the `#[pallet::storage_version(VERSION)]` attribute.",
					#pallet_name,
					on_chain_version,
				);

				return Err("On chain storage version set, while the pallet doesn't \
							have the `#[pallet::storage_version(VERSION)]` attribute.".into());
			}
		}
	};

	quote::quote_spanned!(span =>
		#hooks_impl

		impl<#type_impl_gen>
			#topsoil_support::traits::OnFinalize<#topsoil_system::pallet_prelude::BlockNumberFor::<T>>
			for #pallet_ident<#type_use_gen> #where_clause
		{
			fn on_finalize(n: #topsoil_system::pallet_prelude::BlockNumberFor::<T>) {
				#topsoil_support::__private::subsoil::enter_span!(
					#topsoil_support::__private::subsoil::tracing::trace_span!("on_finalize")
				);
				<
					Self as #topsoil_support::traits::Hooks<
						#topsoil_system::pallet_prelude::BlockNumberFor::<T>
					>
				>::on_finalize(n)
			}
		}

		impl<#type_impl_gen>
			#topsoil_support::traits::OnIdle<#topsoil_system::pallet_prelude::BlockNumberFor::<T>>
			for #pallet_ident<#type_use_gen> #where_clause
		{
			fn on_idle(
				n: #topsoil_system::pallet_prelude::BlockNumberFor::<T>,
				remaining_weight: #topsoil_support::weights::Weight
			) -> #topsoil_support::weights::Weight {
				<
					Self as #topsoil_support::traits::Hooks<
						#topsoil_system::pallet_prelude::BlockNumberFor::<T>
					>
				>::on_idle(n, remaining_weight)
			}
		}

		impl<#type_impl_gen>
			#topsoil_support::traits::OnPoll<#topsoil_system::pallet_prelude::BlockNumberFor::<T>>
			for #pallet_ident<#type_use_gen> #where_clause
		{
			fn on_poll(
				n: #topsoil_system::pallet_prelude::BlockNumberFor::<T>,
				weight: &mut #topsoil_support::weights::WeightMeter
			) {
				<
					Self as #topsoil_support::traits::Hooks<
						#topsoil_system::pallet_prelude::BlockNumberFor::<T>
					>
				>::on_poll(n, weight);
			}
		}

		impl<#type_impl_gen>
			#topsoil_support::traits::OnInitialize<#topsoil_system::pallet_prelude::BlockNumberFor::<T>>
			for #pallet_ident<#type_use_gen> #where_clause
		{
			fn on_initialize(
				n: #topsoil_system::pallet_prelude::BlockNumberFor::<T>
			) -> #topsoil_support::weights::Weight {
				#topsoil_support::__private::subsoil::enter_span!(
					#topsoil_support::__private::subsoil::tracing::trace_span!("on_initialize")
				);
				<
					Self as #topsoil_support::traits::Hooks<
						#topsoil_system::pallet_prelude::BlockNumberFor::<T>
					>
				>::on_initialize(n)
			}
		}

		impl<#type_impl_gen>
			#topsoil_support::traits::BeforeAllRuntimeMigrations
			for #pallet_ident<#type_use_gen> #where_clause
		{
			fn before_all_runtime_migrations() -> #topsoil_support::weights::Weight {
				use #topsoil_support::traits::{Get, PalletInfoAccess};
				use #topsoil_support::__private::hashing::twox_128;
				use #topsoil_support::storage::unhashed::contains_prefixed_key;
				#topsoil_support::__private::subsoil::enter_span!(
					#topsoil_support::__private::subsoil::tracing::trace_span!("before_all")
				);

				// Check if the pallet has any keys set, including the storage version. If there are
				// no keys set, the pallet was just added to the runtime and needs to have its
				// version initialized.
				let pallet_hashed_prefix = <Self as PalletInfoAccess>::name_hash();
				let exists = contains_prefixed_key(&pallet_hashed_prefix);
				if !exists {
					#initialize_on_chain_storage_version
					<T as #topsoil_system::Config>::DbWeight::get().reads_writes(1, 1)
				} else {
					<T as #topsoil_system::Config>::DbWeight::get().reads(1)
				}
			}
		}

		impl<#type_impl_gen>
			#topsoil_support::traits::OnRuntimeUpgrade
			for #pallet_ident<#type_use_gen> #where_clause
		{
			fn on_runtime_upgrade() -> #topsoil_support::weights::Weight {
				#topsoil_support::__private::subsoil::enter_span!(
					#topsoil_support::__private::subsoil::tracing::trace_span!("on_runtime_update")
				);

				// log info about the upgrade.
				#log_runtime_upgrade

				<
					Self as #topsoil_support::traits::Hooks<
						#topsoil_system::pallet_prelude::BlockNumberFor::<T>
					>
				>::on_runtime_upgrade()
			}

			#topsoil_support::try_runtime_enabled! {
				fn pre_upgrade() -> Result<#topsoil_support::__private::Vec<u8>, #topsoil_support::soil_runtime::TryRuntimeError> {
					<
						Self
						as
						#topsoil_support::traits::Hooks<#topsoil_system::pallet_prelude::BlockNumberFor::<T>>
					>::pre_upgrade()
				}

				fn post_upgrade(state: #topsoil_support::__private::Vec<u8>) -> Result<(), #topsoil_support::soil_runtime::TryRuntimeError> {
					#post_storage_version_check

					<
						Self
						as
						#topsoil_support::traits::Hooks<#topsoil_system::pallet_prelude::BlockNumberFor::<T>>
					>::post_upgrade(state)
				}
			}
		}

		impl<#type_impl_gen>
			#topsoil_support::traits::OffchainWorker<#topsoil_system::pallet_prelude::BlockNumberFor::<T>>
			for #pallet_ident<#type_use_gen> #where_clause
		{
			fn offchain_worker(n: #topsoil_system::pallet_prelude::BlockNumberFor::<T>) {
				<
					Self as #topsoil_support::traits::Hooks<
						#topsoil_system::pallet_prelude::BlockNumberFor::<T>
					>
				>::offchain_worker(n)
			}
		}

		// Integrity tests are only required for when `std` is enabled.
		#topsoil_support::std_enabled! {
			impl<#type_impl_gen>
				#topsoil_support::traits::IntegrityTest
			for #pallet_ident<#type_use_gen> #where_clause
			{
				fn integrity_test() {
					#topsoil_support::__private::subsoil::io::TestExternalities::default().execute_with(|| {
						<
							Self as #topsoil_support::traits::Hooks<
								#topsoil_system::pallet_prelude::BlockNumberFor::<T>
							>
						>::integrity_test()
					});
				}
			}
		}

		#topsoil_support::try_runtime_enabled! {
			impl<#type_impl_gen>
				#topsoil_support::traits::TryState<#topsoil_system::pallet_prelude::BlockNumberFor::<T>>
				for #pallet_ident<#type_use_gen> #where_clause
			{
				fn try_state(
					n: #topsoil_system::pallet_prelude::BlockNumberFor::<T>,
					_s: #topsoil_support::traits::TryStateSelect
				) -> Result<(), #topsoil_support::soil_runtime::TryRuntimeError> {
					#topsoil_support::__private::log::info!(
						target: #topsoil_support::LOG_TARGET,
						"🩺 Running {:?} try-state checks",
						#pallet_name,
					);
					<
						Self as #topsoil_support::traits::Hooks<
							#topsoil_system::pallet_prelude::BlockNumberFor::<T>
						>
					>::try_state(n).inspect_err(|err| {
						#topsoil_support::__private::log::error!(
							target: #topsoil_support::LOG_TARGET,
							"❌ {:?} try_state checks failed: {:?}",
							#pallet_name,
							err
						);
					})
				}
			}
		}
	)
}
