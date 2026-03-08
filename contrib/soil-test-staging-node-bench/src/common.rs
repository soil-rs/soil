// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

#[derive(Clone, Copy, Debug, derive_more::Display)]
pub enum SizeType {
	#[display(fmt = "empty")]
	Empty,
	#[display(fmt = "small")]
	Small,
	#[display(fmt = "medium")]
	Medium,
	#[display(fmt = "large")]
	Large,
	#[display(fmt = "full")]
	Full,
	#[display(fmt = "custom")]
	Custom(usize),
}

impl SizeType {
	pub fn transactions(&self) -> Option<usize> {
		match self {
			SizeType::Empty => Some(0),
			SizeType::Small => Some(10),
			SizeType::Medium => Some(100),
			SizeType::Large => Some(500),
			SizeType::Full => None,
			// Custom SizeType will use the `--transactions` input parameter
			SizeType::Custom(val) => Some(*val),
		}
	}
}
