// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_support::derive_impl;

mod common;

pub type Header = subsoil::runtime::generic::Header<u32, subsoil::runtime::traits::BlakeTwo256>;
pub type Block = subsoil::runtime::generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic =
	subsoil::runtime::generic::UncheckedExtrinsic<u32, RuntimeCall, (), ()>;

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Runtime {
	type BaseCallFilter = topsoil_support::traits::Everything;
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

topsoil_support::construct_runtime!(
	pub struct Runtime
	{
		// Exclude part `Storage` in order not to check its metadata in tests.
		System: topsoil_system::{Pallet, Config<T>, Call, Event<T> },

		// This pallet exposes the Error type explicitly.
		Example: common::outer_enums::pallet::{Pallet, Config<T>, Event<T>, Error<T>},
		Instance1Example: common::outer_enums::pallet::<Instance1>::{ Pallet, Config<T>, Event<T> },

		// This pallet does not mention the Error type, but it must be propagated (similarly to the polkadot/kusama).
		Example2: common::outer_enums::pallet2::{Pallet, Config<T>, Event<T> },
		Instance1Example2: common::outer_enums::pallet2::<Instance1>::{Pallet, Config<T>, Event<T>},

		// This pallet does not declare any errors.
		Example3: common::outer_enums::pallet3::{Pallet, Config<T>, Event<T>},
		Instance1Example3: common::outer_enums::pallet3::<Instance1>::{Pallet, Config<T>, Event<T>},
	}
);

#[cfg(feature = "experimental")]
#[test]
fn module_error_outer_enum_expand_explicit() {
	use common::outer_enums::{pallet, pallet2};
	// The Runtime has *all* parts explicitly defined.

	// Check that all error types are propagated
	match RuntimeError::Example(pallet::Error::InsufficientProposersBalance) {
		// Error passed implicitly to the pallet system.
		RuntimeError::System(system) => match system {
			topsoil_system::Error::InvalidSpecName => (),
			topsoil_system::Error::SpecVersionNeedsToIncrease => (),
			topsoil_system::Error::FailedToExtractRuntimeVersion => (),
			topsoil_system::Error::NonDefaultComposite => (),
			topsoil_system::Error::NonZeroRefCount => (),
			topsoil_system::Error::CallFiltered => (),
			topsoil_system::Error::MultiBlockMigrationsOngoing => (),
			topsoil_system::Error::InvalidTask => (),
			topsoil_system::Error::FailedTask => (),
			topsoil_system::Error::NothingAuthorized => (),
			topsoil_system::Error::Unauthorized => (),
			topsoil_system::Error::__Ignore(_, _) => (),
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
