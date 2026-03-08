// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Runtime API definition for the FRAME NFTs pallet.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use codec::{Decode, Encode};

subsoil::api::decl_runtime_apis! {
	pub trait NftsApi<AccountId, CollectionId, ItemId>
	where
		AccountId: Encode + Decode,
		CollectionId: Encode,
		ItemId: Encode,
	{
		fn owner(collection: CollectionId, item: ItemId) -> Option<AccountId>;

		fn collection_owner(collection: CollectionId) -> Option<AccountId>;

		fn attribute(
			collection: CollectionId,
			item: ItemId,
			key: Vec<u8>,
		) -> Option<Vec<u8>>;

		fn custom_attribute(
			account: AccountId,
			collection: CollectionId,
			item: ItemId,
			key: Vec<u8>,
		) -> Option<Vec<u8>>;

		fn system_attribute(
			collection: CollectionId,
			item: Option<ItemId>,
			key: Vec<u8>,
		) -> Option<Vec<u8>>;

		fn collection_attribute(collection: CollectionId, key: Vec<u8>) -> Option<Vec<u8>>;
	}
}
