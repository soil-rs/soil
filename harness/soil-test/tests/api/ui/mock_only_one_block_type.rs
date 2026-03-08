// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

struct Block2;

subsoil::api::decl_runtime_apis! {
	pub trait Api {
		fn test(data: u64);
	}

	pub trait Api2 {
		fn test(data: u64);
	}
}

struct MockApi;

subsoil::api::mock_impl_runtime_apis! {
	impl Api<Block> for MockApi {
		fn test(data: u64) {}
	}

	impl Api2<Block2> for MockApi {
		fn test(data: u64) {}
	}
}

fn main() {}
