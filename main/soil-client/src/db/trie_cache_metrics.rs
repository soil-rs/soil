// This file is part of Soil.
//
// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use soil_prometheus::{
	exponential_buckets, register, CounterVec, GaugeVec, HistogramOpts, HistogramVec, Opts,
	PrometheusError, Registry, U64,
};
use std::time::Duration;
use subsoil::trie::cache::{SharedTrieCacheMetrics, TrieHitStatsSnapshot};

/// Prometheus-backed metrics sink for the shared trie cache.
#[derive(Clone)]
pub(crate) struct PrometheusTrieCacheMetrics {
	shared_update_duration: HistogramVec,
	shared_hits: CounterVec<U64>,
	shared_fetch_attempts: CounterVec<U64>,
	local_hits: CounterVec<U64>,
	local_fetch_attempts: CounterVec<U64>,
	local_cache_lengths: HistogramVec,
	shared_cache_inline_size: GaugeVec<U64>,
	shared_cache_heap_size: GaugeVec<U64>,
}

impl PrometheusTrieCacheMetrics {
	pub(crate) fn register(registry: &Registry) -> Result<Self, PrometheusError> {
		Ok(Self {
			shared_update_duration: register(
				HistogramVec::new(
					HistogramOpts {
						common_opts: Opts::new(
							"trie_cache_shared_update_duration",
							"Duration in seconds to update the shared trie caches from local cache to shared cache",
						),
						buckets: exponential_buckets(0.001, 4.0, 9)
							.expect("function parameters are constant and always valid; qed"),
					},
					&["cache_type"],
				)?,
				registry,
			)?,
			shared_hits: register(
				CounterVec::new(
					Opts::new(
						"trie_cache_shared_hits",
						"Number of attempts hitting the shared trie cache",
					),
					&["cache_type"],
				)?,
				registry,
			)?,
			shared_fetch_attempts: register(
				CounterVec::new(
					Opts::new(
						"trie_cache_shared_fetch_attempts",
						"Number of attempts to the shared trie cache",
					),
					&["cache_type"],
				)?,
				registry,
			)?,
			local_hits: register(
				CounterVec::new(
					Opts::new(
						"trie_cache_local_hits",
						"Number of attempts hitting the local trie cache",
					),
					&["cache_type"],
				)?,
				registry,
			)?,
			local_fetch_attempts: register(
				CounterVec::new(
					Opts::new(
						"trie_cache_local_fetch_attempts",
						"Number of attempts to the local cache",
					),
					&["cache_type"],
				)?,
				registry,
			)?,
			local_cache_lengths: register(
				HistogramVec::new(
					HistogramOpts {
						common_opts: Opts::new(
							"trie_cache_local_cache_lengths",
							"Histogram of length of the local cache",
						),
						buckets: exponential_buckets(1.0, 4.0, 9)
							.expect("function parameters are constant and always valid; qed"),
					},
					&["cache_type"],
				)?,
				registry,
			)?,
			shared_cache_inline_size: register(
				GaugeVec::new(
					Opts::new(
						"trie_cache_shared_cache_inline_size",
						"The inline size of the shared caches",
					),
					&["cache_type"],
				)?,
				registry,
			)?,
			shared_cache_heap_size: register(
				GaugeVec::new(
					Opts::new(
						"trie_cache_shared_cache_heap_size",
						"The heap size of the shared caches",
					),
					&["cache_type"],
				)?,
				registry,
			)?,
		})
	}
}

impl SharedTrieCacheMetrics for PrometheusTrieCacheMetrics {
	fn observe_shared_node_update_duration(&self, duration: Duration) {
		self.shared_update_duration.with_label_values(&["node"]).observe(duration.as_secs_f64());
	}

	fn observe_shared_value_update_duration(&self, duration: Duration) {
		self.shared_update_duration.with_label_values(&["value"]).observe(duration.as_secs_f64());
	}

	fn observe_local_node_cache_length(&self, node_cache_len: usize) {
		self.local_cache_lengths.with_label_values(&["node"]).observe(node_cache_len as f64);
	}

	fn observe_local_value_cache_length(&self, value_cache_len: usize) {
		self.local_cache_lengths.with_label_values(&["value"]).observe(value_cache_len as f64);
	}

	fn observe_node_cache_inline_size(&self, cache_size: usize) {
		self.shared_cache_inline_size.with_label_values(&["node"]).set(cache_size as u64);
	}

	fn observe_value_cache_inline_size(&self, cache_size: usize) {
		self.shared_cache_inline_size.with_label_values(&["value"]).set(cache_size as u64);
	}

	fn observe_node_cache_heap_size(&self, cache_size: usize) {
		self.shared_cache_heap_size.with_label_values(&["node"]).set(cache_size as u64);
	}

	fn observe_value_cache_heap_size(&self, cache_size: usize) {
		self.shared_cache_heap_size.with_label_values(&["value"]).set(cache_size as u64);
	}

	fn observe_hits_stats(&self, stats: &TrieHitStatsSnapshot) {
		self.shared_hits.with_label_values(&["node"]).inc_by(stats.node_cache.shared_hits);
		self.shared_fetch_attempts
			.with_label_values(&["node"])
			.inc_by(stats.node_cache.shared_fetch_attempts);
		self.local_hits.with_label_values(&["node"]).inc_by(stats.node_cache.local_hits);
		self.local_fetch_attempts
			.with_label_values(&["node"])
			.inc_by(stats.node_cache.local_fetch_attempts);

		self.shared_hits.with_label_values(&["value"]).inc_by(stats.value_cache.shared_hits);
		self.shared_fetch_attempts
			.with_label_values(&["value"])
			.inc_by(stats.value_cache.shared_fetch_attempts);
		self.local_hits.with_label_values(&["value"]).inc_by(stats.value_cache.local_hits);
		self.local_fetch_attempts
			.with_label_values(&["value"])
			.inc_by(stats.value_cache.local_fetch_attempts);
	}
}
