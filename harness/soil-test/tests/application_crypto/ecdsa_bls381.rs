// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Integration tests for ecdsa-bls12-381

use std::sync::Arc;
use subsoil::api::{ApiExt, ProvideRuntimeApi};
use subsoil::application_crypto::{ecdsa_bls381::AppPair, RuntimePublic};
use subsoil::core::{
	crypto::ByteArray,
	ecdsa_bls381::Pair as EcdsaBls381Pair,
	proof_of_possession::{ProofOfPossessionGenerator, ProofOfPossessionVerifier},
	testing::ECDSA_BLS381,
	Pair,
};
use subsoil::keystore::{testing::MemoryKeystore, Keystore, KeystoreExt};
use soil_test_node_runtime_client::{
	runtime::{TestAPI, TEST_OWNER},
	DefaultTestClientBuilderExt, TestClientBuilder, TestClientBuilderExt,
};

#[test]
fn ecdsa_bls381_works_in_runtime() {
	subsoil::tracing::try_init_simple();
	let keystore = Arc::new(MemoryKeystore::new());
	let test_client = TestClientBuilder::new().build();

	let mut runtime_api = test_client.runtime_api();
	runtime_api.register_extension(KeystoreExt::new(keystore.clone()));

	let (proof_of_possession, public) = runtime_api
		.test_ecdsa_bls381_crypto(test_client.chain_info().genesis_hash)
		.expect("Tests `ecdsa_bls381` crypto.");

	let supported_keys = keystore.keys(ECDSA_BLS381).unwrap();
	assert!(supported_keys.contains(&public.to_raw_vec()));
	assert!(supported_keys.len() == 3);

	assert!(AppPair::verify_proof_of_possession(
		TEST_OWNER,
		&proof_of_possession.into(),
		&public.into()
	));
}

#[test]
fn ecdsa_bls381_client_proof_of_possession_verified_by_runtime_public() {
	let (mut test_pair, _) = EcdsaBls381Pair::generate();

	let client_generated_proof_of_possession = test_pair.generate_proof_of_possession(TEST_OWNER);
	assert!(RuntimePublic::verify_proof_of_possession(
		&test_pair.public(),
		TEST_OWNER,
		&client_generated_proof_of_possession
	));
}
