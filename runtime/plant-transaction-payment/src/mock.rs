// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use super::*;
use crate as plant_transaction_payment;
use plant_balances::Call as BalancesCall;
use topsoil_support::{
	derive_impl,
	dispatch::DispatchClass,
	parameter_types,
	traits::{fungible, Imbalance, OnUnbalanced},
	weights::{Weight, WeightToFee as WeightToFeeT},
};
use topsoil_system as system;

type Block = topsoil_system::mocking::MockBlock<Runtime>;

topsoil_support::construct_runtime!(
	pub struct Runtime
	{
		System: system,
		Balances: plant_balances,
		TransactionPayment: plant_transaction_payment::{Pallet, Storage, Event<T>},
	}
);

pub(crate) const CALL: &<Runtime as topsoil_system::Config>::RuntimeCall =
	&RuntimeCall::Balances(BalancesCall::transfer_allow_death { dest: 2, value: 69 });

parameter_types! {
	pub(crate) static ExtrinsicBaseWeight: Weight = Weight::zero();
}

pub struct BlockWeights;
impl Get<topsoil_system::limits::BlockWeights> for BlockWeights {
	fn get() -> topsoil_system::limits::BlockWeights {
		topsoil_system::limits::BlockWeights::builder()
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
	pub static OperationalFeeMultiplier: u8 = 5;
}

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Runtime {
	type BlockWeights = BlockWeights;
	type Block = Block;
	type AccountData = plant_balances::AccountData<Self::AccountId>;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Runtime {
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
impl OnUnbalanced<fungible::Credit<<Runtime as topsoil_system::Config>::AccountId, Balances>>
	for DealWithFees
{
	fn on_unbalanceds(
		mut fees_then_tips: impl Iterator<
			Item = fungible::Credit<<Runtime as topsoil_system::Config>::AccountId, Balances>,
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

/// Weights used in testing.
pub struct MockWeights;

impl WeightInfo for MockWeights {
	fn charge_transaction_payment() -> Weight {
		Weight::from_parts(10, 0)
	}
}

impl Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = FungibleAdapter<Balances, DealWithFees>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type WeightToFee = WeightToFee;
	type LengthToFee = TransactionByteFee;
	type FeeMultiplierUpdate = ();
	type WeightInfo = MockWeights;
}

#[cfg(feature = "runtime-benchmarks")]
impl crate::BenchmarkConfig for Runtime {}

#[cfg(feature = "runtime-benchmarks")]
pub fn new_test_ext() -> subsoil::io::TestExternalities {
	crate::tests::ExtBuilder::default()
		.base_weight(Weight::from_parts(100, 0))
		.byte_fee(10)
		.balance_factor(0)
		.build()
}
