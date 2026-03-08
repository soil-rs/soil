// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![allow(useless_deprecated)]

use std::collections::BTreeMap;

use scale_info::TypeInfo;
use topsoil_support::{
	derive_impl,
	dispatch::Parameter,
	parameter_types,
	traits::{ConstU32, StorageVersion},
	OrdNoBound, PartialOrdNoBound,
};

parameter_types! {
	/// Used to control if the storage version should be updated.
	storage UpdateStorageVersion: bool = false;
}

pub struct SomeType1;
impl From<SomeType1> for u64 {
	fn from(_t: SomeType1) -> Self {
		0u64
	}
}

pub trait SomeAssociation1 {
	type _1: Parameter + codec::MaxEncodedLen + TypeInfo;
}
impl SomeAssociation1 for u64 {
	type _1 = u64;
}

#[topsoil_support::pallet]
pub mod pallet {
	use super::*;
	use topsoil_support::pallet_prelude::*;

	pub(crate) const STORAGE_VERSION: StorageVersion = StorageVersion::new(10);

	#[pallet::config]
	pub trait Config: topsoil_system::Config
	where
		<Self as topsoil_system::Config>::AccountId: From<SomeType1> + SomeAssociation1,
	{
		type Balance: Parameter + Default + TypeInfo;

		#[allow(deprecated)]
		type RuntimeEvent: From<Event<Self>>
			+ IsType<<Self as topsoil_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::error]
	#[derive(PartialEq, Eq)]
	pub enum Error<T> {
		/// error doc comment put in metadata
		InsufficientProposersBalance,
		NonExistentStorageValue,
		Code(u8),
		#[codec(skip)]
		Skipped(u128),
		CompactU8(#[codec(compact)] u8),
	}

	#[pallet::event]
	pub enum Event<T: Config>
	where
		T::AccountId: SomeAssociation1 + From<SomeType1>,
	{
		#[deprecated = "second"]
		#[codec(index = 1)]
		A,
		#[deprecated = "first"]
		#[codec(index = 0)]
		B,
	}

	#[pallet::origin]
	#[derive(
		EqNoBound,
		DebugNoBound,
		CloneNoBound,
		PartialEqNoBound,
		PartialOrdNoBound,
		OrdNoBound,
		Encode,
		Decode,
		DecodeWithMemTracking,
		TypeInfo,
		MaxEncodedLen,
	)]
	pub struct Origin<T>(PhantomData<T>);
}

topsoil_support::parameter_types!(
	pub const MyGetParam3: u32 = 12;
);

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
	type MaxConsumers = ConstU32<16>;
}
impl pallet::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u64;
}

pub type Header = subsoil::runtime::generic::Header<u32, subsoil::runtime::traits::BlakeTwo256>;
pub type Block = subsoil::runtime::generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic = subsoil::runtime::generic::UncheckedExtrinsic<
	u64,
	RuntimeCall,
	subsoil::runtime::testing::UintAuthorityId,
	topsoil_system::CheckNonZeroSender<Runtime>,
>;

topsoil_support::construct_runtime!(
	pub struct Runtime {
		// Exclude part `Storage` in order not to check its metadata in tests.
		System: topsoil_system exclude_parts { Pallet, Storage },
		Example: pallet,

	}
);

#[test]
fn pallet_metadata() {
	use subsoil::metadata_ir::{EnumDeprecationInfoIR, VariantDeprecationInfoIR};
	let pallets = Runtime::metadata_ir().pallets;
	let example = pallets[0].clone();
	{
		// Example pallet events are partially and fully deprecated
		let meta = example.event.unwrap();
		assert_eq!(
			EnumDeprecationInfoIR(BTreeMap::from([
				(0, VariantDeprecationInfoIR::Deprecated { note: "first", since: None }),
				(1, VariantDeprecationInfoIR::Deprecated { note: "second", since: None })
			])),
			meta.deprecation_info
		);
	}
}
