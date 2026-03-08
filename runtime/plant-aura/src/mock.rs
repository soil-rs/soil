// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test utilities

#![cfg(test)]

use crate as plant_aura;
use subsoil::consensus::aura::{ed25519::AuthorityId, AuthorityIndex};
use subsoil::runtime::{testing::UintAuthorityId, BuildStorage};
use topsoil_support::{
	derive_impl, parameter_types,
	traits::{ConstU32, ConstU64, DisabledValidators},
};

type Block = topsoil_system::mocking::MockBlock<Test>;

pub const SLOT_DURATION: u64 = 2;

topsoil_support::construct_runtime!(
	pub enum Test
	{
		System: topsoil_system,
		Timestamp: plant_timestamp,
		Aura: plant_aura,
	}
);

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Test {
	type Block = Block;
}

impl plant_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = Aura;
	type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
	type WeightInfo = ();
}

parameter_types! {
	static DisabledValidatorTestValue: Vec<AuthorityIndex> = Default::default();
	pub static AllowMultipleBlocksPerSlot: bool = false;
	pub static SlotDurationValue: u64 = SLOT_DURATION;
}

pub struct MockDisabledValidators;

impl MockDisabledValidators {
	pub fn disable_validator(index: AuthorityIndex) {
		DisabledValidatorTestValue::mutate(|v| {
			if let Err(i) = v.binary_search(&index) {
				v.insert(i, index);
			}
		})
	}
}

impl DisabledValidators for MockDisabledValidators {
	fn is_disabled(index: AuthorityIndex) -> bool {
		DisabledValidatorTestValue::get().binary_search(&index).is_ok()
	}

	fn disabled_validators() -> Vec<u32> {
		DisabledValidatorTestValue::get()
	}
}

impl plant_aura::Config for Test {
	type AuthorityId = AuthorityId;
	type DisabledValidators = MockDisabledValidators;
	type MaxAuthorities = ConstU32<10>;
	type AllowMultipleBlocksPerSlot = AllowMultipleBlocksPerSlot;
	type SlotDuration = SlotDurationValue;
}

pub fn build_ext(authorities: Vec<u64>) -> subsoil::io::TestExternalities {
	let mut storage = topsoil_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	plant_aura::GenesisConfig::<Test> {
		authorities: authorities.into_iter().map(|a| UintAuthorityId(a).to_public_key()).collect(),
	}
	.assimilate_storage(&mut storage)
	.unwrap();
	storage.into()
}

pub fn build_ext_and_execute_test(authorities: Vec<u64>, test: impl FnOnce() -> ()) {
	let mut ext = build_ext(authorities);
	ext.execute_with(|| {
		test();
		Aura::do_try_state().expect("Storage invariants should hold")
	});
}
