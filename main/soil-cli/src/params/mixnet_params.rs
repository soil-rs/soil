// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use clap::Args;
use std::str::FromStr;
use subsoil::core::H256;

fn parse_kx_secret(s: &str) -> Result<soil_network::mixnet::KxSecret, String> {
	H256::from_str(s).map(H256::to_fixed_bytes).map_err(|err| err.to_string())
}

/// Parameters used to create the mixnet configuration.
#[derive(Debug, Clone, Args)]
pub struct MixnetParams {
	/// Enable the mixnet service.
	///
	/// This will make the mixnet RPC methods available. If the node is running as a validator, it
	/// will also attempt to register and operate as a mixnode.
	#[arg(long)]
	pub mixnet: bool,

	/// The mixnet key-exchange secret to use in session 0.
	///
	/// Should be 64 hex characters, giving a 32-byte secret.
	///
	/// WARNING: Secrets provided as command-line arguments are easily exposed. Use of this option
	/// should be limited to development and testing.
	#[arg(long, value_name = "SECRET", value_parser = parse_kx_secret)]
	pub mixnet_session_0_kx_secret: Option<soil_network::mixnet::KxSecret>,
}

impl MixnetParams {
	/// Returns the mixnet configuration, or `None` if the mixnet is disabled.
	pub fn config(&self, is_authority: bool) -> Option<soil_network::mixnet::Config> {
		self.mixnet.then(|| {
			let mut config = soil_network::mixnet::Config {
				core: soil_network::mixnet::CoreConfig {
					session_0_kx_secret: self.mixnet_session_0_kx_secret,
					..Default::default()
				},
				..Default::default()
			};
			if !is_authority {
				// Only authorities can be mixnodes; don't attempt to register
				config.substrate.register = false;
				// Only mixnodes need to allow connections from non-mixnodes
				config.substrate.num_gateway_slots = 0;
			}
			config
		})
	}
}
