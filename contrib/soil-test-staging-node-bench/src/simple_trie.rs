// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use std::{collections::HashMap, sync::Arc};

use hash_db::{AsHashDB, HashDB, Hasher as _, Prefix};
use kvdb::KeyValueDB;
use soil_test_staging_node_primitives::Hash;
use subsoil::trie::DBValue;

pub type Hasher = subsoil::core::Blake2Hasher;

/// Immutable generated trie database with root.
pub struct SimpleTrie<'a> {
	pub db: Arc<dyn KeyValueDB>,
	pub overlay: &'a mut HashMap<Vec<u8>, Option<Vec<u8>>>,
}

impl<'a> AsHashDB<Hasher, DBValue> for SimpleTrie<'a> {
	fn as_hash_db(&self) -> &dyn hash_db::HashDB<Hasher, DBValue> {
		self
	}

	fn as_hash_db_mut<'b>(&'b mut self) -> &'b mut (dyn HashDB<Hasher, DBValue> + 'b) {
		&mut *self
	}
}

impl<'a> HashDB<Hasher, DBValue> for SimpleTrie<'a> {
	fn get(&self, key: &Hash, prefix: Prefix) -> Option<DBValue> {
		let key = subsoil::trie::prefixed_key::<Hasher>(key, prefix);
		if let Some(value) = self.overlay.get(&key) {
			return value.clone();
		}
		self.db.get(0, &key).expect("Database backend error")
	}

	fn contains(&self, hash: &Hash, prefix: Prefix) -> bool {
		self.get(hash, prefix).is_some()
	}

	fn insert(&mut self, prefix: Prefix, value: &[u8]) -> Hash {
		let key = Hasher::hash(value);
		self.emplace(key, prefix, value.to_vec());
		key
	}

	fn emplace(&mut self, key: Hash, prefix: Prefix, value: DBValue) {
		let key = subsoil::trie::prefixed_key::<Hasher>(&key, prefix);
		self.overlay.insert(key, Some(value));
	}

	fn remove(&mut self, key: &Hash, prefix: Prefix) {
		let key = subsoil::trie::prefixed_key::<Hasher>(key, prefix);
		self.overlay.insert(key, None);
	}
}
