// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use std::{collections::HashMap, sync::Arc};

use kvdb::KeyValueDB;
use soil_test_staging_node_primitives::Hash;
use subsoil::trie::{trie_types::TrieDBMutBuilderV1, TrieMut};

use crate::simple_trie::SimpleTrie;

/// Generate trie from given `key_values`.
///
/// Will fill your database `db` with trie data from `key_values` and
/// return root.
pub fn generate_trie(
	db: Arc<dyn KeyValueDB>,
	key_values: impl IntoIterator<Item = (Vec<u8>, Vec<u8>)>,
) -> Hash {
	let mut root = Hash::default();

	let (db, overlay) = {
		let mut overlay = HashMap::new();
		overlay.insert(
			array_bytes::hex2bytes(
				"03170a2e7597b7b7e3d84c05391d139a62b157e78786d8c082f29dcf4c111314",
			)
			.expect("null key is valid"),
			Some(vec![0]),
		);
		let mut trie = SimpleTrie { db, overlay: &mut overlay };
		{
			let mut trie_db =
				TrieDBMutBuilderV1::<crate::simple_trie::Hasher>::new(&mut trie, &mut root).build();
			for (key, value) in key_values {
				trie_db.insert(&key, &value).expect("trie insertion failed");
			}

			trie_db.commit();
		}
		(trie.db, overlay)
	};

	let mut transaction = db.transaction();
	for (key, value) in overlay.into_iter() {
		match value {
			Some(value) => transaction.put(0, &key[..], &value[..]),
			None => transaction.delete(0, &key[..]),
		}
	}
	db.write(transaction).expect("Failed to write transaction");

	root
}
