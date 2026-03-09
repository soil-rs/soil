// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test environment for Asset Rewards pallet.

use super::*;
use crate as plant_asset_rewards;
use core::default::Default;
use subsoil::runtime::{traits::IdentityLookup, BuildStorage};
use topsoil_core::{
	construct_runtime, derive_impl,
	instances::Instance1,
	parameter_types,
	traits::{
		tokens::fungible::{HoldConsideration, NativeFromLeft, NativeOrWithId, UnionOf},
		AsEnsureOriginWithArg, ConstU128, ConstU32, EnsureOrigin, LinearStoragePrice,
	},
	PalletId,
};
use topsoil_core::system::EnsureSigned;

#[cfg(feature = "runtime-benchmarks")]
use self::benchmarking::BenchmarkHelper;

type Block = topsoil_core::system::mocking::MockBlock<MockRuntime>;

construct_runtime!(
	pub enum MockRuntime
	{
		System: topsoil_core::system,
		Balances: plant_balances,
		Assets: plant_assets::<Instance1>,
		AssetsFreezer: plant_assets_freezer::<Instance1>,
		StakingRewards: plant_asset_rewards,
	}
);

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for MockRuntime {
	type AccountId = u128;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type AccountData = plant_balances::AccountData<u128>;
}

impl plant_balances::Config for MockRuntime {
	type Balance = u128;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<100>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = RuntimeFreezeReason;
	type MaxFreezes = ConstU32<50>;
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type DoneSlashHandler = ();
}

impl plant_assets::Config<Instance1> for MockRuntime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u128;
	type RemoveItemsLimit = ConstU32<1000>;
	type AssetId = u32;
	type AssetIdParameter = u32;
	type ReserveData = ();
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<Self::AccountId>>;
	type ForceOrigin = topsoil_core::system::EnsureRoot<Self::AccountId>;
	type AssetDeposit = ConstU128<1>;
	type AssetAccountDeposit = ConstU128<10>;
	type MetadataDepositBase = ConstU128<1>;
	type MetadataDepositPerByte = ConstU128<1>;
	type ApprovalDeposit = ConstU128<1>;
	type StringLimit = ConstU32<50>;
	type Freezer = AssetsFreezer;
	type Holder = ();
	type Extra = ();
	type WeightInfo = ();
	type CallbackHandle = ();
	plant_assets::runtime_benchmarks_enabled! {
		type BenchmarkHelper = ();
	}
}

parameter_types! {
	pub const StakingRewardsPalletId: PalletId = PalletId(*b"py/stkrd");
	pub const Native: NativeOrWithId<u32> = NativeOrWithId::Native;
	pub const PermissionedAccountId: u128 = 0;
}

/// Give Root Origin permission to create pools.
pub struct MockPermissionedOrigin;
impl EnsureOrigin<RuntimeOrigin> for MockPermissionedOrigin {
	type Success = <MockRuntime as topsoil_core::system::Config>::AccountId;

	fn try_origin(origin: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
		match origin.clone().into() {
			Ok(topsoil_core::system::RawOrigin::Root) => Ok(PermissionedAccountId::get()),
			_ => Err(origin),
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
		Ok(RuntimeOrigin::root())
	}
}

/// Allow Freezes for the `Assets` pallet
impl plant_assets_freezer::Config<plant_assets_freezer::Instance1> for MockRuntime {
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type RuntimeEvent = RuntimeEvent;
}

pub type NativeAndAssets = UnionOf<Balances, Assets, NativeFromLeft, NativeOrWithId<u32>, u128>;

pub type NativeAndAssetsFreezer =
	UnionOf<Balances, AssetsFreezer, NativeFromLeft, NativeOrWithId<u32>, u128>;

#[cfg(feature = "runtime-benchmarks")]
pub struct AssetRewardsBenchmarkHelper;
#[cfg(feature = "runtime-benchmarks")]
impl BenchmarkHelper<NativeOrWithId<u32>> for AssetRewardsBenchmarkHelper {
	fn staked_asset() -> NativeOrWithId<u32> {
		NativeOrWithId::<u32>::WithId(101)
	}
	fn reward_asset() -> NativeOrWithId<u32> {
		NativeOrWithId::<u32>::WithId(102)
	}
}

parameter_types! {
	pub const CreationHoldReason: RuntimeHoldReason =
		RuntimeHoldReason::StakingRewards(plant_asset_rewards::HoldReason::PoolCreation);
}

impl Config for MockRuntime {
	type RuntimeEvent = RuntimeEvent;
	type AssetId = NativeOrWithId<u32>;
	type Balance = <Self as plant_balances::Config>::Balance;
	type Assets = NativeAndAssets;
	type AssetsFreezer = NativeAndAssetsFreezer;
	type PalletId = StakingRewardsPalletId;
	type CreatePoolOrigin = MockPermissionedOrigin;
	type WeightInfo = ();
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type Consideration = HoldConsideration<
		u128,
		Balances,
		CreationHoldReason,
		LinearStoragePrice<ConstU128<100>, ConstU128<0>, u128>,
	>;
	type BlockNumberProvider = topsoil_core::system::Pallet<Self>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = AssetRewardsBenchmarkHelper;
}

pub(crate) fn new_test_ext() -> subsoil::io::TestExternalities {
	let mut t = topsoil_core::system::GenesisConfig::<MockRuntime>::default().build_storage().unwrap();

	plant_assets::GenesisConfig::<MockRuntime, Instance1> {
		// Genesis assets: id, owner, is_sufficient, min_balance
		// pub assets: Vec<(T::AssetId, T::AccountId, bool, T::Balance)>,
		assets: vec![(1, 1, true, 1), (10, 1, true, 1), (20, 1, true, 1)],
		// Genesis metadata: id, name, symbol, decimals
		// pub metadata: Vec<(T::AssetId, Vec<u8>, Vec<u8>, u8)>,
		metadata: vec![
			(1, b"test".to_vec(), b"TST".to_vec(), 18),
			(10, b"test10".to_vec(), b"T10".to_vec(), 18),
			(20, b"test20".to_vec(), b"T20".to_vec(), 18),
		],
		// Genesis accounts: id, account_id, balance
		// pub accounts: Vec<(T::AssetId, T::AccountId, T::Balance)>,
		accounts: vec![
			(1, 1, 10000),
			(1, 2, 20000),
			(1, 3, 30000),
			(1, 4, 40000),
			(1, 10, 40000),
			(1, 20, 40000),
		],
		next_asset_id: None,
		reserves: vec![],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let pool_zero_account_id = 31086825966906540362769395565;
	plant_balances::GenesisConfig::<MockRuntime> {
		balances: vec![
			(0, 10000),
			(1, 10000),
			(2, 20000),
			(3, 30000),
			(4, 40000),
			(10, 40000),
			(20, 40000),
			(pool_zero_account_id, 100_000), // Top up the default pool account id
		],
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = subsoil::io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
