// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use soil_test_node_runtime_client::runtime::Block;

subsoil::api::decl_runtime_apis! {
	pub trait Api {
		fn test(data: u64);
		fn test2(data: u64);
	}
}

struct MockApi;

subsoil::api::mock_impl_runtime_apis! {
	impl Api<Block> for MockApi {
		fn test(self, data: u64) {}

		fn test2(&mut self, data: u64) {}
	}
}

fn main() {}
