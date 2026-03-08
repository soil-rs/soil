// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Types used in the pallet.

use crate::{Config, CreditOf, Event, Pallet};
use codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use core::ops::BitOr;
use scale_info::TypeInfo;
use subsoil::runtime::Saturating;
use topsoil_support::traits::{Imbalance, LockIdentifier, OnUnbalanced, WithdrawReasons};

/// Simplified reasons for withdrawing balance.
#[derive(
	Encode,
	Decode,
	DecodeWithMemTracking,
	Clone,
	Copy,
	PartialEq,
	Eq,
	Debug,
	MaxEncodedLen,
	TypeInfo,
)]
pub enum Reasons {
	/// Paying system transaction fees.
	Fee = 0,
	/// Any reason other than paying system transaction fees.
	Misc = 1,
	/// Any reason at all.
	All = 2,
}

impl From<WithdrawReasons> for Reasons {
	fn from(r: WithdrawReasons) -> Reasons {
		if r == WithdrawReasons::TRANSACTION_PAYMENT {
			Reasons::Fee
		} else if r.contains(WithdrawReasons::TRANSACTION_PAYMENT) {
			Reasons::All
		} else {
			Reasons::Misc
		}
	}
}

impl BitOr for Reasons {
	type Output = Reasons;
	fn bitor(self, other: Reasons) -> Reasons {
		if self == other {
			return self;
		}
		Reasons::All
	}
}

/// A single lock on a balance. There can be many of these on an account and they "overlap", so the
/// same balance is frozen by multiple locks.
#[derive(
	Encode, Decode, DecodeWithMemTracking, Clone, PartialEq, Eq, Debug, MaxEncodedLen, TypeInfo,
)]
pub struct BalanceLock<Balance> {
	/// An identifier for this lock. Only one lock may be in existence for each identifier.
	pub id: LockIdentifier,
	/// The amount which the free balance may not drop below when this lock is in effect.
	pub amount: Balance,
	/// If true, then the lock remains in effect even for payment of transaction fees.
	pub reasons: Reasons,
}

/// Store named reserved balance.
#[derive(
	Encode, Decode, DecodeWithMemTracking, Clone, PartialEq, Eq, Debug, MaxEncodedLen, TypeInfo,
)]
pub struct ReserveData<ReserveIdentifier, Balance> {
	/// The identifier for the named reserve.
	pub id: ReserveIdentifier,
	/// The amount of the named reserve.
	pub amount: Balance,
}

/// All balance information for an account.
#[derive(
	Encode,
	Decode,
	DecodeWithMemTracking,
	Clone,
	PartialEq,
	Eq,
	Default,
	Debug,
	MaxEncodedLen,
	TypeInfo,
)]
pub struct AccountData<Balance> {
	/// Non-reserved part of the balance which the account holder may be able to control.
	///
	/// This is the only balance that matters in terms of most operations on tokens.
	pub free: Balance,
	/// Balance which is has active holds on it and may not be used at all.
	///
	/// This is the sum of all individual holds together with any sums still under the (deprecated)
	/// reserves API.
	pub reserved: Balance,
	/// The amount that `free + reserved` may not drop below when reducing the balance, except for
	/// actions where the account owner cannot reasonably benefit from the balance reduction, such
	/// as slashing.
	pub frozen: Balance,
	/// Extra information about this account. The MSB is a flag indicating whether the new ref-
	/// counting logic is in place for this account.
	pub flags: ExtraFlags,
}

const IS_NEW_LOGIC: u128 = 0x80000000_00000000_00000000_00000000u128;

#[derive(
	Encode, Decode, DecodeWithMemTracking, Clone, PartialEq, Eq, Debug, MaxEncodedLen, TypeInfo,
)]
pub struct ExtraFlags(pub u128);
impl Default for ExtraFlags {
	fn default() -> Self {
		Self(IS_NEW_LOGIC)
	}
}
impl ExtraFlags {
	pub fn old_logic() -> Self {
		Self(0)
	}
	pub fn set_new_logic(&mut self) {
		self.0 = self.0 | IS_NEW_LOGIC
	}
	pub fn is_new_logic(&self) -> bool {
		(self.0 & IS_NEW_LOGIC) == IS_NEW_LOGIC
	}
}

impl<Balance: Saturating + Copy + Ord> AccountData<Balance> {
	pub fn usable(&self) -> Balance {
		self.free.saturating_sub(self.frozen)
	}

	/// The total balance in this account including any that is reserved and ignoring any frozen.
	pub fn total(&self) -> Balance {
		self.free.saturating_add(self.reserved)
	}
}

pub struct DustCleaner<T: Config<I>, I: 'static = ()>(
	pub(crate) Option<(T::AccountId, CreditOf<T, I>)>,
);

impl<T: Config<I>, I: 'static> Drop for DustCleaner<T, I> {
	fn drop(&mut self) {
		if let Some((who, dust)) = self.0.take() {
			Pallet::<T, I>::deposit_event(Event::DustLost { account: who, amount: dust.peek() });
			T::DustRemoval::on_unbalanced(dust);
		}
	}
}

/// Whether something should be interpreted as an increase or a decrease.
#[derive(
	Encode, Decode, DecodeWithMemTracking, Clone, PartialEq, Eq, Debug, MaxEncodedLen, TypeInfo,
)]
pub enum AdjustmentDirection {
	/// Increase the amount.
	Increase,
	/// Decrease the amount.
	Decrease,
}
