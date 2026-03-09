// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Convenience trait for working with dynamic type of Imbalance.

use alloc::boxed::Box;

/// Unsafe imbalance cloning constructor and forgetful destructor.
///
/// This trait provides low-level operations that can violate imbalance invariants if misused.
/// These methods are separated into their own trait to make it explicit when unsafe operations
/// are being performed.
pub trait UnsafeConstructorDestructor<Balance> {
	/// Duplicates/clones the imbalance type, effectively leading to double accounting of the
	/// imbalance.
	///
	/// Warning: Use with care!!! one of the duplicates should call `self.forget_amount()` for the
	/// double-tracking to be removed.
	fn unsafe_clone(&self) -> Box<dyn ImbalanceAccounting<Balance>>;
	/// Forgets about the inner imbalance. Drops the inner imbalance without actually resolving it.
	/// Usually implemented by simply setting the imbalance amount to `zero`.
	///
	/// Note this is not equivalent `mem::forget()` as the destructor is still called, and memory is
	/// freed, but imbalance amount to resolve is zero/noop.
	///
	/// Returns the amount "forgotten".
	fn forget_imbalance(&mut self) -> Balance;
}

/// Unsafe manual accounting operations for imbalances.
///
/// This trait provides low-level operations that can violate imbalance invariants if misused.
/// These methods are separated into their own trait to make it explicit when unsafe operations
/// are being performed.
pub trait UnsafeManualAccounting<Balance> {
	/// Saturating add `other` imbalance to the inner imbalance.
	///
	/// The caller is responsible for making sure `self` and `other` are compatible concrete types.
	/// Compatible meaning both `self` and `other` imbalances are equivalent types with same
	/// imbalance resolution implementation.
	fn saturating_subsume(&mut self, other: Box<dyn ImbalanceAccounting<Balance>>);
}

/// Helper trait to be used for generic Imbalance, helpful for tracking multiple concrete types of
/// `Imbalance` using dynamic dispatch of this trait.
pub trait ImbalanceAccounting<Balance>:
	UnsafeConstructorDestructor<Balance> + UnsafeManualAccounting<Balance>
{
	/// Get inner imbalance amount.
	fn amount(&self) -> Balance;
	/// Saturating remove `amount` from the inner imbalance, and return it as a new imbalance
	/// instance.
	fn saturating_take(&mut self, amount: Balance) -> Box<dyn ImbalanceAccounting<Balance>>;
}
