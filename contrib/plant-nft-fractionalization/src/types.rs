// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Various basic types for use in the Nft fractionalization pallet.

use super::*;
use codec::{Decode, Encode, MaxEncodedLen};
use fungible::Inspect as FunInspect;
use fungibles::Inspect;
use scale_info::TypeInfo;

pub type AssetIdOf<T> = <<T as Config>::Assets as Inspect<<T as SystemConfig>::AccountId>>::AssetId;
pub type AssetBalanceOf<T> =
	<<T as Config>::Assets as Inspect<<T as SystemConfig>::AccountId>>::Balance;
pub type DepositOf<T> =
	<<T as Config>::Currency as FunInspect<<T as SystemConfig>::AccountId>>::Balance;
pub type AccountIdLookupOf<T> = <<T as SystemConfig>::Lookup as StaticLookup>::Source;

/// Stores the details of a fractionalized item.
#[derive(Decode, Encode, Default, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct Details<AssetId, Fractions, Deposit, AccountId> {
	/// Minted asset.
	pub asset: AssetId,

	/// Number of fractions minted.
	pub fractions: Fractions,

	/// Reserved deposit for creating a new asset.
	pub deposit: Deposit,

	/// Account that fractionalized an item.
	pub asset_creator: AccountId,
}

/// Benchmark Helper
#[cfg(feature = "runtime-benchmarks")]
pub trait BenchmarkHelper<AssetId, CollectionId, ItemId> {
	/// Returns an asset id from a given integer.
	fn asset(id: u32) -> AssetId;
	/// Returns a collection id from a given integer.
	fn collection(id: u32) -> CollectionId;
	/// Returns an nft id from a given integer.
	fn nft(id: u32) -> ItemId;
}

#[cfg(feature = "runtime-benchmarks")]
impl<AssetId, CollectionId, ItemId> BenchmarkHelper<AssetId, CollectionId, ItemId> for ()
where
	AssetId: From<u32>,
	CollectionId: From<u32>,
	ItemId: From<u32>,
{
	fn asset(id: u32) -> AssetId {
		id.into()
	}
	fn collection(id: u32) -> CollectionId {
		id.into()
	}
	fn nft(id: u32) -> ItemId {
		id.into()
	}
}
