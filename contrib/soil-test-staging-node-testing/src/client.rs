// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Utilities to build a `TestClient` for `soil-test-staging-node-runtime`.

use subsoil::runtime::BuildStorage;
/// Re-export test-client utilities.
pub use soil_test_node_client::*;

/// Call executor for `soil-test-staging-node-runtime` `TestClient`.
use soil_test_staging_node_cli::service::RuntimeExecutor;

/// Default backend type.
pub type Backend = soil_client::db::Backend<soil_test_staging_node_primitives::Block>;

/// Test client type.
pub type Client = client::Client<
	Backend,
	client::LocalCallExecutor<soil_test_staging_node_primitives::Block, Backend, RuntimeExecutor>,
	soil_test_staging_node_primitives::Block,
	soil_test_staging_node_runtime::RuntimeApi,
>;

/// Genesis configuration parameters for `TestClient`.
#[derive(Default)]
pub struct GenesisParameters;

impl soil_test_node_client::GenesisInit for GenesisParameters {
	fn genesis_storage(&self) -> Storage {
		let mut storage = crate::genesis::config().build_storage().unwrap();
		storage.top.insert(
			subsoil::core::storage::well_known_keys::CODE.to_vec(),
			soil_test_staging_node_runtime::wasm_binary_unwrap().into(),
		);
		storage
	}
}

/// A `test-runtime` extensions to `TestClientBuilder`.
pub trait TestClientBuilderExt: Sized {
	/// Create test client builder.
	fn new() -> Self;

	/// Build the test client.
	fn build(self) -> Client;
}

impl TestClientBuilderExt
	for soil_test_node_client::TestClientBuilder<
		soil_test_staging_node_primitives::Block,
		client::LocalCallExecutor<soil_test_staging_node_primitives::Block, Backend, RuntimeExecutor>,
		Backend,
		GenesisParameters,
	>
{
	fn new() -> Self {
		Self::default()
	}
	fn build(self) -> Client {
		let executor = RuntimeExecutor::builder().build();
		use soil_service::client::LocalCallExecutor;
		use std::sync::Arc;
		let executor = LocalCallExecutor::new(
			self.backend().clone(),
			executor.clone(),
			Default::default(),
			ExecutionExtensions::new(None, Arc::new(executor)),
		)
		.expect("Creates LocalCallExecutor");
		self.build_with_executor(executor).0
	}
}
