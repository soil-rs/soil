// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_core::runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(RuntimeCall)]
    pub struct Runtime;

    #[runtime::pallet_index("0")]
    pub type System = topsoil_core::system;
}

fn main() {}
