// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_support::construct_runtime;
use subsoil::runtime::{generic, traits::BlakeTwo256};
use subsoil::core::sr25519;

#[topsoil_support::pallet]
mod pallet {
	#[pallet::config]
	pub trait Config: topsoil_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(_);
}

pub type Signature = sr25519::Signature;
pub type BlockNumber = u64;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<u32, RuntimeCall, Signature, ()>;

impl pallet::Config for Runtime {}

construct_runtime! {
	pub struct Runtime
	{
		System: system::{Pallet, Call, Storage, Config<T>, Event<T>},
		Pallet: pallet exclude_parts { Pallet } use_parts { Pallet },
	}
}

fn main() {}
