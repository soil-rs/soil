// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Convenience type for managing an imbalance whose sign is unknown.

use super::super::imbalance::Imbalance;
use crate::traits::misc::SameOrOther;
use codec::FullCodec;
use core::fmt::Debug;
use subsoil::runtime::traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize};

/// Either a positive or a negative imbalance.
pub enum SignedImbalance<B, PositiveImbalance: Imbalance<B>> {
	/// A positive imbalance (funds have been created but none destroyed).
	Positive(PositiveImbalance),
	/// A negative imbalance (funds have been destroyed but none created).
	Negative(PositiveImbalance::Opposite),
}

impl<
		P: Imbalance<B, Opposite = N>,
		N: Imbalance<B, Opposite = P>,
		B: AtLeast32BitUnsigned + FullCodec + Copy + MaybeSerializeDeserialize + Debug + Default,
	> SignedImbalance<B, P>
{
	/// Create a `Positive` instance of `Self` whose value is zero.
	pub fn zero() -> Self {
		SignedImbalance::Positive(P::zero())
	}

	/// Drop `Self` if and only if it is equal to zero. Return `Err` with `Self` if not.
	pub fn drop_zero(self) -> Result<(), Self> {
		match self {
			SignedImbalance::Positive(x) => x.drop_zero().map_err(SignedImbalance::Positive),
			SignedImbalance::Negative(x) => x.drop_zero().map_err(SignedImbalance::Negative),
		}
	}

	/// Consume `self` and an `other` to return a new instance that combines
	/// both.
	pub fn merge(self, other: Self) -> Self {
		match (self, other) {
			(SignedImbalance::Positive(one), SignedImbalance::Positive(other)) => {
				SignedImbalance::Positive(one.merge(other))
			},
			(SignedImbalance::Negative(one), SignedImbalance::Negative(other)) => {
				SignedImbalance::Negative(one.merge(other))
			},
			(SignedImbalance::Positive(one), SignedImbalance::Negative(other)) => {
				match one.offset(other) {
					SameOrOther::Same(positive) => SignedImbalance::Positive(positive),
					SameOrOther::Other(negative) => SignedImbalance::Negative(negative),
					SameOrOther::None => SignedImbalance::Positive(P::zero()),
				}
			},
			(one, other) => other.merge(one),
		}
	}
}
