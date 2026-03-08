// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_support::pallet]
mod pallet {
    #[pallet::config]
    pub trait Config: topsoil_system::Config {}

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {}
}

#[topsoil_support::runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(RuntimeCall)]
    pub struct Runtime;

    #[runtime::pallet_index(0)]
    pub type System = topsoil_system;

    #[runtime::pallet_index(0)]
    pub type Pallet = pallet;
}

fn main() {}
