// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Inspect and Mutate traits for Asset metadata
//!
//! See the [`crate::traits::fungibles`] doc for more information about fungibles traits.

use crate::dispatch::DispatchResult;
use alloc::vec::Vec;

pub trait Inspect<AccountId>: super::Inspect<AccountId> {
	// Get name for an AssetId.
	fn name(asset: Self::AssetId) -> Vec<u8>;
	// Get symbol for an AssetId.
	fn symbol(asset: Self::AssetId) -> Vec<u8>;
	// Get decimals for an AssetId.
	fn decimals(asset: Self::AssetId) -> u8;
}

pub trait Mutate<AccountId>: Inspect<AccountId> {
	// Set name, symbol and decimals for a given assetId.
	fn set(
		asset: Self::AssetId,
		from: &AccountId,
		name: Vec<u8>,
		symbol: Vec<u8>,
		decimals: u8,
	) -> DispatchResult;
}

pub trait MetadataDeposit<DepositBalance> {
	// Returns the required deposit amount for a given metadata.
	fn calc_metadata_deposit(name: &[u8], symbol: &[u8]) -> DepositBalance;
}
