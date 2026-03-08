// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Nft fractionalization pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use topsoil::benchmarking::prelude::*;

use fungible::{Inspect as InspectFungible, Mutate as MutateFungible};
use nonfungibles_v2::{Create, Mutate};
use topsoil::deps::topsoil_support::assert_ok;

use plant_nfts::{CollectionConfig, CollectionSettings, ItemConfig, MintSettings};
use topsoil_system::RawOrigin as SystemOrigin;

use crate::Pallet as NftFractionalization;

type BalanceOf<T> =
	<<T as Config>::Currency as InspectFungible<<T as SystemConfig>::AccountId>>::Balance;

type CollectionConfigOf<T> =
	CollectionConfig<BalanceOf<T>, BlockNumberFor<T>, <T as Config>::NftCollectionId>;

fn default_collection_config<T: Config>() -> CollectionConfigOf<T>
where
	T::Currency: InspectFungible<T::AccountId>,
{
	CollectionConfig {
		settings: CollectionSettings::all_enabled(),
		max_supply: None,
		mint_settings: MintSettings::default(),
	}
}

fn mint_nft<T: Config>(nft_id: T::NftId) -> (T::AccountId, AccountIdLookupOf<T>)
where
	T::Nfts: Create<T::AccountId, CollectionConfig<BalanceOf<T>, BlockNumberFor<T>, T::NftCollectionId>>
		+ Mutate<T::AccountId, ItemConfig>,
{
	let caller: T::AccountId = whitelisted_caller();
	let caller_lookup = T::Lookup::unlookup(caller.clone());
	let ed = T::Currency::minimum_balance();
	let multiplier = BalanceOf::<T>::from(100u8);
	T::Currency::set_balance(&caller, ed * multiplier + T::Deposit::get() * multiplier);

	assert_ok!(T::Nfts::create_collection(&caller, &caller, &default_collection_config::<T>()));
	let collection = T::BenchmarkHelper::collection(0);
	assert_ok!(T::Nfts::mint_into(&collection, &nft_id, &caller, &ItemConfig::default(), true));
	(caller, caller_lookup)
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	let events = topsoil_system::Pallet::<T>::events();
	let system_event: <T as topsoil_system::Config>::RuntimeEvent = generic_event.into();
	// compare to the last event record
	let topsoil_system::EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

#[benchmarks(
	where
		T::Nfts:
			Create<
				T::AccountId,
				CollectionConfig<BalanceOf<T>,
				topsoil_system::pallet_prelude::BlockNumberFor::<T>,
				T::NftCollectionId>
			>
			+ Mutate<T::AccountId, ItemConfig>,
)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn fractionalize() {
		let asset = T::BenchmarkHelper::asset(0);
		let collection = T::BenchmarkHelper::collection(0);
		let nft = T::BenchmarkHelper::nft(0);
		let (caller, caller_lookup) = mint_nft::<T>(nft);

		#[extrinsic_call]
		_(
			SystemOrigin::Signed(caller.clone()),
			collection,
			nft,
			asset.clone(),
			caller_lookup,
			1000u32.into(),
		);

		assert_last_event::<T>(
			Event::NftFractionalized {
				nft_collection: collection,
				nft,
				fractions: 1000u32.into(),
				asset,
				beneficiary: caller,
			}
			.into(),
		);
	}

	#[benchmark]
	fn unify() {
		let asset = T::BenchmarkHelper::asset(0);
		let collection = T::BenchmarkHelper::collection(0);
		let nft = T::BenchmarkHelper::nft(0);
		let (caller, caller_lookup) = mint_nft::<T>(nft);

		assert_ok!(NftFractionalization::<T>::fractionalize(
			SystemOrigin::Signed(caller.clone()).into(),
			collection,
			nft,
			asset.clone(),
			caller_lookup.clone(),
			1000u32.into(),
		));

		#[extrinsic_call]
		_(SystemOrigin::Signed(caller.clone()), collection, nft, asset.clone(), caller_lookup);

		assert_last_event::<T>(
			Event::NftUnified { nft_collection: collection, nft, asset, beneficiary: caller }
				.into(),
		);
	}

	impl_benchmark_test_suite!(
		NftFractionalization,
		crate::mock::new_test_ext(),
		crate::mock::Test
	);
}
