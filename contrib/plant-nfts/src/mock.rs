// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test environment for Nfts pallet.

use super::*;
use crate as plant_nfts;

use subsoil::keystore::{testing::MemoryKeystore, KeystoreExt};
use subsoil::runtime::{
	traits::{IdentifyAccount, IdentityLookup, Verify},
	BuildStorage, MultiSignature,
};
use topsoil_support::{
	construct_runtime, derive_impl, parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU32, ConstU64},
};

type Block = topsoil_system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test
	{
		System: topsoil_system,
		Balances: plant_balances,
		Nfts: plant_nfts,
	}
);

pub type Signature = MultiSignature;
pub type AccountPublic = <Signature as Verify>::Signer;
pub type AccountId = <AccountPublic as IdentifyAccount>::AccountId;

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Test {
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type AccountData = plant_balances::AccountData<u64>;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Test {
	type AccountStore = System;
}

parameter_types! {
	pub storage Features: PalletFeatures = PalletFeatures::all_enabled();
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<topsoil_system::EnsureSigned<Self::AccountId>>;
	type ForceOrigin = topsoil_system::EnsureRoot<Self::AccountId>;
	type Locker = ();
	type CollectionDeposit = ConstU64<2>;
	type ItemDeposit = ConstU64<1>;
	type MetadataDepositBase = ConstU64<1>;
	type AttributeDepositBase = ConstU64<1>;
	type DepositPerByte = ConstU64<1>;
	type StringLimit = ConstU32<50>;
	type KeyLimit = ConstU32<50>;
	type ValueLimit = ConstU32<50>;
	type ApprovalsLimit = ConstU32<10>;
	type ItemAttributesApprovalsLimit = ConstU32<2>;
	type MaxTips = ConstU32<10>;
	type MaxDeadlineDuration = ConstU64<10000>;
	type MaxAttributesPerCall = ConstU32<2>;
	type Features = Features;
	/// Off-chain = signature On-chain - therefore no conversion needed.
	/// It needs to be From<MultiSignature> for benchmarking.
	type OffchainSignature = Signature;
	/// Using `AccountPublic` here makes it trivial to convert to `AccountId` via `into_account()`.
	type OffchainPublic = AccountPublic;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type BlockNumberProvider = topsoil_system::Pallet<Test>;
}

pub(crate) fn new_test_ext() -> subsoil::io::TestExternalities {
	let t = topsoil_system::GenesisConfig::<Test>::default().build_storage().unwrap();

	let mut ext = subsoil::io::TestExternalities::new(t);
	ext.register_extension(KeystoreExt::new(MemoryKeystore::new()));
	ext.execute_with(|| System::set_block_number(1));
	ext
}
