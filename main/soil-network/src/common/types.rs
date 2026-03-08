// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

/// Description of a reputation adjustment for a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReputationChange {
	/// Reputation delta.
	pub value: i32,
	/// Reason for reputation change.
	pub reason: &'static str,
}

impl ReputationChange {
	/// New reputation change with given delta and reason.
	pub const fn new(value: i32, reason: &'static str) -> ReputationChange {
		Self { value, reason }
	}

	/// New reputation change that forces minimum possible reputation.
	pub const fn new_fatal(reason: &'static str) -> ReputationChange {
		Self { value: i32::MIN, reason }
	}
}
