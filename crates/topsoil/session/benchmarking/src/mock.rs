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

//! Mock file for session benchmarking.

#![cfg(test)]

use codec::Encode;
use soil_runtime::{traits::IdentityLookup, BuildStorage, KeyTypeId};
use topsoil_election_provider_support::{
	bounds::{ElectionBounds, ElectionBoundsBuilder},
	onchain, SequentialPhragmen,
};
use topsoil_support::{
	derive_impl, parameter_types,
	traits::{ConstU32, ConstU64},
};

type AccountId = u64;
type Nonce = u32;

type Block = topsoil_system::mocking::MockBlock<Test>;

topsoil_support::construct_runtime!(
	pub enum Test
	{
		System: topsoil_system,
		Balances: topsoil_balances,
		Staking: topsoil_staking,
		Session: topsoil_session,
		Historical: topsoil_session::historical
	}
);

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Test {
	type Nonce = Nonce;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type AccountData = topsoil_balances::AccountData<u64>;
}

#[derive_impl(topsoil_balances::config_preludes::TestDefaultConfig)]
impl topsoil_balances::Config for Test {
	type ExistentialDeposit = ConstU64<10>;
	type AccountStore = System;
}

impl topsoil_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<5>;
	type WeightInfo = ();
}
impl topsoil_session::historical::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type FullIdentification = ();
	type FullIdentificationOf = topsoil_staking::UnitIdentificationOf<Self>;
}

soil_runtime::impl_opaque_keys! {
	pub struct SessionKeys {
		pub foo: soil_runtime::testing::UintAuthorityId,
	}
}

pub struct TestSessionHandler;
impl topsoil_session::SessionHandler<AccountId> for TestSessionHandler {
	// corresponds to the opaque key id above
	const KEY_TYPE_IDS: &'static [KeyTypeId] = &[KeyTypeId([100u8, 117u8, 109u8, 121u8])];

	fn on_genesis_session<Ks: soil_runtime::traits::OpaqueKeys>(_validators: &[(AccountId, Ks)]) {}

	fn on_new_session<Ks: soil_runtime::traits::OpaqueKeys>(
		_: bool,
		_: &[(AccountId, Ks)],
		_: &[(AccountId, Ks)],
	) {
	}

	fn on_disabled(_: u32) {}
}

impl topsoil_session::Config for Test {
	type SessionManager = topsoil_session::historical::NoteHistoricalRoot<Test, Staking>;
	type Keys = SessionKeys;
	type ShouldEndSession = topsoil_session::PeriodicSessions<(), ()>;
	type NextSessionRotation = topsoil_session::PeriodicSessions<(), ()>;
	type SessionHandler = TestSessionHandler;
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = AccountId;
	type ValidatorIdOf = soil_runtime::traits::ConvertInto;
	type DisablingStrategy = ();
	type WeightInfo = ();
	type Currency = Balances;
	// Note: setting to a large amount to ensure bench setup can handle increasing the balance of
	// the validator before setting session keys; see `ensure_can_pay_key_deposit`.
	type KeyDeposit = ConstU64<2000000000>;
}
topsoil_staking_reward_curve::build! {
	const I_NPOS: soil_runtime::curve::PiecewiseLinear<'static> = curve!(
		min_inflation: 0_025_000,
		max_inflation: 0_100_000,
		ideal_stake: 0_500_000,
		falloff: 0_050_000,
		max_piece_count: 40,
		test_precision: 0_005_000,
	);
}
parameter_types! {
	pub const RewardCurve: &'static soil_runtime::curve::PiecewiseLinear<'static> = &I_NPOS;
	pub static ElectionsBounds: ElectionBounds = ElectionBoundsBuilder::default().build();
	pub const Sort: bool = true;
}

pub struct OnChainSeqPhragmen;
impl onchain::Config for OnChainSeqPhragmen {
	type System = Test;
	type Solver = SequentialPhragmen<AccountId, soil_runtime::Perbill>;
	type DataProvider = Staking;
	type WeightInfo = ();
	type MaxWinnersPerPage = ConstU32<100>;
	type MaxBackersPerWinner = ConstU32<100>;
	type Sort = Sort;
	type Bounds = ElectionsBounds;
}

#[derive_impl(topsoil_staking::config_preludes::TestDefaultConfig)]
impl topsoil_staking::Config for Test {
	type OldCurrency = Balances;
	type Currency = Balances;
	type CurrencyBalance = <Self as topsoil_balances::Config>::Balance;
	type UnixTime = topsoil_timestamp::Pallet<Self>;
	type AdminOrigin = topsoil_system::EnsureRoot<Self::AccountId>;
	type SessionInterface = Self;
	type EraPayout = topsoil_staking::ConvertCurve<RewardCurve>;
	type NextNewSession = Session;
	type ElectionProvider = onchain::OnChainExecution<OnChainSeqPhragmen>;
	type GenesisElectionProvider = Self::ElectionProvider;
	type VoterList = topsoil_staking::UseNominatorsAndValidatorsMap<Self>;
	type TargetList = topsoil_staking::UseValidatorsMap<Self>;
}

impl crate::Config for Test {
	fn generate_session_keys_and_proof(owner: Self::AccountId) -> (Self::Keys, Vec<u8>) {
		let keys = SessionKeys::generate(&owner.encode(), None);

		(keys.keys, keys.proof.encode())
	}
}

pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let t = topsoil_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	subsoil::io::TestExternalities::new(t)
}
