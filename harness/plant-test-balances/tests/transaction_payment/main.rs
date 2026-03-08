// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use plant_transaction_payment::*;
use plant_balances::Call as BalancesCall;
use subsoil::runtime::{traits::Zero, FixedPointNumber, SaturatedConversion};
use topsoil_support::{
	derive_impl,
	dispatch::DispatchClass,
	parameter_types,
	traits::{fungible, Get, Imbalance, OnUnbalanced},
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
impl plant_transaction_payment::BenchmarkConfig for Runtime {}

#[cfg(feature = "runtime-benchmarks")]
pub fn new_test_ext() -> subsoil::io::TestExternalities {
	ExtBuilder::default()
		.base_weight(Weight::from_parts(100, 0))
		.byte_fee(10)
		.balance_factor(0)
		.build()
}

use codec::Encode;

use subsoil::runtime::{
	generic::UncheckedExtrinsic,
	traits::{DispatchTransaction, One, TransactionExtension},
	transaction_validity::{InvalidTransaction, TransactionSource::External, TransactionValidityError},
	BuildStorage,
};

use topsoil_support::{
	assert_ok,
	dispatch::{DispatchInfo, GetDispatchInfo, Pays, PostDispatchInfo},
	traits::{Currency, OriginTrait},
};

pub struct ExtBuilder {
	balance_factor: u64,
	base_weight: Weight,
	byte_fee: u64,
	weight_to_fee: u64,
	initial_multiplier: Option<Multiplier>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			balance_factor: 1,
			base_weight: Weight::zero(),
			byte_fee: 1,
			weight_to_fee: 1,
			initial_multiplier: None,
		}
	}
}

impl ExtBuilder {
	pub fn base_weight(mut self, base_weight: Weight) -> Self {
		self.base_weight = base_weight;
		self
	}
	pub fn byte_fee(mut self, byte_fee: u64) -> Self {
		self.byte_fee = byte_fee;
		self
	}
	pub fn weight_fee(mut self, weight_to_fee: u64) -> Self {
		self.weight_to_fee = weight_to_fee;
		self
	}
	pub fn balance_factor(mut self, factor: u64) -> Self {
		self.balance_factor = factor;
		self
	}
	pub fn with_initial_multiplier(mut self, multiplier: Multiplier) -> Self {
		self.initial_multiplier = Some(multiplier);
		self
	}
	fn set_constants(&self) {
		ExtrinsicBaseWeight::mutate(|v| *v = self.base_weight);
		TRANSACTION_BYTE_FEE.with(|v| *v.borrow_mut() = self.byte_fee);
		WEIGHT_TO_FEE.with(|v| *v.borrow_mut() = self.weight_to_fee);
	}
	pub fn build(self) -> subsoil::io::TestExternalities {
		self.set_constants();
		let mut t = topsoil_system::GenesisConfig::<Runtime>::default().build_storage().unwrap();
		plant_balances::GenesisConfig::<Runtime> {
			balances: if self.balance_factor > 0 {
				vec![
					(1, 10 * self.balance_factor),
					(2, 20 * self.balance_factor),
					(3, 30 * self.balance_factor),
					(4, 40 * self.balance_factor),
					(5, 50 * self.balance_factor),
					(6, 60 * self.balance_factor),
				]
			} else {
				vec![]
			},
			..Default::default()
		}
		.assimilate_storage(&mut t)
		.unwrap();

		if let Some(multiplier) = self.initial_multiplier {
			pallet::GenesisConfig::<Runtime> { multiplier, ..Default::default() }
				.assimilate_storage(&mut t)
				.unwrap();
		}

		t.into()
	}
}

/// create a transaction info struct from weight. Handy to avoid building the whole struct.
pub fn info_from_weight(w: Weight) -> DispatchInfo {
	// pays_fee: Pays::Yes -- class: DispatchClass::Normal
	DispatchInfo { call_weight: w, ..Default::default() }
}

fn post_info_from_weight(w: Weight) -> PostDispatchInfo {
	PostDispatchInfo { actual_weight: Some(w), pays_fee: Default::default() }
}

fn post_info_from_pays(p: Pays) -> PostDispatchInfo {
	PostDispatchInfo { actual_weight: None, pays_fee: p }
}

fn default_post_info() -> PostDispatchInfo {
	PostDispatchInfo { actual_weight: None, pays_fee: Default::default() }
}

mod tests;
