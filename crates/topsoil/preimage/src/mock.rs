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

//! # Preimage test environment.

use super::*;

use crate as topsoil_preimage;
use subsoil::core::H256;
use soil_runtime::{
	traits::{BlakeTwo256, Convert},
	BuildStorage,
};
use topsoil_support::{
	derive_impl, ord_parameter_types, parameter_types,
	traits::{fungible::HoldConsideration, ConstU64},
};
use topsoil_system::EnsureSignedBy;

type Block = topsoil_system::mocking::MockBlock<Test>;

topsoil_support::construct_runtime!(
	pub enum Test
	{
		System: topsoil_system,
		Balances: topsoil_balances,
		Preimage: topsoil_preimage,
	}
);

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Test {
	type Block = Block;
	type AccountData = topsoil_balances::AccountData<u64>;
}

#[derive_impl(topsoil_balances::config_preludes::TestDefaultConfig)]
impl topsoil_balances::Config for Test {
	type ExistentialDeposit = ConstU64<5>;
	type AccountStore = System;
}

ord_parameter_types! {
	pub const One: u64 = 1;
}

parameter_types! {
	pub const PreimageHoldReason: RuntimeHoldReason = RuntimeHoldReason::Preimage(topsoil_preimage::HoldReason::Preimage);
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

pub fn new_test_ext() -> soil_io::TestExternalities {
	let mut t = topsoil_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	let balances = topsoil_balances::GenesisConfig::<Test> {
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
) -> <T as topsoil_system::Config>::Hash {
	// The preimage size does not matter here as it is not touched.
	let preimage = s.to_le_bytes();
	let hash = <T as topsoil_system::Config>::Hashing::hash(&preimage[..]);

	#[allow(deprecated)]
	StatusFor::<T>::insert(
		&hash,
		OldRequestStatus::Unrequested { deposit: (acc, 123u32.into()), len: preimage.len() as u32 },
	);
	hash
}
