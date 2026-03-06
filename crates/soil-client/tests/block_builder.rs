// Integration tests for block_builder that use substrate-test-runtime-client.
// These must be integration tests (not unit tests) because soil-client and
// substrate-test-runtime-client have a circular dev-dependency, causing
// "multiple versions of crate" errors when compiled as unit tests.

use soil_client::block_builder::BlockBuilderBuilder;
use soil_client::blockchain::HeaderBackend;
use subsoil::api::ProofRecorder;
use subsoil::core::{Blake2Hasher, Encode};
use subsoil::state_machine::Backend;
use substrate_test_runtime_client::{
	runtime::{Block, ExtrinsicBuilder},
	DefaultTestClientBuilderExt, TestClientBuilderExt,
};

#[test]
fn block_building_storage_proof_does_not_include_runtime_by_default() {
	let builder = substrate_test_runtime_client::TestClientBuilder::new();
	let client = builder.build();

	let genesis_hash = client.info().best_hash;

	let storage_proof_recorder = ProofRecorder::<Block>::default();

	BlockBuilderBuilder::new(&client)
		.on_parent_block(genesis_hash)
		.with_parent_block_number(0)
		.with_proof_recorder(storage_proof_recorder.clone())
		.build()
		.unwrap()
		.build()
		.unwrap();

	let proof = storage_proof_recorder.drain_storage_proof();
	let genesis_state_root = client.header(genesis_hash).unwrap().unwrap().state_root;

	let backend = subsoil::state_machine::create_proof_check_backend::<Blake2Hasher>(
		genesis_state_root,
		proof,
	)
	.unwrap();

	assert!(backend
		.storage(&subsoil::core::storage::well_known_keys::CODE)
		.unwrap_err()
		.contains("Database missing expected key"),);
}

#[test]
fn failing_extrinsic_rolls_back_changes_in_storage_proof() {
	let builder = substrate_test_runtime_client::TestClientBuilder::new();
	let client = builder.build();
	let genesis_hash = client.info().best_hash;

	let proof_recorder = ProofRecorder::<Block>::default();

	let mut block_builder = BlockBuilderBuilder::new(&client)
		.on_parent_block(genesis_hash)
		.with_parent_block_number(0)
		.with_proof_recorder(proof_recorder.clone())
		.build()
		.unwrap();

	block_builder.push(ExtrinsicBuilder::new_read_and_panic(8).build()).unwrap_err();

	block_builder.build().unwrap();

	let proof_with_panic = proof_recorder.drain_storage_proof().encoded_size();

	let proof_recorder = ProofRecorder::<Block>::default();

	let mut block_builder = BlockBuilderBuilder::new(&client)
		.on_parent_block(genesis_hash)
		.with_parent_block_number(0)
		.with_proof_recorder(proof_recorder.clone())
		.build()
		.unwrap();

	block_builder.push(ExtrinsicBuilder::new_read(8).build()).unwrap();

	block_builder.build().unwrap();

	let proof_without_panic = proof_recorder.drain_storage_proof().encoded_size();

	let proof_recorder = ProofRecorder::<Block>::default();

	BlockBuilderBuilder::new(&client)
		.on_parent_block(genesis_hash)
		.with_parent_block_number(0)
		.with_proof_recorder(proof_recorder.clone())
		.build()
		.unwrap()
		.build()
		.unwrap();

	let proof_empty_block = proof_recorder.drain_storage_proof().encoded_size();

	// Ensure that we rolled back the changes of the panicked transaction.
	assert!(proof_without_panic > proof_with_panic);
	assert!(proof_without_panic > proof_empty_block);
	assert_eq!(proof_empty_block, proof_with_panic);
}
