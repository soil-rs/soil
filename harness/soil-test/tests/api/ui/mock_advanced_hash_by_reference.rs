// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use soil_test_node_runtime_client::runtime::Block;
use subsoil::api::ApiError;

subsoil::api::decl_runtime_apis! {
	pub trait Api {
		fn test();
	}
}

struct MockApi;

subsoil::api::mock_impl_runtime_apis! {
	impl Api<Block> for MockApi {
		#[advanced]
		fn test(&self, _: &Hash) -> Result<(), ApiError> {
			Ok(().into())
		}
	}
}

fn main() {}
