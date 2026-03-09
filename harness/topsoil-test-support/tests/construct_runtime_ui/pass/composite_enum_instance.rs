// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg_attr(not(feature = "std"), no_std)]

use topsoil_core::derive_impl;

pub use pallet::*;

#[topsoil_core::pallet(dev_mode)]
pub mod pallet {
	use topsoil_core::pallet_prelude::*;

	// The struct on which we build all of our Pallet logic.
	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

	// Your Pallet's configuration trait, representing custom external types and interfaces.
	#[pallet::config]
	pub trait Config<I: 'static = ()>: topsoil_core::system::Config {}
	
	#[pallet::composite_enum]
	pub enum HoldReason<I: 'static = ()> {
		SomeHoldReason
	}

	#[pallet::composite_enum]
	pub enum FreezeReason<I: 'static = ()> {
		SomeFreezeReason
	}

	#[pallet::composite_enum]
	pub enum SlashReason<I: 'static = ()> {
		SomeSlashReason
	}

	#[pallet::composite_enum]
	pub enum LockId<I: 'static = ()> {
		SomeLockId
	}
}

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Runtime {
	type Block = Block;
}

pub type Header = subsoil::runtime::generic::Header<u64, subsoil::runtime::traits::BlakeTwo256>;
pub type UncheckedExtrinsic = subsoil::runtime::generic::UncheckedExtrinsic<u64, RuntimeCall, (), ()>;
pub type Block = subsoil::runtime::generic::Block<Header, UncheckedExtrinsic>;

topsoil_core::construct_runtime!(
	pub enum Runtime
	{
		// Exclude part `Storage` in order not to check its metadata in tests.
		System: topsoil_core::system,
		Pallet1: pallet,
		Pallet2: pallet::<Instance2>,
	}
);

impl pallet::Config for Runtime {}

impl pallet::Config<pallet::Instance2> for Runtime {}

fn main() {}
