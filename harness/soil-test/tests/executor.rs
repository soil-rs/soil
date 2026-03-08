// This file is part of Soil.

// Copyright (C) Soil contributors.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use soil_client::executor::{common::runtime_blob::RuntimeBlob, WasmExecutor};
use subsoil::io::TestExternalities;

#[test]
fn call_in_interpreted_wasm_works() {
	let mut ext = TestExternalities::default();
	let mut ext = ext.ext();

	let executor = WasmExecutor::<subsoil::io::SubstrateHostFunctions>::builder().build();
	let res = executor
		.uncached_call(
			RuntimeBlob::uncompress_if_needed(&soil_test::empty_return_runtime_wasm()).unwrap(),
			&mut ext,
			true,
			"test_empty_return",
			&[],
		)
		.unwrap();
	assert_eq!(res, vec![0u8; 0]);
}
