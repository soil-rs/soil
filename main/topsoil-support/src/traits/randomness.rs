// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Traits for dealing with on-chain randomness.

/// A trait that is able to provide randomness.
///
/// Being a deterministic blockchain, real randomness is difficult to come by, different
/// implementations of this trait will provide different security guarantees. At best,
/// this will be randomness which was hard to predict a long time ago, but that has become
/// easy to predict recently.
pub trait Randomness<Output, BlockNumber> {
	/// Get the most recently determined random seed, along with the time in the past
	/// since when it was determinable by chain observers.
	///
	/// `subject` is a context identifier and allows you to get a different result to
	/// other callers of this function; use it like `random(&b"my context"[..])`.
	///
	/// NOTE: The returned seed should only be used to distinguish commitments made before
	/// the returned block number. If the block number is too early (i.e. commitments were
	/// made afterwards), then ensure no further commitments may be made and repeatedly
	/// call this on later blocks until the block number returned is later than the latest
	/// commitment.
	fn random(subject: &[u8]) -> (Output, BlockNumber);

	/// Get the basic random seed.
	///
	/// In general you won't want to use this, but rather `Self::random` which allows
	/// you to give a subject for the random result and whose value will be
	/// independently low-influence random from any other such seeds.
	///
	/// NOTE: The returned seed should only be used to distinguish commitments made before
	/// the returned block number. If the block number is too early (i.e. commitments were
	/// made afterwards), then ensure no further commitments may be made and repeatedly
	/// call this on later blocks until the block number returned is later than the latest
	/// commitment.
	fn random_seed() -> (Output, BlockNumber) {
		Self::random(&[][..])
	}
}
