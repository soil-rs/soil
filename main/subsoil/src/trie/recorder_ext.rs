// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Extension for the default recorder.

use super::RawStorageProof;
use alloc::{collections::BTreeSet, vec::Vec};
use trie_db::{Recorder, TrieLayout};

/// Convenience extension for the `Recorder` struct.
///
/// Used to deduplicate some logic.
pub trait RecorderExt<L: TrieLayout>
where
	Self: Sized,
{
	/// Convert the recorder into a `BTreeSet`.
	fn into_set(self) -> BTreeSet<Vec<u8>>;

	/// Convert the recorder into a `RawStorageProof`, avoiding duplicate nodes.
	fn into_raw_storage_proof(self) -> RawStorageProof {
		// The recorder may record the same trie node multiple times,
		// and we don't want duplicate nodes in our proofs
		// => let's deduplicate it by collecting to a BTreeSet first
		self.into_set().into_iter().collect()
	}
}

impl<L: TrieLayout> RecorderExt<L> for Recorder<L> {
	fn into_set(mut self) -> BTreeSet<Vec<u8>> {
		self.drain().into_iter().map(|record| record.data).collect::<BTreeSet<_>>()
	}
}
