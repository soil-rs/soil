// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use super::*;
use crate as plant_asset_conversion_tx_payment;

use subsoil::runtime::{
	traits::{AccountIdConversion, IdentityLookup, SaturatedConversion},
	Permill,
};
use plant_asset_conversion::{Ascending, Chain, WithFirstAsset};
use topsoil_core::{
	derive_impl,
	dispatch::DispatchClass,
	instances::Instance2,
	ord_parameter_types,
	pallet_prelude::*,
	parameter_types,
	traits::{
		fungible, fungibles,
		tokens::{
			fungible::{NativeFromLeft, NativeOrWithId, UnionOf},
			imbalance::ResolveAssetTo,
		},
		AsEnsureOriginWithArg, ConstU32, ConstU64, ConstU8, Imbalance, OnUnbalanced,
	},
	weights::{Weight, WeightToFee as WeightToFeeT},
	PalletId,
};
use topsoil_core::system as system;
use topsoil_core::system::{EnsureRoot, EnsureSignedBy};
use plant_transaction_payment::FungibleAdapter;

type Block = topsoil_core::system::mocking::MockBlock<Runtime>;
type Balance = u64;
type AccountId = u64;

topsoil_core::construct_runtime!(
	pub enum Runtime
	{
		System: system,
		Balances: plant_balances,
		TransactionPayment: plant_transaction_payment,
		Assets: plant_assets,
		PoolAssets: plant_assets::<Instance2>,
		AssetConversion: plant_asset_conversion,
		AssetTxPayment: plant_asset_conversion_tx_payment,
	}
);

parameter_types! {
	pub(crate) static ExtrinsicBaseWeight: Weight = Weight::zero();
}

pub struct BlockWeights;
impl Get<topsoil_core::system::limits::BlockWeights> for BlockWeights {
	fn get() -> topsoil_core::system::limits::BlockWeights {
		topsoil_core::system::limits::BlockWeights::builder()
			.base_block(Weight::zero())
			.for_class(DispatchClass::all(), |weights| {
				weights.base_extrinsic = ExtrinsicBaseWeight::get().into();
			})
			.for_class(DispatchClass::non_mandatory(), |weights| {
				weights.max_total = Weight::from_parts(1024, u64::MAX).into();
			})
			.build_or_panic()
	}
}

parameter_types! {
	pub static WeightToFee: u64 = 1;
	pub static TransactionByteFee: u64 = 1;
}

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Runtime {
	type BlockWeights = BlockWeights;
	type Nonce = u64;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type AccountData = plant_balances::AccountData<u64>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 10;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Runtime {
	type ExistentialDeposit = ConstU64<10>;
	type AccountStore = System;
}

impl WeightToFeeT for WeightToFee {
	type Balance = u64;

	fn weight_to_fee(weight: &Weight) -> Self::Balance {
		Self::Balance::saturated_from(weight.ref_time())
			.saturating_mul(WEIGHT_TO_FEE.with(|v| *v.borrow()))
	}
}

impl WeightToFeeT for TransactionByteFee {
	type Balance = u64;

	fn weight_to_fee(weight: &Weight) -> Self::Balance {
		Self::Balance::saturated_from(weight.ref_time())
			.saturating_mul(TRANSACTION_BYTE_FEE.with(|v| *v.borrow()))
	}
}

parameter_types! {
	pub(crate) static TipUnbalancedAmount: u64 = 0;
	pub(crate) static FeeUnbalancedAmount: u64 = 0;
}

pub struct DealWithFees;
impl OnUnbalanced<fungible::Credit<<Runtime as topsoil_core::system::Config>::AccountId, Balances>>
	for DealWithFees
{
	fn on_unbalanceds(
		mut fees_then_tips: impl Iterator<
			Item = fungible::Credit<<Runtime as topsoil_core::system::Config>::AccountId, Balances>,
		>,
	) {
		if let Some(fees) = fees_then_tips.next() {
			FeeUnbalancedAmount::mutate(|a| *a += fees.peek());
			if let Some(tips) = fees_then_tips.next() {
				TipUnbalancedAmount::mutate(|a| *a += tips.peek());
			}
		}
	}
}

pub struct MockTxPaymentWeights;

impl plant_transaction_payment::WeightInfo for MockTxPaymentWeights {
	fn charge_transaction_payment() -> Weight {
		Weight::from_parts(10, 0)
	}
}

pub struct DealWithFungiblesFees;
impl OnUnbalanced<fungibles::Credit<AccountId, NativeAndAssets>> for DealWithFungiblesFees {
	fn on_unbalanceds(
		mut fees_then_tips: impl Iterator<
			Item = fungibles::Credit<
				<Runtime as topsoil_core::system::Config>::AccountId,
				NativeAndAssets,
			>,
		>,
	) {
		if let Some(fees) = fees_then_tips.next() {
			FeeUnbalancedAmount::mutate(|a| *a += fees.peek());
			if let Some(tips) = fees_then_tips.next() {
				TipUnbalancedAmount::mutate(|a| *a += tips.peek());
			}
		}
	}
}

#[derive_impl(plant_transaction_payment::config_preludes::TestDefaultConfig)]
impl plant_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = FungibleAdapter<Balances, DealWithFees>;
	type WeightToFee = WeightToFee;
	type LengthToFee = TransactionByteFee;
	type OperationalFeeMultiplier = ConstU8<5>;
	type WeightInfo = MockTxPaymentWeights;
}

