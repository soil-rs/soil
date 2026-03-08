// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! MultiAddress type is a wrapper for multiple downstream account formats.

use alloc::vec::Vec;
use codec::{Decode, DecodeWithMemTracking, Encode};

/// A multi-format address wrapper for on-chain accounts.
#[derive(
	Encode, Decode, DecodeWithMemTracking, PartialEq, Eq, Clone, Debug, scale_info::TypeInfo,
)]
#[cfg_attr(feature = "std", derive(Hash))]
pub enum MultiAddress<AccountId, AccountIndex> {
	/// It's an account ID (pubkey).
	Id(AccountId),
	/// It's an account index.
	Index(#[codec(compact)] AccountIndex),
	/// It's some arbitrary raw bytes.
	Raw(Vec<u8>),
	/// It's a 32 byte representation.
	Address32([u8; 32]),
	/// It's a 20 byte representation.
	Address20([u8; 20]),
}

#[cfg(feature = "std")]
impl<AccountId, AccountIndex> ::std::fmt::Display for MultiAddress<AccountId, AccountIndex>
where
	AccountId: ::std::fmt::Debug,
	AccountIndex: ::std::fmt::Debug,
{
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		use crate::core::hexdisplay::HexDisplay;
		match self {
			Self::Raw(inner) => write!(f, "MultiAddress::Raw({})", HexDisplay::from(inner)),
			Self::Address32(inner) => {
				write!(f, "MultiAddress::Address32({})", HexDisplay::from(inner))
			},
			Self::Address20(inner) => {
				write!(f, "MultiAddress::Address20({})", HexDisplay::from(inner))
			},
			_ => write!(f, "{:?}", self),
		}
	}
}

impl<AccountId, AccountIndex> From<AccountId> for MultiAddress<AccountId, AccountIndex> {
	fn from(a: AccountId) -> Self {
		Self::Id(a)
	}
}
