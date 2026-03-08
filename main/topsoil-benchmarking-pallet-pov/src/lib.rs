// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! End-to-end testing pallet for PoV benchmarking. Should only be deployed in a  testing runtime.

#![cfg_attr(not(feature = "std"), no_std)]

mod benchmarking;
mod tests;
mod weights;

extern crate alloc;

pub use pallet::*;

#[topsoil_support::pallet]
pub mod pallet {
	use alloc::vec::Vec;
	use topsoil_support::pallet_prelude::*;
	use topsoil_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: topsoil_system::Config {
		#[allow(deprecated)]
		type RuntimeEvent: From<Event<Self>>
			+ IsType<<Self as topsoil_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	pub(crate) type Value<T: Config> = StorageValue<Value = u32, QueryKind = OptionQuery>;

	#[pallet::storage]
	pub(crate) type Value2<T: Config> = StorageValue<Value = u32, QueryKind = OptionQuery>;

	/// A value without a MEL bound.
	#[pallet::storage]
	#[pallet::unbounded]
	pub(crate) type UnboundedValue<T: Config> =
		StorageValue<Value = Vec<u8>, QueryKind = OptionQuery>;

	/// A value with a MEL bound of 32 byte.
	#[pallet::storage]
	pub(crate) type BoundedValue<T: Config> =
		StorageValue<Value = BoundedVec<u8, ConstU32<32>>, QueryKind = OptionQuery>;

	/// 4MiB value.
	#[pallet::storage]
	pub(crate) type LargeValue<T: Config> =
		StorageValue<Value = BoundedVec<u8, ConstU32<{ 1 << 22 }>>, QueryKind = OptionQuery>;

	#[pallet::storage]
	pub(crate) type LargeValue2<T: Config> =
		StorageValue<Value = BoundedVec<u8, ConstU32<{ 1 << 22 }>>, QueryKind = OptionQuery>;

	/// A map with a maximum of 1M entries.
	#[pallet::storage]
	pub(crate) type Map1M<T: Config> = StorageMap<
		Hasher = Blake2_256,
		Key = u32,
		Value = u32,
		QueryKind = OptionQuery,
		MaxValues = ConstU32<1_000_000>,
	>;

	/// A map with a maximum of 16M entries.
	#[pallet::storage]
	pub(crate) type Map16M<T: Config> = StorageMap<
		Hasher = Blake2_256,
		Key = u32,
		Value = u32,
		QueryKind = OptionQuery,
		MaxValues = ConstU32<16_000_000>,
	>;

	#[pallet::storage]
	pub(crate) type DoubleMap1M<T: Config> = StorageDoubleMap<
		Hasher1 = Blake2_256,
		Hasher2 = Blake2_256,
		Key1 = u32,
		Key2 = u32,
		Value = u32,
		QueryKind = OptionQuery,
		MaxValues = ConstU32<1_000_000>,
	>;

	#[pallet::storage]
	#[pallet::unbounded]
	pub(crate) type UnboundedMap<T: Config> =
		StorageMap<Hasher = Blake2_256, Key = u32, Value = Vec<u32>, QueryKind = OptionQuery>;

	#[pallet::storage]
	#[pallet::unbounded]
	pub(crate) type UnboundedMap2<T: Config> =
		StorageMap<Hasher = Blake2_256, Key = u32, Value = Vec<u32>, QueryKind = OptionQuery>;

	#[pallet::storage]
	#[pallet::unbounded]
	pub(crate) type UnboundedMapTwox<T: Config> =
		StorageMap<Hasher = Twox64Concat, Key = u32, Value = Vec<u32>, QueryKind = OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		TestEvent,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight({0})]
		pub fn emit_event(_origin: OriginFor<T>) -> DispatchResult {
			Self::deposit_event(Event::TestEvent);
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight({0})]
		pub fn noop(_origin: OriginFor<T>) -> DispatchResult {
			Ok(())
		}
	}
}
