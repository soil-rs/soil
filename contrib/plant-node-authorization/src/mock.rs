// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test environment for node-authorization pallet.

use super::*;
use crate as plant_node_authorization;

use topsoil::testing_prelude::*;

type Block = topsoil_system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test
	{
		System: topsoil_system,
		NodeAuthorization: plant_node_authorization,
	}
);

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Test {
	type Block = Block;
}

ord_parameter_types! {
	pub const One: u64 = 1;
	pub const Two: u64 = 2;
	pub const Three: u64 = 3;
	pub const Four: u64 = 4;
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxWellKnownNodes = ConstU32<4>;
	type MaxPeerIdLength = ConstU32<2>;
	type AddOrigin = EnsureSignedBy<One, u64>;
	type RemoveOrigin = EnsureSignedBy<Two, u64>;
	type SwapOrigin = EnsureSignedBy<Three, u64>;
	type ResetOrigin = EnsureSignedBy<Four, u64>;
	type WeightInfo = ();
}

pub fn test_node(id: u8) -> PeerId {
	PeerId(vec![id])
}

pub fn new_test_ext() -> TestState {
	let mut t = topsoil_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	plant_node_authorization::GenesisConfig::<Test> {
		nodes: vec![(test_node(10), 10), (test_node(20), 20), (test_node(30), 30)],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
}
