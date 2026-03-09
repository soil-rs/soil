// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Provide types to help defining a mock environment when testing pallets.

use subsoil::runtime::generic;

/// An unchecked extrinsic type to be used in tests.
pub type MockUncheckedExtrinsic<T, Signature = (), Extra = ()> = generic::UncheckedExtrinsic<
	<T as crate::system::Config>::AccountId,
	<T as crate::system::Config>::RuntimeCall,
	Signature,
	Extra,
>;

/// An implementation of `subsoil::runtime::traits::Block` to be used in tests.
pub type MockBlock<T> = generic::Block<
	generic::Header<u64, subsoil::runtime::traits::BlakeTwo256>,
	MockUncheckedExtrinsic<T>,
>;

/// An implementation of `subsoil::runtime::traits::Block` to be used in tests with u32 BlockNumber type.
pub type MockBlockU32<T> = generic::Block<
	generic::Header<u32, subsoil::runtime::traits::BlakeTwo256>,
	MockUncheckedExtrinsic<T>,
>;

/// An implementation of `subsoil::runtime::traits::Block` to be used in tests with u128 BlockNumber
/// type.
pub type MockBlockU128<T> = generic::Block<
	generic::Header<u128, subsoil::runtime::traits::BlakeTwo256>,
	MockUncheckedExtrinsic<T>,
>;
