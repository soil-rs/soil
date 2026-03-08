// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use alloc::{collections::btree_set::BTreeSet, vec::Vec};
use codec::{Decode, DecodeWithMemTracking, Encode};
use core::iter::{DoubleEndedIterator, IntoIterator};
use hash_db::{HashDB, Hasher};
use scale_info::TypeInfo;

// Note that `LayoutV1` usage here (proof compaction) is compatible
// with `LayoutV0`.
use super::LayoutV1 as Layout;

/// Error associated with the `storage_proof` module.
#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug, TypeInfo)]
pub enum StorageProofError {
	/// The proof contains duplicate nodes.
	DuplicateNodes,
}

/// A proof that some set of key-value pairs are included in the storage trie. The proof contains
/// the storage values so that the partial storage backend can be reconstructed by a verifier that
/// does not already have access to the key-value pairs.
///
/// The proof consists of the set of serialized nodes in the storage trie accessed when looking up
/// the keys covered by the proof. Verifying the proof requires constructing the partial trie from
/// the serialized nodes and performing the key lookups.
#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode, DecodeWithMemTracking, TypeInfo)]
pub struct StorageProof {
	trie_nodes: BTreeSet<Vec<u8>>,
}

impl StorageProof {
	/// Constructs a storage proof from a subset of encoded trie nodes in a storage backend.
	pub fn new(trie_nodes: impl IntoIterator<Item = Vec<u8>>) -> Self {
		StorageProof { trie_nodes: BTreeSet::from_iter(trie_nodes) }
	}

	/// Constructs a storage proof from a subset of encoded trie nodes in a storage backend.
	///
	/// Returns an error if the provided subset of encoded trie nodes contains duplicates.
	pub fn new_with_duplicate_nodes_check(
		trie_nodes: impl IntoIterator<Item = Vec<u8>>,
	) -> Result<Self, StorageProofError> {
		let mut trie_nodes_set = BTreeSet::new();
		for node in trie_nodes {
			if !trie_nodes_set.insert(node) {
				return Err(StorageProofError::DuplicateNodes);
			}
		}

		Ok(StorageProof { trie_nodes: trie_nodes_set })
	}

	/// Returns a new empty proof.
	///
	/// An empty proof is capable of only proving trivial statements (ie. that an empty set of
	/// key-value pairs exist in storage).
	pub fn empty() -> Self {
		StorageProof { trie_nodes: BTreeSet::new() }
	}

	/// Returns whether this is an empty proof.
	pub fn is_empty(&self) -> bool {
		self.trie_nodes.is_empty()
	}

	/// Returns the number of nodes in the proof.
	pub fn len(&self) -> usize {
		self.trie_nodes.len()
	}

	/// Convert into an iterator over encoded trie nodes in lexicographical order constructed
	/// from the proof.
	pub fn into_iter_nodes(self) -> impl Sized + DoubleEndedIterator<Item = Vec<u8>> {
		self.trie_nodes.into_iter()
	}

	/// Create an iterator over encoded trie nodes in lexicographical order constructed
	/// from the proof.
	pub fn iter_nodes(&self) -> impl Sized + DoubleEndedIterator<Item = &Vec<u8>> {
		self.trie_nodes.iter()
	}

	/// Convert into plain node vector.
	pub fn into_nodes(self) -> BTreeSet<Vec<u8>> {
		self.trie_nodes
	}

	/// Creates a [`MemoryDB`](super::MemoryDB) from `Self`.
	pub fn into_memory_db<H: Hasher>(self) -> super::MemoryDB<H> {
		self.into()
	}

	/// Creates a [`MemoryDB`](super::MemoryDB) from `Self` reference.
	pub fn to_memory_db<H: Hasher>(&self) -> super::MemoryDB<H> {
		self.into()
	}

	/// Merges multiple storage proofs covering potentially different sets of keys into one proof
	/// covering all keys. The merged proof output may be smaller than the aggregate size of the
	/// input proofs due to deduplication of trie nodes.
	pub fn merge(proofs: impl IntoIterator<Item = Self>) -> Self {
		let trie_nodes = proofs
			.into_iter()
			.flat_map(|proof| proof.into_iter_nodes())
			.collect::<BTreeSet<_>>()
			.into_iter()
			.collect();

		Self { trie_nodes }
	}

