// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_core::construct_runtime;

construct_runtime! {
	pub struct Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = Uxt,
	{
		System: topsoil_core::system::{Pallet, Call, Storage, Config<T>, Event<T>},
	}
}

fn main() {}
