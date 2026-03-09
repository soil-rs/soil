// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_core::derive_impl;

pub type Block = topsoil_core::system::mocking::MockBlock<Runtime>;

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig as topsoil_core::system::DefaultConfig)]
impl topsoil_core::system::Config for Runtime {
	type Block = Block;
}

#[topsoil_core::runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(RuntimeCall, RuntimeEvent, RuntimeOrigin, RuntimeError, RuntimeTask, RuntimeViewFunction)]
    pub struct Runtime;

    #[runtime::pallet_index(0)]
    pub type System = topsoil_core::system;
}

fn main() {}
