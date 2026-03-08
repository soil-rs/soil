// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use std::sync::Arc;
use subsoil::core::offchain::{storage::InMemOffchainStorage, OffchainStorage};

type TestBackend = soil_client::client_api::in_mem::Backend<soil_test_node_runtime::Block>;

#[test]
fn test_leaves_with_complex_block_tree() {
	let backend = Arc::new(TestBackend::new());

	soil_test_node_runtime_client::trait_tests::test_leaves_for_backend(backend);
}

#[test]
fn test_blockchain_query_by_number_gets_canonical() {
	let backend = Arc::new(TestBackend::new());

	soil_test_node_runtime_client::trait_tests::test_blockchain_query_by_number_gets_canonical(
		backend,
	);
}

#[test]
fn in_memory_offchain_storage() {
	let mut storage = InMemOffchainStorage::default();
	assert_eq!(storage.get(b"A", b"B"), None);
	assert_eq!(storage.get(b"B", b"A"), None);

	storage.set(b"A", b"B", b"C");
	assert_eq!(storage.get(b"A", b"B"), Some(b"C".to_vec()));
	assert_eq!(storage.get(b"B", b"A"), None);

	storage.compare_and_set(b"A", b"B", Some(b"X"), b"D");
	assert_eq!(storage.get(b"A", b"B"), Some(b"C".to_vec()));
	storage.compare_and_set(b"A", b"B", Some(b"C"), b"D");
	assert_eq!(storage.get(b"A", b"B"), Some(b"D".to_vec()));

	assert!(!storage.compare_and_set(b"B", b"A", Some(b""), b"Y"));
	assert!(storage.compare_and_set(b"B", b"A", None, b"X"));
	assert_eq!(storage.get(b"B", b"A"), Some(b"X".to_vec()));
}
