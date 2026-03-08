// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

fn main() {
	// regular build
	#[cfg(feature = "std")]
	{
		substrate_wasm_builder::WasmBuilder::new()
			.with_current_project()
			.export_heap_base()
			.import_memory()
			.disable_runtime_version_section_check()
			.build();
	}

	// and building with tracing activated
	#[cfg(feature = "std")]
	{
		substrate_wasm_builder::WasmBuilder::new()
			.with_current_project()
			.export_heap_base()
			.import_memory()
			.set_file_name("wasm_binary_with_tracing.rs")
			.append_to_rust_flags(r#"--cfg feature="with-tracing""#)
			.disable_runtime_version_section_check()
			.build();
	}
}
