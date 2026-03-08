// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Shared logic between on-chain and off-chain components used for slashing using an off-chain
//! worker.

use alloc::{borrow::ToOwned, vec::Vec};
use codec::Encode;
use subsoil::staking::SessionIndex;

pub(super) const PREFIX: &[u8] = b"session_historical";
pub(super) const LAST_PRUNE: &[u8] = b"session_historical_last_prune";

/// Derive the key used to store the list of validators
pub(super) fn derive_key<P: AsRef<[u8]>>(prefix: P, session_index: SessionIndex) -> Vec<u8> {
	session_index.using_encoded(|encoded_session_index| {
		let mut key = prefix.as_ref().to_owned();
		key.push(b'/');
		key.extend_from_slice(encoded_session_index);
		key
	})
}
