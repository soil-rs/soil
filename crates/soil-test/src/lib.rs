//! Shared utilities for integration tests.

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
