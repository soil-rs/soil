// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_core::derive_impl;

mod common;

pub type Header = subsoil::runtime::generic::Header<u32, subsoil::runtime::traits::BlakeTwo256>;
pub type Block = subsoil::runtime::generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic =
	subsoil::runtime::generic::UncheckedExtrinsic<u32, RuntimeCall, (), ()>;

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Runtime {
	type BaseCallFilter = topsoil_core::traits::Everything;
	type Block = Block;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type PalletInfo = PalletInfo;
	type OnSetCode = ();
}

impl common::outer_enums::pallet::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}
impl common::outer_enums::pallet::Config<common::outer_enums::pallet::Instance1> for Runtime {
	type RuntimeEvent = RuntimeEvent;
}
impl common::outer_enums::pallet2::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}
impl common::outer_enums::pallet2::Config<common::outer_enums::pallet::Instance1> for Runtime {
	type RuntimeEvent = RuntimeEvent;
}
impl common::outer_enums::pallet3::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}
impl common::outer_enums::pallet3::Config<common::outer_enums::pallet::Instance1> for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

topsoil_core::construct_runtime!(
	pub struct Runtime
	{
		// Exclude part `Storage` in order not to check its metadata in tests.
		System: topsoil_core::system exclude_parts { Storage },

		// Pallet exposes `Error` implicitly.
		Example: common::outer_enums::pallet,
		Instance1Example: common::outer_enums::pallet::<Instance1>,

		// Pallet exposes `Error` implicitly.
		Example2: common::outer_enums::pallet2,
		Instance1Example2: common::outer_enums::pallet2::<Instance1>,

		// Pallet does not implement error.
		Example3: common::outer_enums::pallet3,
		Instance1Example3: common::outer_enums::pallet3::<Instance1>,
	}
);

#[cfg(feature = "experimental")]
#[test]
fn module_error_outer_enum_expand_implicit() {
	use common::outer_enums::{pallet, pallet2};
	// The Runtime has *all* parts implicitly defined.

	// Check that all error types are propagated
	match RuntimeError::Example(pallet::Error::InsufficientProposersBalance) {
		// Error passed implicitly to the pallet system.
		RuntimeError::System(system) => match system {
			topsoil_core::system::Error::InvalidSpecName => (),
			topsoil_core::system::Error::SpecVersionNeedsToIncrease => (),
			topsoil_core::system::Error::FailedToExtractRuntimeVersion => (),
			topsoil_core::system::Error::NonDefaultComposite => (),
			topsoil_core::system::Error::NonZeroRefCount => (),
			topsoil_core::system::Error::CallFiltered => (),
			topsoil_core::system::Error::MultiBlockMigrationsOngoing => (),
			topsoil_core::system::Error::InvalidTask => (),
			topsoil_core::system::Error::FailedTask => (),
			topsoil_core::system::Error::NothingAuthorized => (),
			topsoil_core::system::Error::Unauthorized => (),
			topsoil_core::system::Error::__Ignore(_, _) => (),
		},

		// Error declared explicitly.
		RuntimeError::Example(example) => match example {
			pallet::Error::InsufficientProposersBalance => (),
			pallet::Error::NonExistentStorageValue => (),
			pallet::Error::__Ignore(_, _) => (),
		},
		// Error declared explicitly.
		RuntimeError::Instance1Example(example) => match example {
			pallet::Error::InsufficientProposersBalance => (),
			pallet::Error::NonExistentStorageValue => (),
			pallet::Error::__Ignore(_, _) => (),
		},

		// Error must propagate even if not defined explicitly as pallet part.
		RuntimeError::Example2(example) => match example {
			pallet2::Error::OtherInsufficientProposersBalance => (),
			pallet2::Error::OtherNonExistentStorageValue => (),
			pallet2::Error::__Ignore(_, _) => (),
		},
		// Error must propagate even if not defined explicitly as pallet part.
		RuntimeError::Instance1Example2(example) => match example {
			pallet2::Error::OtherInsufficientProposersBalance => (),
			pallet2::Error::OtherNonExistentStorageValue => (),
			pallet2::Error::__Ignore(_, _) => (),
		},
	};
}
