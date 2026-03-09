// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_core::construct_runtime;

construct_runtime! {
	pub struct Runtime where
		UncheckedExtrinsic = UncheckedExtrinsic,
		Block = Block,
		NodeBlock = Block,
	{
		System: system::{} = 5,
		Pallet1: pallet1::{} = 3,
		Pallet2: pallet2::{},
		Pallet3: pallet3::{},
	}
}

fn main() {}
