// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Command ran by the CLI

use crate::{
	cli::{InspectCmd, InspectSubCmd},
	Inspector,
};
use soil_cli::{CliConfiguration, ImportParams, Result, SharedParams};
use soil_service::Configuration;
use subsoil::runtime::traits::Block;

type HostFunctions =
	(subsoil::io::SubstrateHostFunctions, soil_statement_store::runtime_api::HostFunctions);

impl InspectCmd {
	/// Run the inspect command, passing the inspector.
	pub fn run<B, RA>(&self, config: Configuration) -> Result<()>
	where
		B: Block,
		RA: Send + Sync + 'static,
	{
		let executor = soil_service::new_wasm_executor::<HostFunctions>(&config.executor);
		let client =
			soil_service::new_full_client::<B, RA, _>(&config, None, executor, Default::default())?;
		let inspect = Inspector::<B>::new(client);

		match &self.command {
			InspectSubCmd::Block { input } => {
				let input = input.parse()?;
				let res = inspect.block(input).map_err(|e| e.to_string())?;
				println!("{res}");
				Ok(())
			},
			InspectSubCmd::Extrinsic { input } => {
				let input = input.parse()?;
				let res = inspect.extrinsic(input).map_err(|e| e.to_string())?;
				println!("{res}");
				Ok(())
			},
		}
	}
}

impl CliConfiguration for InspectCmd {
	fn shared_params(&self) -> &SharedParams {
		&self.shared_params
	}

	fn import_params(&self) -> Option<&ImportParams> {
		Some(&self.import_params)
	}
}
