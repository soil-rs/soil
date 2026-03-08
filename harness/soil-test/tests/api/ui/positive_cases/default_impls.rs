// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use subsoil::runtime::traits::Block as BlockT;
use soil_test_node_runtime_client::runtime::Block;

struct Runtime {}

subsoil::api::decl_runtime_apis! {
	#[api_version(2)]
	pub trait Api {
		fn test1();
		fn test2();
		#[api_version(3)]
		fn test3();
		#[api_version(4)]
		fn test4();
	}
}

subsoil::api::impl_runtime_apis! {
	#[api_version(2)]
	impl self::Api<Block> for Runtime {
		fn test1() {}
		fn test2() {}
	}

	impl subsoil::api::Core<Block> for Runtime {
		fn version() -> subsoil::version::RuntimeVersion {
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
