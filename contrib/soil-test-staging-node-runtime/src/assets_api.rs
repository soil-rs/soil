// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Runtime API definition for assets.

use alloc::vec::Vec;
use codec::Codec;

subsoil::api::decl_runtime_apis! {
	pub trait AssetsApi<AccountId, AssetBalance, AssetId>
	where
		AccountId: Codec,
		AssetBalance: Codec,
		AssetId: Codec,
	{
		/// Returns the list of `AssetId`s and corresponding balance that an `AccountId` has.
		fn account_balances(account: AccountId) -> Vec<(AssetId, AssetBalance)>;
	}
}
