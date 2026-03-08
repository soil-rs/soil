// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Traits for the npos-election operations.

use super::ExtendedBalance;
use crate::arithmetic::PerThing;
use core::{fmt::Debug, ops::Mul};

/// an aggregator trait for a generic type of a voter/target identifier. This usually maps to
/// substrate's account id.
pub trait IdentifierT: Clone + Eq + Ord + Debug + codec::Codec {}
impl<T: Clone + Eq + Ord + Debug + codec::Codec> IdentifierT for T {}

/// Aggregator trait for a PerThing that can be multiplied by u128 (ExtendedBalance).
pub trait PerThing128: PerThing + Mul<ExtendedBalance, Output = ExtendedBalance> {}
impl<T: PerThing + Mul<ExtendedBalance, Output = ExtendedBalance>> PerThing128 for T {}
