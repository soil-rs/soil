// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

fn main() {
	#[cfg(feature = "std")]
	{
		substrate_wasm_builder::WasmBuilder::new()
			.with_current_project()
			.export_heap_base()
			// Note that we set the stack-size to 1MB explicitly even though it is set
			// to this value by default. This is because some of our tests
			// (`restoration_of_globals`) depend on the stack-size.
			.append_to_rust_flags("-Clink-arg=-zstack-size=1048576")
			.enable_metadata_hash("TOKEN", 10)
			.import_memory()
			.build();
	}

	#[cfg(feature = "std")]
	{
		substrate_wasm_builder::WasmBuilder::new()
			.with_current_project()
			.export_heap_base()
			.import_memory()
			.set_file_name("wasm_binary_logging_disabled.rs")
			.enable_feature("disable-logging")
			.build();
	}
}
