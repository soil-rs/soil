// Integration tests for db backend that use substrate-test-runtime-client.
// These must be integration tests (not unit tests) because soil-client and
// substrate-test-runtime-client have a circular dev-dependency, causing
// "multiple versions of crate" errors when compiled as unit tests.

use std::sync::Arc;
use soil_client::db::Backend;

#[test]
fn test_leaves_with_complex_block_tree() {
	let backend: Arc<Backend<substrate_test_runtime_client::runtime::Block>> =
		Arc::new(Backend::new_test(20, 20));
	substrate_test_runtime_client::trait_tests::test_leaves_for_backend(backend);
}

#[test]
fn test_children_with_complex_block_tree() {
	let backend: Arc<Backend<substrate_test_runtime_client::runtime::Block>> =
		Arc::new(Backend::new_test(20, 20));
	substrate_test_runtime_client::trait_tests::test_children_for_backend(backend);
}

#[test]
fn test_blockchain_query_by_number_gets_canonical() {
	let backend: Arc<Backend<substrate_test_runtime_client::runtime::Block>> =
		Arc::new(Backend::new_test(20, 20));
	substrate_test_runtime_client::trait_tests::test_blockchain_query_by_number_gets_canonical(
		backend,
	);
}
