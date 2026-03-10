// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

pub use polkavm_derive::{polkavm_export, polkavm_import};

#[polkavm_derive::polkavm_define_abi(allow_extra_input_registers)]
pub mod polkavm_abi {}

impl self::polkavm_abi::FromHost for *mut u8 {
	type Regs = (u64,);

	#[inline]
	fn from_host((value,): Self::Regs) -> Self {
		value as *mut u8
	}
}
