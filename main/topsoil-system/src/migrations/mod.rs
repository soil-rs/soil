// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Migrate the reference counting state.

use super::LOG_TARGET;
use crate::{Config, Pallet};
use codec::{Decode, Encode, FullCodec};
use topsoil_support::{
	pallet_prelude::ValueQuery, traits::PalletInfoAccess, weights::Weight, Blake2_128Concat,
};
use Debug;

/// Type used to encode the number of references an account has.
type RefCount = u32;

/// Information of an account.
#[derive(Clone, Eq, PartialEq, Default, Debug, Encode, Decode)]
struct AccountInfo<Nonce, AccountData> {
	nonce: Nonce,
	consumers: RefCount,
	providers: RefCount,
	sufficients: RefCount,
	data: AccountData,
}

/// Trait to implement to give information about types used for migration
pub trait V2ToV3 {
	/// The system pallet.
	type Pallet: 'static + PalletInfoAccess;

	/// System config account id
	type AccountId: 'static + FullCodec;

	/// System config nonce
	type Nonce: 'static + FullCodec + Copy;

	/// System config account data
	type AccountData: 'static + FullCodec;
}

#[topsoil_support::storage_alias]
type UpgradedToU32RefCount<T: Config> = StorageValue<Pallet<T>, bool, ValueQuery>;

#[topsoil_support::storage_alias]
type UpgradedToTripleRefCount<T: Config> = StorageValue<Pallet<T>, bool, ValueQuery>;

#[topsoil_support::storage_alias]
type Account<V, T: Config> = StorageMap<
	Pallet<T>,
	Blake2_128Concat,
	<V as V2ToV3>::AccountId,
	AccountInfo<<V as V2ToV3>::Nonce, <V as V2ToV3>::AccountData>,
>;

/// Migrate from unique `u8` reference counting to triple `u32` reference counting.
pub fn migrate_from_single_u8_to_triple_ref_count<V: V2ToV3, T: Config>() -> Weight {
	let mut translated: usize = 0;
	<Account<V, T>>::translate::<(V::Nonce, u8, V::AccountData), _>(|_key, (nonce, rc, data)| {
		translated += 1;
		Some(AccountInfo { nonce, consumers: rc as RefCount, providers: 1, sufficients: 0, data })
	});
	log::info!(
		target: LOG_TARGET,
		"Applied migration from single u8 to triple reference counting to {:?} elements.",
		translated
	);
	<UpgradedToU32RefCount<T>>::put(true);
	<UpgradedToTripleRefCount<T>>::put(true);
	Weight::MAX
}

/// Migrate from unique `u32` reference counting to triple `u32` reference counting.
pub fn migrate_from_single_to_triple_ref_count<V: V2ToV3, T: Config>() -> Weight {
	let mut translated: usize = 0;
	<Account<V, T>>::translate::<(V::Nonce, RefCount, V::AccountData), _>(
		|_key, (nonce, consumers, data)| {
			translated += 1;
			Some(AccountInfo { nonce, consumers, providers: 1, sufficients: 0, data })
		},
	);
	log::info!(
		target: LOG_TARGET,
		"Applied migration from single to triple reference counting to {:?} elements.",
		translated
	);
	<UpgradedToTripleRefCount<T>>::put(true);
	Weight::MAX
}

/// Migrate from dual `u32` reference counting to triple `u32` reference counting.
pub fn migrate_from_dual_to_triple_ref_count<V: V2ToV3, T: Config>() -> Weight {
	let mut translated: usize = 0;
	<Account<V, T>>::translate::<(V::Nonce, RefCount, RefCount, V::AccountData), _>(
		|_key, (nonce, consumers, providers, data)| {
			translated += 1;
			Some(AccountInfo { nonce, consumers, providers, sufficients: 0, data })
		},
	);
	log::info!(
		target: LOG_TARGET,
		"Applied migration from dual to triple reference counting to {:?} elements.",
		translated
	);
	<UpgradedToTripleRefCount<T>>::put(true);
	Weight::MAX
}
