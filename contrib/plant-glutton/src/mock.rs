// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use super::*;
use crate as plant_glutton;

use subsoil::runtime::BuildStorage;
use topsoil_core::{assert_ok, derive_impl};

type Block = topsoil_core::system::mocking::MockBlock<Test>;

topsoil_core::construct_runtime!(
	pub enum Test
	{
		System: topsoil_core::system,
		Glutton: plant_glutton,
	}
);

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type Block = Block;
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type AdminOrigin = topsoil_core::system::EnsureRoot<Self::AccountId>;
	type WeightInfo = ();
}

pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let t = topsoil_core::system::GenesisConfig::<Test>::default().build_storage().unwrap();

	let mut ext = subsoil::io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

/// Set the `compute`, `storage` and `block_length` limits.
///
/// `1.0` corresponds to `100%`.
pub fn set_limits(compute: f64, storage: f64, block_length: f64) {
	assert_ok!(Glutton::set_compute(RuntimeOrigin::root(), FixedU64::from_float(compute)));
	assert_ok!(Glutton::set_storage(RuntimeOrigin::root(), FixedU64::from_float(storage)));
	assert_ok!(Glutton::set_block_length(
		RuntimeOrigin::root(),
		FixedU64::from_float(block_length)
	));
}
