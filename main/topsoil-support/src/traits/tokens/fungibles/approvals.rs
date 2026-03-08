// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Inspect and Mutate traits for Asset approvals
//!
//! See the [`crate::traits::fungibles`] doc for more information about fungibles traits.

use crate::dispatch::DispatchResult;
pub trait Inspect<AccountId>: super::Inspect<AccountId> {
	// Check the amount approved by an owner to be spent by a delegate
	fn allowance(asset: Self::AssetId, owner: &AccountId, delegate: &AccountId) -> Self::Balance;
}

pub trait Mutate<AccountId>: Inspect<AccountId> {
	// Approve a delegate account to spend an amount of tokens owned by an owner
	fn approve(
		asset: Self::AssetId,
		owner: &AccountId,
		delegate: &AccountId,
		amount: Self::Balance,
	) -> DispatchResult;

	// Transfer from a delegate account an amount approved by the owner of the asset
	fn transfer_from(
		asset: Self::AssetId,
		owner: &AccountId,
		delegate: &AccountId,
		dest: &AccountId,
		amount: Self::Balance,
	) -> DispatchResult;
}
