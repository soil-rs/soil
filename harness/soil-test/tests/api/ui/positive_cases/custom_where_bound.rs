// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use subsoil::runtime::traits::Block as BlockT;
use soil_test_node_runtime_client::runtime::Block;

struct Runtime {}

pub trait CustomTrait: Encode + Decode + TypeInfo {}

#[derive(Encode, Decode, TypeInfo)]
pub struct SomeImpl;
impl CustomTrait for SomeImpl {}

#[derive(Encode, Decode, TypeInfo)]
pub struct SomeOtherType<C: CustomTrait>(C);

subsoil::api::decl_runtime_apis! {
	pub trait Api<A> where A: CustomTrait {
		fn test() -> A;
		fn test2() -> SomeOtherType<A>;
	}
}

subsoil::api::impl_runtime_apis! {
	impl self::Api<Block, SomeImpl> for Runtime {
		fn test() -> SomeImpl { SomeImpl }
		fn test2() -> SomeOtherType<SomeImpl> { SomeOtherType(SomeImpl) }
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