type AssetId = u32;

impl plant_assets::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = AssetId;
	type AssetIdParameter = codec::Compact<AssetId>;
	type ReserveData = ();
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<topsoil_core::system::EnsureSigned<AccountId>>;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = ConstU64<2>;
	type AssetAccountDeposit = ConstU64<2>;
	type MetadataDepositBase = ConstU64<0>;
	type MetadataDepositPerByte = ConstU64<0>;
	type ApprovalDeposit = ConstU64<0>;
	type StringLimit = ConstU32<20>;
	type Holder = ();
	type Freezer = ();
	type Extra = ();
	type CallbackHandle = ();
	type WeightInfo = ();
	type RemoveItemsLimit = ConstU32<1000>;
	plant_assets::runtime_benchmarks_enabled! {
		type BenchmarkHelper = ();
	}
}

impl plant_assets::Config<Instance2> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u64;
	type RemoveItemsLimit = ConstU32<1000>;
	type AssetId = u32;
	type AssetIdParameter = u32;
	type ReserveData = ();
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSignedBy<AssetConversionOrigin, u64>>;
	type ForceOrigin = topsoil_core::system::EnsureRoot<u64>;
	type AssetDeposit = ConstU64<0>;
	type AssetAccountDeposit = ConstU64<0>;
	type MetadataDepositBase = ConstU64<0>;
	type MetadataDepositPerByte = ConstU64<0>;
	type ApprovalDeposit = ConstU64<0>;
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
	pub storage LiquidityWithdrawalFee: Permill = Permill::from_percent(0);
	pub const MaxSwapPathLength: u32 = 4;
	pub const Native: NativeOrWithId<u32> = NativeOrWithId::Native;
}

ord_parameter_types! {
	pub const AssetConversionOrigin: u64 = AccountIdConversion::<u64>::into_account_truncating(&AssetConversionPalletId::get());
}

pub type PoolIdToAccountId = plant_asset_conversion::AccountIdConverter<
	AssetConversionPalletId,
	(NativeOrWithId<u32>, NativeOrWithId<u32>),
>;

type NativeAndAssets = UnionOf<Balances, Assets, NativeFromLeft, NativeOrWithId<u32>, AccountId>;

