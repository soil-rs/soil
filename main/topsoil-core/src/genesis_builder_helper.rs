// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Helper functions for implementing [`subsoil::genesis_builder::GenesisBuilder`] for runtimes.
//!
//! Provides common logic. For more info refer to [`subsoil::genesis_builder::GenesisBuilder`].

extern crate alloc;

use alloc::{format, vec::Vec};
use subsoil::genesis_builder::{PresetId, Result as BuildResult};
use topsoil_core::traits::BuildGenesisConfig;

/// Build `GenesisConfig` from a JSON blob not using any defaults and store it in the storage. For
/// more info refer to [`subsoil::genesis_builder::GenesisBuilder::build_state`].
pub fn build_state<GC: BuildGenesisConfig>(json: Vec<u8>) -> BuildResult {
	let gc =
		serde_json::from_slice::<GC>(&json).map_err(|e| format!("Invalid JSON blob: {}", e))?;
	<GC as BuildGenesisConfig>::build(&gc);
	Ok(())
}

/// Get the default `GenesisConfig` as a JSON blob if `name` is None.
///
/// Query of named presets is delegetaed to provided `preset_for_name` closure. For more info refer
/// to [`subsoil::genesis_builder::GenesisBuilder::get_preset`].
pub fn get_preset<GC>(
	name: &Option<PresetId>,
	preset_for_name: impl FnOnce(&PresetId) -> Option<Vec<u8>>,
) -> Option<Vec<u8>>
where
	GC: BuildGenesisConfig + Default,
{
	name.as_ref().map_or(
		Some(
			serde_json::to_string(&GC::default())
				.expect("serialization to json is expected to work. qed.")
				.into_bytes(),
		),
		preset_for_name,
	)
}
