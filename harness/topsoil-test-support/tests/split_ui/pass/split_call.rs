// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_core::pallet_macros::import_section;

mod call;

#[import_section(call::call)]
#[topsoil_core::pallet(dev_mode)]
pub mod pallet {
    use topsoil_core::pallet_prelude::*;
    use topsoil_core::system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: topsoil_core::system::Config {}
}

fn main() {
}
