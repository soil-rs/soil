// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test environment for Asset Conversion Ops pallet.

use crate as plant_asset_conversion_ops;
use core::default::Default;
use subsoil::arithmetic::Permill;
use subsoil::runtime::{traits::AccountIdConversion, BuildStorage};
use plant_asset_conversion::{self, AccountIdConverter, AccountIdConverterNoSeed, Ascending};
use topsoil_core::{
	construct_runtime, derive_impl,
	instances::{Instance1, Instance2},
	ord_parameter_types, parameter_types,
	traits::{
		tokens::{
			fungible::{NativeFromLeft, NativeOrWithId, UnionOf},
			imbalance::ResolveAssetTo,
		},
		AsEnsureOriginWithArg, ConstU32, ConstU64,
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
	AssetConversionOps: plant_asset_conversion_ops,
  }
);

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type Block = Block;
	type AccountData = plant_balances::AccountData<u64>;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Test {
	type AccountStore = System;
}

#[derive_impl(plant_assets::config_preludes::TestDefaultConfig)]
impl plant_assets::Config<Instance1> for Test {
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<Self::AccountId>>;
	type ForceOrigin = topsoil_core::system::EnsureRoot<Self::AccountId>;
	type Holder = ();
	type Freezer = ();
}

#[derive_impl(plant_assets::config_preludes::TestDefaultConfig)]
impl plant_assets::Config<Instance2> for Test {
	type Currency = Balances;
	type CreateOrigin =
		AsEnsureOriginWithArg<EnsureSignedBy<AssetConversionOrigin, Self::AccountId>>;
	type ForceOrigin = topsoil_core::system::EnsureRoot<Self::AccountId>;
	type Holder = ();
	type Freezer = ();
}

parameter_types! {
  pub const AssetConversionPalletId: PalletId = PalletId(*b"py/ascon");
  pub const Native: NativeOrWithId<u32> = NativeOrWithId::Native;
  pub storage LiquidityWithdrawalFee: Permill = Permill::from_percent(0);
}

ord_parameter_types! {
  pub const AssetConversionOrigin: u64 = AccountIdConversion::<u64>::into_account_truncating(&AssetConversionPalletId::get());
}

pub type NativeAndAssets = UnionOf<Balances, Assets, NativeFromLeft, NativeOrWithId<u32>, u64>;
pub type PoolIdToAccountId =
	AccountIdConverter<AssetConversionPalletId, (NativeOrWithId<u32>, NativeOrWithId<u32>)>;
pub type AscendingLocator = Ascending<u64, NativeOrWithId<u32>, PoolIdToAccountId>;

impl plant_asset_conversion::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = <Self as plant_balances::Config>::Balance;
	type HigherPrecisionBalance = subsoil::core::U256;
	type AssetKind = NativeOrWithId<u32>;
	type Assets = NativeAndAssets;
	type PoolId = (Self::AssetKind, Self::AssetKind);
	type PoolLocator = AscendingLocator;
	type PoolAssetId = u32;
	type PoolAssets = PoolAssets;
	type PoolSetupFee = ConstU64<100>;
	type PoolSetupFeeAsset = Native;
	type PoolSetupFeeTarget = ResolveAssetTo<AssetConversionOrigin, Self::Assets>;
	type PalletId = AssetConversionPalletId;
	type WeightInfo = ();
	type LPFee = ConstU32<3>;
	type LiquidityWithdrawalFee = LiquidityWithdrawalFee;
	type MaxSwapPathLength = ConstU32<4>;
	type MintMinLiquidity = ConstU64<100>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

pub type OldPoolIdToAccountId =
	AccountIdConverterNoSeed<(NativeOrWithId<u32>, NativeOrWithId<u32>)>;

impl plant_asset_conversion_ops::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type PriorAccountIdConverter = OldPoolIdToAccountId;
	type AssetsRefund = NativeAndAssets;
	type PoolAssetsRefund = PoolAssets;
	type PoolAssetsTeam = PoolAssets;
	type DepositAsset = Balances;
	type WeightInfo = ();
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
