// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use clap::Args;
use soil_service::config::PrometheusConfig;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

/// Parameters used to config prometheus.
#[derive(Debug, Clone, Args)]
pub struct PrometheusParams {
	/// Specify Prometheus exporter TCP Port.
	#[arg(long, value_name = "PORT")]
	pub prometheus_port: Option<u16>,
	/// Expose Prometheus exporter on all interfaces.
	///
	/// Default is local.
	#[arg(long)]
	pub prometheus_external: bool,
	/// Do not expose a Prometheus exporter endpoint.
	///
	/// Prometheus metric endpoint is enabled by default.
	#[arg(long)]
	pub no_prometheus: bool,
}

impl PrometheusParams {
	/// Creates [`PrometheusConfig`].
	pub fn prometheus_config(
		&self,
		default_listen_port: u16,
		chain_id: String,
	) -> Option<PrometheusConfig> {
		if self.no_prometheus {
			None
		} else {
			let interface: IpAddr = if self.prometheus_external {
				Ipv6Addr::UNSPECIFIED.into()
			} else {
				Ipv4Addr::LOCALHOST.into()
			};

			Some(PrometheusConfig::new_with_default_registry(
				SocketAddr::new(
					interface.into(),
					self.prometheus_port.unwrap_or(default_listen_port),
				),
				chain_id,
			))
		}
	}
}
