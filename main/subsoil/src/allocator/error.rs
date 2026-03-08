// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

/// The error type used by the allocators.
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
	/// Someone tried to allocate more memory than the allowed maximum per allocation.
	#[error("Requested allocation size is too large")]
	RequestedAllocationTooLarge,
	/// Allocator run out of space.
	#[error("Allocator ran out of space")]
	AllocatorOutOfSpace,
	/// The client passed a memory instance which is smaller than previously observed.
	#[error("Shrinking of the underlying memory is observed")]
	MemoryShrinked,
	/// Some other error occurred.
	#[error("Other: {0}")]
	Other(&'static str),
}
