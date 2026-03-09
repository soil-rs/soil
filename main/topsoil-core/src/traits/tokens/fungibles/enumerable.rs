// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Contains an interface for enumerating assets in existence or owned by a given account.
//!
//! See the [`crate::traits::fungibles`] doc for more information about fungibles traits.

/// Interface for enumerating assets in existence or owned by a given account.
pub trait Inspect<AccountId>: super::Inspect<AccountId> {
	type AssetsIterator;

	/// Returns an iterator of the collections in existence.
	fn asset_ids() -> Self::AssetsIterator;
}
