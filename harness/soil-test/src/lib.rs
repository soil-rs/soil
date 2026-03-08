// This file is part of Soil.

// Copyright (C) Soil contributors.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Shared utilities for integration tests.

pub mod network;
pub mod primitives;
pub mod service;

/// A minimal runtime ABI-compatible Wasm module that returns an empty byte slice from
/// `test_empty_return`.
pub fn empty_return_runtime_wasm() -> Vec<u8> {
	wat::parse_str(
		r#"
			(module
				(memory (export "memory") 1)
				(global (export "__heap_base") i32 (i32.const 1024))
				(func (export "test_empty_return") (param i32 i32) (result i64)
					(i64.const 0)
				)
			)
		"#,
	)
	.expect("test runtime wat is valid")
}
