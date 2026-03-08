// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test that `construct_runtime!` also works when `topsoil-support` or `topsoil-system` are renamed in
//! the `Cargo.toml`.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use subsoil::core::{sr25519, H256};
use subsoil::runtime::{
	generic,
	traits::{BlakeTwo256, IdentityLookup, Verify},
};
use subsoil::version::RuntimeVersion;
use topsoil_support::{
	construct_runtime, derive_impl, parameter_types,
	traits::{ConstU16, ConstU32, ConstU64, Everything},
};

pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: alloc::borrow::Cow::Borrowed("topsoil-test-support-compile-pass"),
	impl_name: alloc::borrow::Cow::Borrowed("substrate-topsoil-test-support-compile-pass-runtime"),
	authoring_version: 0,
	spec_version: 0,
	impl_version: 0,
	apis: subsoil::create_apis_vec!([]),
	transaction_version: 0,
	system_version: 0,
};

pub type Signature = sr25519::Signature;
pub type AccountId = <Signature as Verify>::Signer;
pub type BlockNumber = u64;

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
}

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Runtime {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type Nonce = u128;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Block = Block;
	type Lookup = IdentityLookup<Self::AccountId>;
	type BlockHashCount = ConstU64<2400>;
	type Version = Version;
	type AccountData = ();
	type RuntimeOrigin = RuntimeOrigin;
	type AccountId = AccountId;
	type RuntimeEvent = RuntimeEvent;
	type PalletInfo = PalletInfo;
	type RuntimeCall = RuntimeCall;
	type DbWeight = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<0>;
}

pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<u32, RuntimeCall, Signature, ()>;

construct_runtime!(
	pub enum Runtime {
		System: topsoil_system,
	}
);
