// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_support::{construct_runtime, derive_impl};
use subsoil::core::sr25519;
use subsoil::runtime::{generic, traits::BlakeTwo256};

#[topsoil_support::pallet]
mod pallet {
	#[pallet::config]
	pub trait Config: topsoil_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(_);
}

pub type Signature = sr25519::Signature;
pub type BlockNumber = u32;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<u32, RuntimeCall, Signature, ()>;

impl pallet::Config for Runtime {}

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Runtime {
	type BaseCallFilter = topsoil_support::traits::Everything;
	type RuntimeOrigin = RuntimeOrigin;
	type Nonce = u64;
	type RuntimeCall = RuntimeCall;
	type Hash = subsoil::runtime::testing::H256;
	type Hashing = subsoil::runtime::traits::BlakeTwo256;
	type AccountId = u64;
	type Lookup = subsoil::runtime::traits::IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = topsoil_support::traits::ConstU32<250>;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = topsoil_support::traits::ConstU32<16>;
}

construct_runtime! {
	pub struct Runtime
	{
		System: topsoil_system expanded::{}::{Pallet, Call, Storage, Config<T>, Event<T>},
		Pallet: pallet expanded::{}::{Pallet, Event},
	}
}

fn main() {}
