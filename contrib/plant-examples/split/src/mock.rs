// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: MIT-0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate as pallet_template;
use topsoil_core::{derive_impl, subsoil::runtime::BuildStorage};

type Block = topsoil_core::system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
topsoil_core::construct_runtime!(
	pub enum Test
	{
		System: topsoil_core::system,
		TemplatePallet: pallet_template,
	}
);

/// Using a default config for [`topsoil_system`] in tests. See `default-config` example for more
/// details.
#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type Block = Block;
}

impl pallet_template::Config for Test {
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> subsoil::io::TestExternalities {
	topsoil_core::system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
