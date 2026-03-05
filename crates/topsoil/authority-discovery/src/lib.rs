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

//! # Authority discovery pallet.
//!
//! This pallet is used by the `client/authority-discovery` and by polkadot's parachain logic
//! to retrieve the current and the next set of authorities.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use topsoil_support::{
	traits::{Get, OneSessionHandler},
	WeakBoundedVec,
};
use soil_authority_discovery::AuthorityId;

pub use pallet::*;

#[topsoil_support::pallet]
pub mod pallet {
	use super::*;
	use topsoil_support::pallet_prelude::*;
	use topsoil_system::pallet_prelude::BlockNumberFor;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	/// The pallet's config trait.
	pub trait Config: topsoil_system::Config + topsoil_session::Config {
		/// The maximum number of authorities that can be added.
		type MaxAuthorities: Get<u32>;
	}

	#[pallet::storage]
	/// Keys of the current authority set.
	pub type Keys<T: Config> =
		StorageValue<_, WeakBoundedVec<AuthorityId, T::MaxAuthorities>, ValueQuery>;

	#[pallet::storage]
	/// Keys of the next authority set.
	pub type NextKeys<T: Config> =
		StorageValue<_, WeakBoundedVec<AuthorityId, T::MaxAuthorities>, ValueQuery>;

	#[derive(topsoil_support::DefaultNoBound)]
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub keys: Vec<AuthorityId>,
		#[serde(skip)]
		pub _config: core::marker::PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			Pallet::<T>::initialize_keys(&self.keys)
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		#[cfg(feature = "try-runtime")]
		fn try_state(_n: BlockNumberFor<T>) -> Result<(), soil_runtime::TryRuntimeError> {
			Self::do_try_state()
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Retrieve authority identifiers of the current and next authority set
	/// sorted and deduplicated.
	pub fn authorities() -> Vec<AuthorityId> {
		let mut keys = Keys::<T>::get().to_vec();
		let next = NextKeys::<T>::get().to_vec();

		keys.extend(next);
		keys.sort();
		keys.dedup();

		keys.to_vec()
	}

	/// Retrieve authority identifiers of the current authority set in the original order.
	pub fn current_authorities() -> WeakBoundedVec<AuthorityId, T::MaxAuthorities> {
		Keys::<T>::get()
	}

	/// Retrieve authority identifiers of the next authority set in the original order.
	pub fn next_authorities() -> WeakBoundedVec<AuthorityId, T::MaxAuthorities> {
		NextKeys::<T>::get()
	}

	fn initialize_keys(keys: &Vec<AuthorityId>) {
		if !keys.is_empty() {
			assert!(Keys::<T>::get().is_empty(), "Keys are already initialized!");

			let bounded_keys =
				WeakBoundedVec::<AuthorityId, T::MaxAuthorities>::try_from((*keys).clone())
					.expect("Keys vec too big");

			Keys::<T>::put(&bounded_keys);
			NextKeys::<T>::put(&bounded_keys);
		}
	}
}

impl<T: Config> soil_runtime::BoundToRuntimeAppPublic for Pallet<T> {
	type Public = AuthorityId;
}

impl<T: Config> OneSessionHandler<T::AccountId> for Pallet<T> {
	type Key = AuthorityId;

