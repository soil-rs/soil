// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! On-chain logic to store a validator-set for deferred validation using an off-chain worker.

use alloc::vec::Vec;
use codec::Encode;
use subsoil::runtime::traits::Convert;

use super::{shared, Config as HistoricalConfig};
use crate::{Config as SessionConfig, Pallet as SessionModule, SessionIndex};

/// Store the validator-set associated to the `session_index` to the off-chain database.
///
/// Further processing is then done [`off-chain side`](super::offchain).
///
/// **Must** be called from on-chain, i.e. a call that originates from
/// `on_initialize(..)` or `on_finalization(..)`.
/// **Must** be called during the session, which validator-set is to be stored for further
/// off-chain processing. Otherwise the `FullIdentification` might not be available.
pub fn store_session_validator_set_to_offchain<T: HistoricalConfig + SessionConfig>(
	session_index: SessionIndex,
) {
	let encoded_validator_list = <SessionModule<T>>::validators()
		.into_iter()
		.filter_map(|validator_id: <T as SessionConfig>::ValidatorId| {
			let full_identification =
				<<T as HistoricalConfig>::FullIdentificationOf>::convert(validator_id.clone());
			full_identification.map(|full_identification| (validator_id, full_identification))
		})
		.collect::<Vec<_>>();

	encoded_validator_list.using_encoded(|encoded_validator_list| {
		let derived_key = shared::derive_key(shared::PREFIX, session_index);
		subsoil::io::offchain_index::set(derived_key.as_slice(), encoded_validator_list);
	});
}

/// Store the validator set associated to the _current_ session index to the off-chain database.
///
/// See [`store_session_validator_set_to_offchain`]
/// for further information and restrictions.
pub fn store_current_session_validator_set_to_offchain<T: HistoricalConfig + SessionConfig>() {
	store_session_validator_set_to_offchain::<T>(<SessionModule<T>>::current_index());
}
