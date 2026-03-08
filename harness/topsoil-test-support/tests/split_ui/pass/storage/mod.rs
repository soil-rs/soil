// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_support::pallet_macros::pallet_section;

#[pallet_section]
mod storage {
	#[pallet::storage]
	pub type Value<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	pub type Map<T> = StorageMap<_, _, u32, u32, ValueQuery>;
}
