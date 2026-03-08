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
		System: topsoil_system exclude_parts { Storage },

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
