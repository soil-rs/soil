// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Produce opaque sequential IDs.

/// A Sequence of IDs.
#[derive(Debug, Default)]
// The `Clone` trait is intentionally not defined on this type.
pub struct IDSequence {
	next_id: u64,
}

/// A Sequential ID.
///
/// Its integer value is intentionally not public: it is supposed to be instantiated from within
/// this module only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SeqID(u64);

impl std::fmt::Display for SeqID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl IDSequence {
	/// Create a new ID-sequence.
	pub fn new() -> Self {
		Default::default()
	}

	/// Obtain another ID from this sequence.
	pub fn next_id(&mut self) -> SeqID {
		let id = SeqID(self.next_id);
		self.next_id += 1;

		id
	}
}

impl Into<u64> for SeqID {
	fn into(self) -> u64 {
		self.0
	}
}

impl From<u64> for SeqID {
	fn from(value: u64) -> Self {
		SeqID(value)
	}
}
