// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg_attr(not(feature = "std"), no_std)]

use topsoil_support::{derive_impl, traits::ConstU32};

pub use pallet::*;

#[topsoil_support::pallet(dev_mode)]
pub mod pallet {
	use topsoil_support::pallet_prelude::*;
	use topsoil_system::pallet_prelude::*;

	// The struct on which we build all of our Pallet logic.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// Your Pallet's configuration trait, representing custom external types and interfaces.
	#[pallet::config]
	pub trait Config: topsoil_system::Config {}

	// The MEL requirement for bounded pallets is skipped by `dev_mode`.
	#[pallet::storage]
	type MyStorage<T: Config> = StorageValue<_, Vec<u8>>;

	// The Hasher requirement skipped by `dev_mode`.
	#[pallet::storage]
	pub type MyStorageMap<T: Config> = StorageMap<_, _, u32, u64>;

	#[pallet::storage]
	type MyStorageDoubleMap<T: Config> = StorageDoubleMap<_, _, u32, _, u64, u64>;

	#[pallet::storage]
	type MyCountedStorageMap<T: Config> = CountedStorageMap<_, _, u32, u64>;

	#[pallet::storage]
	pub type MyStorageMap2<T: Config> = StorageMap<Key = u32, Value = u64>;

	#[pallet::storage]
	type MyStorageDoubleMap2<T: Config> = StorageDoubleMap<Key1 = u32, Key2 = u64, Value = u64>;

	#[pallet::storage]
	type MyCountedStorageMap2<T: Config> = CountedStorageMap<Key = u32, Value = u64>;

	// Your Pallet's callable functions.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// No need to define a `weight` attribute here because of `dev_mode`.
		pub fn my_call(_origin: OriginFor<T>) -> DispatchResult {
			Ok(())
		}
	}

	// Your Pallet's internal functions.
	impl<T: Config> Pallet<T> {}
}

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Runtime {
	type BaseCallFilter = topsoil_support::traits::Everything;
	type RuntimeOrigin = RuntimeOrigin;
	type Nonce = u64;
	type RuntimeCall = RuntimeCall;
	type Hash = subsoil::runtime::testing::H256;
	type Hashing = subsoil::runtime::traits::BlakeTwo256;
	type AccountId = u64;
	type Lookup = subsoil::runtime::traits::IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

pub type Header = subsoil::runtime::generic::Header<u32, subsoil::runtime::traits::BlakeTwo256>;
pub type Block = subsoil::runtime::generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic = subsoil::runtime::generic::UncheckedExtrinsic<u32, RuntimeCall, (), ()>;

topsoil_support::construct_runtime!(
	pub struct Runtime
	{
		// Exclude part `Storage` in order not to check its metadata in tests.
		System: topsoil_system exclude_parts { Pallet, Storage },
		Example: pallet,
	}
);

impl pallet::Config for Runtime {}

fn main() {
	use topsoil_support::pallet_prelude::*;
	use subsoil::io::{
		hashing::{blake2_128, twox_128},
		TestExternalities,
	};
	use storage::unhashed;

	fn blake2_128_concat(d: &[u8]) -> Vec<u8> {
		let mut v = blake2_128(d).to_vec();
		v.extend_from_slice(d);
		v
	}

	TestExternalities::default().execute_with(|| {
		pallet::MyStorageMap::<Runtime>::insert(1, 2);
		let mut k = [twox_128(b"Example"), twox_128(b"MyStorageMap")].concat();
		k.extend(1u32.using_encoded(blake2_128_concat));
		assert_eq!(unhashed::get::<u64>(&k), Some(2u64));
	});
}
