// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Inspect traits for Asset roles
//!
//! See the [`crate::traits::fungibles`] doc for more information about fungibles traits.

use subsoil::runtime::DispatchResult;

pub trait Inspect<AccountId>: super::Inspect<AccountId> {
	// Get owner for an AssetId.
	fn owner(asset: Self::AssetId) -> Option<AccountId>;
	// Get issuer for an AssetId.
	fn issuer(asset: Self::AssetId) -> Option<AccountId>;
	// Get admin for an AssetId.
	fn admin(asset: Self::AssetId) -> Option<AccountId>;
	// Get freezer for an AssetId.
	fn freezer(asset: Self::AssetId) -> Option<AccountId>;
}

/// Trait for resetting the team configuration of an existing fungible asset.
pub trait ResetTeam<AccountId>: super::Inspect<AccountId> {
	/// Reset the team for the asset with the given `id`.
	///
	/// ### Parameters
	/// - `id`: The identifier of the asset for which the team is being reset.
	/// - `owner`: The new `owner` account for the asset.
	/// - `admin`: The new `admin` account for the asset.
	/// - `issuer`: The new `issuer` account for the asset.
	/// - `freezer`: The new `freezer` account for the asset.
	fn reset_team(
		id: Self::AssetId,
		owner: AccountId,
		admin: AccountId,
		issuer: AccountId,
		freezer: AccountId,
	) -> DispatchResult;
}
