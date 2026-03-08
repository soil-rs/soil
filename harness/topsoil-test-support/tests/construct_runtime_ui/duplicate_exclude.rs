// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_support::construct_runtime;

construct_runtime! {
	pub struct Runtime where
		UncheckedExtrinsic = UncheckedExtrinsic,
		Block = Block,
		NodeBlock = Block,
	{
		System: topsoil_system exclude_parts { Call, Call },
	}
}

fn main() {}
