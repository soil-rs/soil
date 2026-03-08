// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_support::pallet_macros::import_section;

mod storage;

#[import_section(storage::storage)]
#[topsoil_support::pallet(dev_mode)]
pub mod pallet {
    use topsoil_support::pallet_prelude::*;
    use topsoil_system::pallet_prelude::*;

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(8);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: topsoil_system::Config {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn increment_value(_origin: OriginFor<T>) -> DispatchResult {
			Value::<T>::mutate(|v| {
				v.saturating_add(1)
			});
			Ok(())
		}
	}
}

fn main() {
}
