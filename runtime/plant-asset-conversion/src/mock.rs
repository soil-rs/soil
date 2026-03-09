// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test environment for Asset Conversion pallet.

use super::*;
use crate as plant_asset_conversion;
use core::default::Default;
use subsoil::arithmetic::Permill;
use subsoil::runtime::{
	traits::{AccountIdConversion, IdentityLookup},
	BuildStorage,
};
use topsoil_core::{
	construct_runtime, derive_impl,
	instances::{Instance1, Instance2},
	ord_parameter_types, parameter_types,
	traits::{
		tokens::{
			fungible::{NativeFromLeft, NativeOrWithId, UnionOf},
			imbalance::ResolveAssetTo,
		},
		AsEnsureOriginWithArg, ConstU128, ConstU32,
	},
	PalletId,
};
use topsoil_core::system::{EnsureSigned, EnsureSignedBy};

type Block = topsoil_core::system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test
	{
		System: topsoil_core::system,
		Balances: plant_balances,
		Assets: plant_assets::<Instance1>,
		PoolAssets: plant_assets::<Instance2>,
		AssetConversion: plant_asset_conversion,
	}
);

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type AccountId = u128;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type AccountData = plant_balances::AccountData<u128>;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Test {
	type Balance = u128;
	type ExistentialDeposit = ConstU128<100>;
	type AccountStore = System;
}

impl plant_assets::Config<Instance1> for Test {
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
	type Holder = ();
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
	type CallbackHandle = ();
	plant_assets::runtime_benchmarks_enabled! {
		type BenchmarkHelper = ();
	}
}

impl plant_assets::Config<Instance2> for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u128;
	type RemoveItemsLimit = ConstU32<1000>;
	type AssetId = u32;
	type AssetIdParameter = u32;
	type ReserveData = ();
	type Currency = Balances;
	type CreateOrigin =
		AsEnsureOriginWithArg<EnsureSignedBy<AssetConversionOrigin, Self::AccountId>>;
	type ForceOrigin = topsoil_core::system::EnsureRoot<Self::AccountId>;
	type AssetDeposit = ConstU128<0>;
	type AssetAccountDeposit = ConstU128<0>;
	type MetadataDepositBase = ConstU128<0>;
	type MetadataDepositPerByte = ConstU128<0>;
	type ApprovalDeposit = ConstU128<0>;
	type StringLimit = ConstU32<50>;
	type Holder = ();
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
	type CallbackHandle = ();
	plant_assets::runtime_benchmarks_enabled! {
		type BenchmarkHelper = ();
	}
}

parameter_types! {
	pub const AssetConversionPalletId: PalletId = PalletId(*b"py/ascon");
	pub const Native: NativeOrWithId<u32> = NativeOrWithId::Native;
	pub storage LiquidityWithdrawalFee: Permill = Permill::from_percent(0);
}

ord_parameter_types! {
	pub const AssetConversionOrigin: u128 = AccountIdConversion::<u128>::into_account_truncating(&AssetConversionPalletId::get());
}

pub type NativeAndAssets = UnionOf<Balances, Assets, NativeFromLeft, NativeOrWithId<u32>, u128>;
pub type PoolIdToAccountId =
	AccountIdConverter<AssetConversionPalletId, (NativeOrWithId<u32>, NativeOrWithId<u32>)>;
pub type AscendingLocator = Ascending<u128, NativeOrWithId<u32>, PoolIdToAccountId>;
pub type WithFirstAssetLocator =
	WithFirstAsset<Native, u128, NativeOrWithId<u32>, PoolIdToAccountId>;

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = <Self as plant_balances::Config>::Balance;
	type HigherPrecisionBalance = subsoil::core::U256;
	type AssetKind = NativeOrWithId<u32>;
	type Assets = NativeAndAssets;
	type PoolId = (Self::AssetKind, Self::AssetKind);
	type PoolLocator = Chain<WithFirstAssetLocator, AscendingLocator>;
	type PoolAssetId = u32;
	type PoolAssets = PoolAssets;
	type PoolSetupFee = ConstU128<100>; // should be more or equal to the existential deposit
	type PoolSetupFeeAsset = Native;
	type PoolSetupFeeTarget = ResolveAssetTo<AssetConversionOrigin, Self::Assets>;
	type PalletId = AssetConversionPalletId;
	type WeightInfo = ();
	type LPFee = ConstU32<3>; // means 0.3%
	type LiquidityWithdrawalFee = LiquidityWithdrawalFee;
	type MaxSwapPathLength = ConstU32<4>;
	type MintMinLiquidity = ConstU128<100>; // 100 is good enough when the main currency has 12 decimals.
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

pub(crate) fn new_test_ext() -> subsoil::io::TestExternalities {
	let mut t = topsoil_core::system::GenesisConfig::<Test>::default().build_storage().unwrap();

	plant_balances::GenesisConfig::<Test> {
		balances: vec![(1, 10000), (2, 20000), (3, 30000), (4, 40000)],
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = subsoil::io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
