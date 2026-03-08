// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_support::{*, pallet_prelude::inject_runtime_type};
use static_assertions::assert_type_eq_all;

pub trait Config {
    type RuntimeInfo;
}

type RuntimeInfo = u32;

struct Pallet;

#[register_default_impl(Pallet)]
impl Config for Pallet {
    #[inject_runtime_type]
    type RuntimeInfo = ();
}

struct SomePallet;

#[derive_impl(Pallet)] // Injects type RuntimeInfo = RuntimeInfo;
impl Config for SomePallet {}

assert_type_eq_all!(<SomePallet as Config>::RuntimeInfo, u32);

fn main() {}
