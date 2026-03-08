// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test environment for remarks pallet.

use crate as plant_remark;
use subsoil::runtime::BuildStorage;
use topsoil_support::derive_impl;

pub type Block = topsoil_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
topsoil_support::construct_runtime!(
	pub enum Test
	{
		System: topsoil_system,
		Remark: plant_remark,
	}
);

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Test {
	type Block = Block;
}

impl plant_remark::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let t = RuntimeGenesisConfig { system: Default::default() }.build_storage().unwrap();
	t.into()
}