	/// Encode as a compact proof with default trie layout.
	pub fn into_compact_proof<H: Hasher>(
		self,
		root: H::Out,
	) -> Result<CompactProof, super::CompactProofError<H::Out, super::Error<H::Out>>> {
		let db = self.into_memory_db();
		super::encode_compact::<Layout<H>, super::MemoryDB<H>>(&db, &root)
	}

	/// Encode as a compact proof with default trie layout.
	pub fn to_compact_proof<H: Hasher>(
		&self,
		root: H::Out,
	) -> Result<CompactProof, super::CompactProofError<H::Out, super::Error<H::Out>>> {
		let db = self.to_memory_db();
		super::encode_compact::<Layout<H>, super::MemoryDB<H>>(&db, &root)
	}

	/// Returns the estimated encoded size of the compact proof.
	///
	/// Running this operation is a slow operation (build the whole compact proof) and should only
	/// be in non sensitive path.
	///
	/// Return `None` on error.
	pub fn encoded_compact_size<H: Hasher>(self, root: H::Out) -> Option<usize> {
		let compact_proof = self.into_compact_proof::<H>(root);
		compact_proof.ok().map(|p| p.encoded_size())
	}
}

impl<H: Hasher> From<StorageProof> for super::MemoryDB<H> {
	fn from(proof: StorageProof) -> Self {
		From::from(&proof)
	}
}

impl<H: Hasher> From<&StorageProof> for super::MemoryDB<H> {
	fn from(proof: &StorageProof) -> Self {
		let mut db = super::MemoryDB::with_hasher(super::RandomState::default());
		proof.iter_nodes().for_each(|n| {
			db.insert(super::EMPTY_PREFIX, &n);
		});
		db
	}
}

/// Storage proof in compact form.
#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode, DecodeWithMemTracking, TypeInfo)]
pub struct CompactProof {
	pub encoded_nodes: Vec<Vec<u8>>,
}

impl CompactProof {
	/// Return an iterator on the compact encoded nodes.
	pub fn iter_compact_encoded_nodes(&self) -> impl Iterator<Item = &[u8]> {
		self.encoded_nodes.iter().map(Vec::as_slice)
	}

	/// Decode to a full storage_proof.
	pub fn to_storage_proof<H: Hasher>(
		&self,
		expected_root: Option<&H::Out>,
	) -> Result<(StorageProof, H::Out), super::CompactProofError<H::Out, super::Error<H::Out>>> {
		let mut db = super::MemoryDB::<H>::new(&[]);
		let root = super::decode_compact::<Layout<H>, _, _>(
			&mut db,
			self.iter_compact_encoded_nodes(),
			expected_root,
		)?;
		Ok((
			StorageProof::new(db.drain().into_iter().filter_map(|kv| {
				if (kv.1).1 > 0 {
					Some((kv.1).0)
				} else {
					None
				}
			})),
			root,
		))
	}

	/// Convert self into a [`MemoryDB`](super::MemoryDB).
	///
	/// `expected_root` is the expected root of this compact proof.
	///
	/// Returns the memory db and the root of the trie.
	pub fn to_memory_db<H: Hasher>(
		&self,
		expected_root: Option<&H::Out>,
	) -> Result<(super::MemoryDB<H>, H::Out), super::CompactProofError<H::Out, super::Error<H::Out>>>
	{
		let mut db = super::MemoryDB::<H>::new(&[]);
		let root = super::decode_compact::<Layout<H>, _, _>(
			&mut db,
			self.iter_compact_encoded_nodes(),
			expected_root,
		)?;

		Ok((db, root))
	}
}

#[cfg(test)]
pub mod tests {
	use super::super::{tests::create_storage_proof, StorageProof};
	use super::*;

	type Hasher = crate::core::Blake2Hasher;
	type Layout = super::super::LayoutV1<Hasher>;

	const TEST_DATA: &[(&[u8], &[u8])] =
		&[(b"key1", &[1; 64]), (b"key2", &[2; 64]), (b"key3", &[3; 64]), (b"key11", &[4; 64])];

	#[test]
	fn proof_with_duplicate_nodes_is_rejected() {
		let (raw_proof, _root) = create_storage_proof::<Layout>(TEST_DATA);
		assert!(matches!(
			StorageProof::new_with_duplicate_nodes_check(raw_proof),
			Err(StorageProofError::DuplicateNodes)
		));
	}

	#[test]
	fn invalid_compact_proof_does_not_panic_when_decoding() {
		let invalid_proof = CompactProof { encoded_nodes: vec![vec![135]] };
		let result = invalid_proof.to_memory_db::<Hasher>(None);
		assert!(result.is_err());
	}
}
