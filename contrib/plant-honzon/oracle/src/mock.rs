// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) 2020-2025 Acala Foundation.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate as plant_oracle;

use crate::{Config, DefaultCombineData};
use subsoil::runtime::{traits::IdentityLookup, BuildStorage};
use topsoil_core::{
	construct_runtime, derive_impl, parameter_types,
	traits::{ConstU32, SortedMembers, Time},
	PalletId,
};

pub type AccountId = u128;
type Key = u32;
type Value = u32;

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig as topsoil_core::system::DefaultConfig)]
impl topsoil_core::system::Config for Test {
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
}

parameter_types! {
	pub static TIME: u32 = 0;
	pub static MEMBERS: Vec<AccountId> = vec![1, 2, 3];
}

pub struct Timestamp;
impl Time for Timestamp {
	type Moment = u32;

	fn now() -> Self::Moment {
		TIME::get()
	}
}

impl Timestamp {
	pub fn set_timestamp(val: u32) {
		TIME::set(val);
	}
}

pub struct Members;

impl SortedMembers<AccountId> for Members {
	fn sorted_members() -> Vec<AccountId> {
		MEMBERS::get().clone()
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn add(who: &AccountId) {
		MEMBERS::mutate(|members| {
			members.push(*who);
			members.sort();
		})
	}
}
parameter_types! {
	pub const MaxFeedValues: u32 = 5;
	pub const OraclePalletId: PalletId = PalletId(*b"py/oracl");
}

impl Config for Test {
	type OnNewData = ();
	type CombineData = DefaultCombineData<Self, ConstU32<3>, ConstU32<600>>;
	type Time = Timestamp;
	type OracleKey = Key;
	type OracleValue = Value;
	type PalletId = OraclePalletId;
	type Members = Members;
	type WeightInfo = ();
	type MaxHasDispatchedSize = ConstU32<100>;
	type MaxFeedValues = MaxFeedValues;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

type Block = topsoil_core::system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test {
		System: topsoil_core::system,
		ModuleOracle: plant_oracle,
	}
);

pub fn set_members(members: Vec<AccountId>) {
	MEMBERS::set(members);
}

// This function basically just builds a genesis storage key/value store
// according to our desired mockup.
pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let storage = topsoil_core::system::GenesisConfig::<Test>::default().build_storage().unwrap();

	let mut t: subsoil::io::TestExternalities = storage.into();

	t.execute_with(|| {
		Timestamp::set_timestamp(12345);
	});

	t
}