impl plant_asset_conversion::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type HigherPrecisionBalance = u128;
	type AssetKind = NativeOrWithId<u32>;
	type Assets = NativeAndAssets;
	type PoolId = (Self::AssetKind, Self::AssetKind);
	type PoolLocator = Chain<
		WithFirstAsset<Native, AccountId, NativeOrWithId<u32>, PoolIdToAccountId>,
		Ascending<AccountId, NativeOrWithId<u32>, PoolIdToAccountId>,
	>;
	type PoolAssetId = u32;
	type PoolAssets = PoolAssets;
	type PoolSetupFee = ConstU64<100>; // should be more or equal to the existential deposit
	type PoolSetupFeeAsset = Native;
	type PoolSetupFeeTarget = ResolveAssetTo<AssetConversionOrigin, Self::Assets>;
	type PalletId = AssetConversionPalletId;
	type LPFee = ConstU32<3>; // means 0.3%
	type LiquidityWithdrawalFee = LiquidityWithdrawalFee;
	type MaxSwapPathLength = MaxSwapPathLength;
	type MintMinLiquidity = ConstU64<100>; // 100 is good enough when the main currency has 12 decimals.
	type WeightInfo = ();
	plant_asset_conversion::runtime_benchmarks_enabled! {
		type BenchmarkHelper = ();
	}
}

/// Weights used in testing.
pub struct MockWeights;

impl WeightInfo for MockWeights {
	fn charge_asset_tx_payment_zero() -> Weight {
		Weight::from_parts(0, 0)
	}

	fn charge_asset_tx_payment_native() -> Weight {
		Weight::from_parts(15, 0)
	}

	fn charge_asset_tx_payment_asset() -> Weight {
		Weight::from_parts(20, 0)
	}
}

impl Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AssetId = NativeOrWithId<u32>;
	type OnChargeAssetTransaction =
		SwapAssetAdapter<Native, NativeAndAssets, AssetConversion, DealWithFungiblesFees>;
	type WeightInfo = MockWeights;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = Helper;
}

#[cfg(feature = "runtime-benchmarks")]
pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let base_weight = 5;
	let balance_factor = 100;
	crate::tests::ExtBuilder::default()
		.balance_factor(balance_factor)
		.base_weight(Weight::from_parts(base_weight, 0))
		.build()
}

#[cfg(feature = "runtime-benchmarks")]
pub struct Helper;

#[cfg(feature = "runtime-benchmarks")]
impl BenchmarkHelperTrait<u64, NativeOrWithId<u32>, NativeOrWithId<u32>> for Helper {
	fn create_asset_id_parameter(id: u32) -> (NativeOrWithId<u32>, NativeOrWithId<u32>) {
		(NativeOrWithId::WithId(id), NativeOrWithId::WithId(id))
	}

	fn setup_balances_and_pool(asset_id: NativeOrWithId<u32>, account: u64) {
		use subsoil::runtime::traits::StaticLookup;
		use topsoil_core::{assert_ok, traits::fungibles::Mutate};
		let NativeOrWithId::WithId(asset_idx) = asset_id.clone() else { unimplemented!() };
		assert_ok!(Assets::force_create(
			RuntimeOrigin::root(),
			asset_idx.into(),
			42,   // owner
			true, // is_sufficient
			1,
		));

		let lp_provider = 12;
		assert_ok!(Balances::force_set_balance(RuntimeOrigin::root(), lp_provider, u64::MAX / 2));
		let lp_provider_account = <Runtime as system::Config>::Lookup::unlookup(lp_provider);
		assert_ok!(Assets::mint_into(asset_idx, &lp_provider_account, u64::MAX / 2));

		let token_1 = Box::new(NativeOrWithId::Native);
		let token_2 = Box::new(asset_id);
		assert_ok!(AssetConversion::create_pool(
			RuntimeOrigin::signed(lp_provider),
			token_1.clone(),
			token_2.clone()
		));

		assert_ok!(AssetConversion::add_liquidity(
			RuntimeOrigin::signed(lp_provider),
			token_1,
			token_2,
			(u32::MAX / 8).into(), // 1 desired
			u32::MAX.into(),       // 2 desired
			1,                     // 1 min
			1,                     // 2 min
			lp_provider_account,
		));

		use topsoil_core::traits::Currency;
		let _ = Balances::deposit_creating(&account, u32::MAX.into());

		let beneficiary = <Runtime as system::Config>::Lookup::unlookup(account);
		let balance = 1000;

		assert_ok!(Assets::mint_into(asset_idx, &beneficiary, balance));
		assert_eq!(Assets::balance(asset_idx, account), balance);
	}
}