	fn on_genesis_session<'a, I: 'a>(authorities: I)
	where
		I: Iterator<Item = (&'a T::AccountId, Self::Key)>,
	{
		Self::initialize_keys(&authorities.map(|x| x.1).collect::<Vec<_>>());
	}

	fn on_new_session<'a, I: 'a>(changed: bool, validators: I, queued_validators: I)
	where
		I: Iterator<Item = (&'a T::AccountId, Self::Key)>,
	{
		// Remember who the authorities are for the new and next session.
		if changed {
			let keys = validators.map(|x| x.1).collect::<Vec<_>>();

			let bounded_keys = WeakBoundedVec::<_, T::MaxAuthorities>::force_from(
				keys,
				Some(
					"Warning: The session has more validators than expected. \
				A runtime configuration adjustment may be needed.",
				),
			);

			Keys::<T>::put(bounded_keys);
		}

		// `changed` represents if queued_validators changed in the previous session not in the
		// current one.
		let next_keys = queued_validators.map(|x| x.1).collect::<Vec<_>>();

		let next_bounded_keys = WeakBoundedVec::<_, T::MaxAuthorities>::force_from(
			next_keys,
			Some(
				"Warning: The session has more queued validators than expected. \
			A runtime configuration adjustment may be needed.",
			),
		);

		NextKeys::<T>::put(next_bounded_keys);
	}

	fn on_disabled(_i: u32) {
		// ignore
	}
}

#[cfg(any(feature = "try-runtime", test))]
impl<T: Config> Pallet<T> {
	/// Ensure the correctness of the state of this pallet.
	///
	/// # Invariants
	///
	/// * `Keys` must not exceed `MaxAuthorities`.
	/// * `NextKeys` must not exceed `MaxAuthorities`.
	/// * `Keys` should not contain duplicates.
	/// * `NextKeys` should not contain duplicates.
	pub fn do_try_state() -> Result<(), soil_runtime::TryRuntimeError> {
		use topsoil_support::ensure;
		let keys = Keys::<T>::get();
		ensure!(keys.len() as u32 <= T::MaxAuthorities::get(), "Keys exceeds MaxAuthorities");
		let mut sorted_keys = keys.to_vec();
		sorted_keys.sort();
		ensure!(sorted_keys.windows(2).all(|w| w[0] != w[1]), "Duplicate keys found in Keys");

		let next_keys = NextKeys::<T>::get();
		ensure!(
			next_keys.len() as u32 <= T::MaxAuthorities::get(),
			"NextKeys exceeds MaxAuthorities"
		);
		let mut sorted_next_keys = next_keys.to_vec();
		sorted_next_keys.sort();
		ensure!(
			sorted_next_keys.windows(2).all(|w| w[0] != w[1]),
			"Duplicate keys found in NextKeys"
		);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate as topsoil_authority_discovery;
	use alloc::vec;
	use topsoil_support::{derive_impl, parameter_types, traits::ConstU32};
	use soil_application_crypto::Pair;
	use soil_authority_discovery::AuthorityPair;
	use soil_core::crypto::key_types;
	use soil_io::TestExternalities;
	use soil_runtime::{
		testing::UintAuthorityId,
		traits::{ConvertInto, IdentityLookup, OpaqueKeys},
		BuildStorage, KeyTypeId, Perbill,
	};

	type Block = topsoil_system::mocking::MockBlock<Test>;

	topsoil_support::construct_runtime!(
		pub enum Test
		{
			System: topsoil_system,
			Session: topsoil_session,
			Balances: topsoil_balances,
			AuthorityDiscovery: topsoil_authority_discovery,
		}
	);

	parameter_types! {
		pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(33);
	}

	impl Config for Test {
		type MaxAuthorities = ConstU32<100>;
	}

	impl topsoil_session::Config for Test {
		type SessionManager = ();
		type Keys = UintAuthorityId;
		type ShouldEndSession = topsoil_session::PeriodicSessions<Period, Offset>;
		type SessionHandler = TestSessionHandler;
		type RuntimeEvent = RuntimeEvent;
		type ValidatorId = AuthorityId;
		type ValidatorIdOf = ConvertInto;
		type NextSessionRotation = topsoil_session::PeriodicSessions<Period, Offset>;
		type DisablingStrategy = ();
		type WeightInfo = ();
		type Currency = Balances;
		type KeyDeposit = ();
	}

	pub type BlockNumber = u64;

	parameter_types! {
		pub const Period: BlockNumber = 1;
		pub const Offset: BlockNumber = 0;
	}

	#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
	impl topsoil_system::Config for Test {
		type AccountId = AuthorityId;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Block = Block;
		type AccountData = topsoil_balances::AccountData<u64>;
	}

	#[derive_impl(topsoil_balances::config_preludes::TestDefaultConfig)]
	impl topsoil_balances::Config for Test {
		type AccountStore = System;
	}

	pub struct TestSessionHandler;
	impl topsoil_session::SessionHandler<AuthorityId> for TestSessionHandler {
		const KEY_TYPE_IDS: &'static [KeyTypeId] = &[key_types::DUMMY];

		fn on_new_session<Ks: OpaqueKeys>(
			_changed: bool,
			_validators: &[(AuthorityId, Ks)],
			_queued_validators: &[(AuthorityId, Ks)],
		) {
		}

		fn on_disabled(_validator_index: u32) {}

		fn on_genesis_session<Ks: OpaqueKeys>(_validators: &[(AuthorityId, Ks)]) {}
	}

	#[test]
	fn authorities_returns_current_and_next_authority_set() {
		// The whole authority discovery pallet ignores account ids, but we still need them for
		// `topsoil_session::OneSessionHandler::on_new_session`, thus its safe to use the same value
		// everywhere.
		let account_id = AuthorityPair::from_seed_slice(vec![10; 32].as_ref()).unwrap().public();

		let mut first_authorities: Vec<AuthorityId> = vec![0, 1]
			.into_iter()
			.map(|i| AuthorityPair::from_seed_slice(vec![i; 32].as_ref()).unwrap().public())
			.map(AuthorityId::from)
			.collect();

		let second_authorities: Vec<AuthorityId> = vec![2, 3]
			.into_iter()
			.map(|i| AuthorityPair::from_seed_slice(vec![i; 32].as_ref()).unwrap().public())
			.map(AuthorityId::from)
			.collect();
		// Needed for `topsoil_session::OneSessionHandler::on_new_session`.
		let second_authorities_and_account_ids = second_authorities
			.clone()
			.into_iter()
			.map(|id| (&account_id, id))
			.collect::<Vec<(&AuthorityId, AuthorityId)>>();

		let third_authorities: Vec<AuthorityId> = vec![4, 5]
			.into_iter()
			.map(|i| AuthorityPair::from_seed_slice(vec![i; 32].as_ref()).unwrap().public())
			.map(AuthorityId::from)
			.collect();
		// Needed for `topsoil_session::OneSessionHandler::on_new_session`.
		let third_authorities_and_account_ids = third_authorities
			.clone()
			.into_iter()
			.map(|id| (&account_id, id))
			.collect::<Vec<(&AuthorityId, AuthorityId)>>();

		let mut fourth_authorities: Vec<AuthorityId> = vec![6, 7]
			.into_iter()
			.map(|i| AuthorityPair::from_seed_slice(vec![i; 32].as_ref()).unwrap().public())
			.map(AuthorityId::from)
			.collect();
		// Needed for `topsoil_session::OneSessionHandler::on_new_session`.
		let fourth_authorities_and_account_ids = fourth_authorities
			.clone()
			.into_iter()
			.map(|id| (&account_id, id))
			.collect::<Vec<(&AuthorityId, AuthorityId)>>();

		// Build genesis.
		let mut t = topsoil_system::GenesisConfig::<Test>::default().build_storage().unwrap();

		topsoil_authority_discovery::GenesisConfig::<Test> { keys: vec![], ..Default::default() }
			.assimilate_storage(&mut t)
			.unwrap();

		// Create externalities.
		let mut externalities = TestExternalities::new(t);

		externalities.execute_with(|| {
			use topsoil_support::traits::OneSessionHandler;

			AuthorityDiscovery::on_genesis_session(
				first_authorities.iter().map(|id| (id, id.clone())),
			);
			first_authorities.sort();
			let mut authorities_returned = AuthorityDiscovery::authorities();
			authorities_returned.sort();
			assert_eq!(first_authorities, authorities_returned);

			// Verify state
			AuthorityDiscovery::do_try_state().unwrap();

			// When `changed` set to false, the authority set should not be updated.
			AuthorityDiscovery::on_new_session(
				false,
				second_authorities_and_account_ids.clone().into_iter(),
				third_authorities_and_account_ids.clone().into_iter(),
			);
			let authorities_returned = AuthorityDiscovery::authorities();
			let mut first_and_third_authorities = first_authorities
				.iter()
				.chain(third_authorities.iter())
				.cloned()
				.collect::<Vec<AuthorityId>>();
			first_and_third_authorities.sort();

			assert_eq!(
				first_and_third_authorities, authorities_returned,
				"Expected authority set not to change as `changed` was set to false.",
			);

			// Verify state
			AuthorityDiscovery::do_try_state().unwrap();

			// When `changed` set to true, the authority set should be updated.
			AuthorityDiscovery::on_new_session(
				true,
				third_authorities_and_account_ids.into_iter(),
				fourth_authorities_and_account_ids.clone().into_iter(),
			);

			let mut third_and_fourth_authorities = third_authorities
				.iter()
				.chain(fourth_authorities.iter())
				.cloned()
				.collect::<Vec<AuthorityId>>();
			third_and_fourth_authorities.sort();
			assert_eq!(
				third_and_fourth_authorities,
				AuthorityDiscovery::authorities(),
				"Expected authority set to contain both the authorities of the new as well as the \
				 next session."
			);

			// Verify state
			AuthorityDiscovery::do_try_state().unwrap();

			// With overlapping authority sets, `authorities()` should return a deduplicated set.
			AuthorityDiscovery::on_new_session(
				true,
				fourth_authorities_and_account_ids.clone().into_iter(),
				fourth_authorities_and_account_ids.clone().into_iter(),
			);
			fourth_authorities.sort();
			assert_eq!(
				fourth_authorities,
				AuthorityDiscovery::authorities(),
				"Expected authority set to be deduplicated."
			);

			// Verify state
			AuthorityDiscovery::do_try_state().unwrap();
		});
	}
}
