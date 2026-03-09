// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! # Preimage test environment.

use super::*;

use crate as plant_preimage;
use subsoil::core::H256;
use subsoil::runtime::{
	traits::{BlakeTwo256, Convert},
	BuildStorage,
};
use topsoil_core::{
	derive_impl, ord_parameter_types, parameter_types,
	traits::{fungible::HoldConsideration, ConstU64},
};
use topsoil_core::system::EnsureSignedBy;

type Block = topsoil_core::system::mocking::MockBlock<Test>;

topsoil_core::construct_runtime!(
	pub enum Test
	{
		System: topsoil_core::system,
		Balances: plant_balances,
		Preimage: plant_preimage,
	}
);

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type Block = Block;
	type AccountData = plant_balances::AccountData<u64>;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Test {
	type ExistentialDeposit = ConstU64<5>;
	type AccountStore = System;
}

ord_parameter_types! {
	pub const One: u64 = 1;
}

parameter_types! {
	pub const PreimageHoldReason: RuntimeHoldReason = RuntimeHoldReason::Preimage(plant_preimage::HoldReason::Preimage);
}

pub struct ConvertDeposit;
impl Convert<Footprint, u64> for ConvertDeposit {
	fn convert(a: Footprint) -> u64 {
		a.count * 2 + a.size
	}
}

impl Config for Test {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type ManagerOrigin = EnsureSignedBy<One, u64>;
	type Consideration = HoldConsideration<u64, Balances, PreimageHoldReason, ConvertDeposit>;
}

pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let mut t = topsoil_core::system::GenesisConfig::<Test>::default().build_storage().unwrap();
	let balances = plant_balances::GenesisConfig::<Test> {
		balances: vec![(1, 100), (2, 100), (3, 100), (4, 100), (5, 100)],
		..Default::default()
	};
	balances.assimilate_storage(&mut t).unwrap();
	t.into()
}

pub fn hashed(data: impl AsRef<[u8]>) -> H256 {
	BlakeTwo256::hash(data.as_ref())
}

/// Insert an un-migrated preimage.
pub fn insert_old_unrequested<T: Config>(
	s: u32,
	acc: T::AccountId,
) -> <T as topsoil_core::system::Config>::Hash {
	// The preimage size does not matter here as it is not touched.
	let preimage = s.to_le_bytes();
	let hash = <T as topsoil_core::system::Config>::Hashing::hash(&preimage[..]);

	#[allow(deprecated)]
	StatusFor::<T>::insert(
		&hash,
		OldRequestStatus::Unrequested { deposit: (acc, 123u32.into()), len: preimage.len() as u32 },
	);
	hash
}
