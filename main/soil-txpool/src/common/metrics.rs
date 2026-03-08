// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Transaction pool Prometheus metrics for implementation of Chain API.

use soil_prometheus::{register, Counter, PrometheusError, Registry, U64};
use std::sync::Arc;

use crate::LOG_TARGET;

/// Provides interface to register the specific metrics in the Prometheus register.
pub(crate) trait MetricsRegistrant {
	/// Registers the metrics at given Prometheus registry.
	fn register(registry: &Registry) -> Result<Box<Self>, PrometheusError>;
}

/// Generic structure to keep a link to metrics register.
pub(crate) struct GenericMetricsLink<M: MetricsRegistrant>(Arc<Option<Box<M>>>);

impl<M: MetricsRegistrant> Default for GenericMetricsLink<M> {
	fn default() -> Self {
		Self(Arc::from(None))
	}
}

impl<M: MetricsRegistrant> Clone for GenericMetricsLink<M> {
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

impl<M: MetricsRegistrant> GenericMetricsLink<M> {
	pub fn new(registry: Option<&Registry>) -> Self {
		Self(Arc::new(registry.and_then(|registry| {
			M::register(registry)
				.map_err(|error| {
					tracing::warn!(
						target: LOG_TARGET,
						%error,
						"Failed to register prometheus metrics"
					);
				})
				.ok()
		})))
	}

	pub fn report(&self, do_this: impl FnOnce(&M)) {
		if let Some(metrics) = self.0.as_ref() {
			do_this(&**metrics);
		}
	}
}

/// Transaction pool api Prometheus metrics.
pub struct ApiMetrics {
	pub validations_scheduled: Counter<U64>,
	pub validations_finished: Counter<U64>,
}

impl ApiMetrics {
	/// Register the metrics at the given Prometheus registry.
	pub fn register(registry: &Registry) -> Result<Self, PrometheusError> {
		Ok(Self {
			validations_scheduled: register(
				Counter::new(
					"substrate_sub_txpool_validations_scheduled",
					"Total number of transactions scheduled for validation",
				)?,
				registry,
			)?,
			validations_finished: register(
				Counter::new(
					"substrate_sub_txpool_validations_finished",
					"Total number of transactions that finished validation",
				)?,
				registry,
			)?,
		})
	}
}

/// An extension trait for [`ApiMetrics`].
pub trait ApiMetricsExt {
	/// Report an event to the metrics.
	fn report(&self, report: impl FnOnce(&ApiMetrics));
}

impl ApiMetricsExt for Option<Arc<ApiMetrics>> {
	fn report(&self, report: impl FnOnce(&ApiMetrics)) {
		if let Some(metrics) = self.as_ref() {
			report(metrics)
		}
	}
}
