// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[cfg(test)]
use super::{Config, Runtime};
#[cfg(test)]
use topsoil_support::{derive_impl, pallet_prelude::inject_runtime_type};
#[cfg(test)]
use static_assertions::assert_type_eq_all;

#[docify::export]
#[test]
fn derive_impl_works_with_runtime_type_injection() {
	assert_type_eq_all!(<Runtime as Config>::RuntimeOrigin, super::RuntimeOrigin);
	assert_type_eq_all!(<Runtime as Config>::RuntimeCall, super::RuntimeCall);
	assert_type_eq_all!(<Runtime as Config>::PalletInfo, super::PalletInfo);
}

#[docify::export]
#[test]
fn derive_impl_works_with_no_aggregated_types() {
	struct DummyRuntime;

	#[derive_impl(
        super::topsoil_system::config_preludes::TestDefaultConfig as super::topsoil_system::DefaultConfig,
        no_aggregated_types
    )]
	impl Config for DummyRuntime {
		type Block = super::Block;
		type AccountId = super::AccountId;
		type PalletInfo = super::PalletInfo;
	}

	assert_type_eq_all!(<DummyRuntime as Config>::RuntimeOrigin, ());
	assert_type_eq_all!(<DummyRuntime as Config>::RuntimeCall, ());
}
