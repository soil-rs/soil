// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use subsoil::core::sr25519;
use subsoil::runtime::{
	generic,
	traits::{BlakeTwo256, Verify},
};
use topsoil_support::derive_impl;
use topsoil_system::pallet_prelude::BlockNumberFor;

#[topsoil_support::pallet]
pub mod pallet {
	use super::*;
	use topsoil_support::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: topsoil_system::Config {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[pallet::storage]
	#[pallet::unbounded]
	pub type AppendableDM<T: Config> =
		StorageDoubleMap<_, Identity, u32, Identity, BlockNumberFor<T>, Vec<u32>>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub t: Vec<(u32, BlockNumberFor<T>, Vec<u32>)>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { t: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			for (k1, k2, v) in &self.t {
				<AppendableDM<T>>::insert(k1, k2, v);
			}
		}
	}
}

pub type BlockNumber = u32;
pub type Signature = sr25519::Signature;
pub type AccountId = <Signature as Verify>::Signer;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<u32, RuntimeCall, Signature, ()>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

topsoil_support::construct_runtime!(
	pub enum Test

	{
		System: topsoil_system,
		MyPallet: pallet,
	}
);

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Test {
	type BaseCallFilter = topsoil_support::traits::Everything;
	type Block = Block;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type PalletInfo = PalletInfo;
	type OnSetCode = ();
}

impl pallet::Config for Test {}

#[test]
fn init_genesis_config() {
	pallet::GenesisConfig::<Test>::default();
}
