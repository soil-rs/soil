// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Transaction pool Prometheus metrics for single-state transaction pool.

use crate::common::metrics::{GenericMetricsLink, MetricsRegistrant};
use soil_prometheus::{register, Counter, PrometheusError, Registry, U64};

pub type MetricsLink = GenericMetricsLink<Metrics>;

/// Transaction pool Prometheus metrics.
pub struct Metrics {
	pub submitted_transactions: Counter<U64>,
	pub validations_invalid: Counter<U64>,
	pub block_transactions_pruned: Counter<U64>,
	pub block_transactions_resubmitted: Counter<U64>,
}

impl MetricsRegistrant for Metrics {
	fn register(registry: &Registry) -> Result<Box<Self>, PrometheusError> {
		Ok(Box::from(Self {
			submitted_transactions: register(
				Counter::new(
					"substrate_sub_txpool_submitted_transactions",
					"Total number of transactions submitted",
				)?,
				registry,
			)?,
			validations_invalid: register(
				Counter::new(
					"substrate_sub_txpool_validations_invalid",
					"Total number of transactions that were removed from the pool as invalid",
				)?,
				registry,
			)?,
			block_transactions_pruned: register(
				Counter::new(
					"substrate_sub_txpool_block_transactions_pruned",
					"Total number of transactions that was requested to be pruned by block events",
				)?,
				registry,
			)?,
			block_transactions_resubmitted: register(
				Counter::new(
					"substrate_sub_txpool_block_transactions_resubmitted",
					"Total number of transactions that was requested to be resubmitted by block events",
				)?,
				registry,
			)?,
		}))
	}
}
