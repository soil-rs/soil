// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![allow(deprecated, clippy::deprecated_semver)]

use super::Block;
use crate::derive_impl;

#[crate::pallet(dev_mode)]
mod pallet_basic {
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {}
}

impl pallet_basic::Config for Runtime {}

#[crate::pallet(dev_mode)]
mod pallet_with_disabled_call {
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {}
}

impl pallet_with_disabled_call::Config for Runtime {}

#[crate::pallet(dev_mode)]
mod pallet_with_disabled_unsigned {
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {}
}

impl pallet_with_disabled_unsigned::Config for Runtime {}

#[crate::pallet]
mod pallet_with_instance {
	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(_);

	#[pallet::config]
	pub trait Config<I: 'static = ()>: topsoil_core::system::Config {}
}

#[allow(unused)]
type Instance1 = pallet_with_instance::Pallet<pallet_with_instance::Instance1>;

impl pallet_with_instance::Config<pallet_with_instance::Instance1> for Runtime {}

#[allow(unused)]
type Instance2 = pallet_with_instance::Pallet<pallet_with_instance::Instance2>;

impl pallet_with_instance::Config<pallet_with_instance::Instance2> for Runtime {}

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Runtime {
	type Block = Block;
}

#[docify::export(runtime_macro)]
#[crate::runtime]
mod runtime {
	// The main runtime
	#[runtime::runtime]
	// Runtime Types to be generated
	#[runtime::derive(
		RuntimeCall,
		RuntimeEvent,
		RuntimeError,
		RuntimeOrigin,
		RuntimeFreezeReason,
		RuntimeHoldReason,
		RuntimeSlashReason,
		RuntimeLockId,
		RuntimeTask,
		RuntimeViewFunction
	)]
	pub struct Runtime;

	// Use the concrete pallet type
	#[runtime::pallet_index(0)]
	pub type System = topsoil_core::system::Pallet<Runtime>;

	// Use path to the pallet
	#[runtime::pallet_index(1)]
	pub type Basic = pallet_basic;

	// Use the concrete pallet type with instance
	#[runtime::pallet_index(2)]
	pub type PalletWithInstance1 = pallet_with_instance::Pallet<Runtime, Instance1>;

	// Use path to the pallet with instance
	#[runtime::pallet_index(3)]
	pub type PalletWithInstance2 = pallet_with_instance<Instance2>;

	// Ensure that the runtime does not export the calls from the pallet
	#[runtime::pallet_index(4)]
	#[runtime::disable_call]
	#[deprecated = "example"]
	pub type PalletWithDisabledCall = pallet_with_disabled_call::Pallet<Runtime>;

	// Ensure that the runtime does not export the unsigned calls from the pallet
	#[runtime::pallet_index(5)]
	#[runtime::disable_unsigned]
	pub type PalletWithDisabledUnsigned = pallet_with_disabled_unsigned::Pallet<Runtime>;
}
