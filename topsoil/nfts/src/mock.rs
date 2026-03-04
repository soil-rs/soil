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

//! Test environment for Nfts pallet.

use super::*;
use crate as topsoil_nfts;

use topsoil_support::{
	construct_runtime, derive_impl, parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU32, ConstU64},
};
use soil_keystore::{testing::MemoryKeystore, KeystoreExt};
use soil_runtime::{
	traits::{IdentifyAccount, IdentityLookup, Verify},
	BuildStorage, MultiSignature,
};

type Block = topsoil_system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test
	{
		System: topsoil_system,
		Balances: topsoil_balances,
		Nfts: topsoil_nfts,
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
	type AccountData = topsoil_balances::AccountData<u64>;
}

#[derive_impl(topsoil_balances::config_preludes::TestDefaultConfig)]
impl topsoil_balances::Config for Test {
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

pub(crate) fn new_test_ext() -> soil_io::TestExternalities {
	let t = topsoil_system::GenesisConfig::<Test>::default().build_storage().unwrap();

	let mut ext = soil_io::TestExternalities::new(t);
	ext.register_extension(KeystoreExt::new(MemoryKeystore::new()));
	ext.execute_with(|| System::set_block_number(1));
	ext
}
