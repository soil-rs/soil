// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_core::pallet_macros::pallet_section;

#[pallet_section]
mod call {
	#[pallet::call]
	impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        pub fn noop0(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;
            Ok(())
        }

        #[pallet::call_index(1)]
        pub fn noop1(origin: OriginFor<T>, _x: u64) -> DispatchResult {
            ensure_signed(origin)?;
            Ok(())
        }

        #[pallet::call_index(2)]
        pub fn noop2(origin: OriginFor<T>, _x: u64, _y: u64) -> DispatchResult {
            ensure_signed(origin)?;
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::feeless_if(|_origin: &OriginFor<T>| -> bool { true })]
        pub fn noop_feeless0(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;
            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::feeless_if(|_origin: &OriginFor<T>, x: &u64| -> bool { *x == 1 })]
        pub fn noop_feeless1(origin: OriginFor<T>, _x: u64) -> DispatchResult {
            ensure_signed(origin)?;
            Ok(())
        }

        #[pallet::call_index(5)]
        #[pallet::feeless_if(|_origin: &OriginFor<T>, x: &u64, y: &u64| -> bool { *x == *y })]
        pub fn noop_feeless2(origin: OriginFor<T>, _x: u64, _y: u64) -> DispatchResult {
            ensure_signed(origin)?;
            Ok(())
        }

        #[pallet::call_index(6)]
        #[pallet::authorize(|_source, _x, _y| Ok(Default::default()))]
        pub fn noop_authorize(origin: OriginFor<T>, _x: u64, _y: u64) -> DispatchResult {
            ensure_authorized(origin)?;
            Ok(())
        }
	}
}
