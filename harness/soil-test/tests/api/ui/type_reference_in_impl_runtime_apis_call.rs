// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use subsoil::runtime::traits::Block as BlockT;
use soil_test_node_runtime_client::runtime::Block;

/// The declaration of the `Runtime` type is done by the `construct_runtime!` macro in a real
/// runtime.
struct Runtime {}

subsoil::api::decl_runtime_apis! {
	pub trait Api {
		fn test(data: u64);
	}
}

subsoil::api::impl_runtime_apis! {
	impl self::Api<Block> for Runtime {
		fn test(data: &u64) {
			unimplemented!()
		}
	}

	impl subsoil::api::Core<Block> for Runtime {
		fn version() -> subsoil::api::RuntimeVersion {
			unimplemented!()
		}
		fn execute_block(_: <Block as BlockT>::LazyBlock) {
			unimplemented!()
		}
		fn initialize_block(_: &<Block as BlockT>::Header) -> subsoil::runtime::ExtrinsicInclusionMode {
			unimplemented!()
		}
	}
}

fn main() {}
