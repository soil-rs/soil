// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Metrics abstractions for the trie cache.

use std::time::Duration;

/// Metrics sink for the shared trie cache.
pub trait SharedTrieCacheMetrics: Send + Sync {
	/// Observe the duration spent updating the shared node cache from the local cache.
	fn observe_shared_node_update_duration(&self, duration: Duration);
	/// Observe the duration spent updating the shared value cache from the local cache.
	fn observe_shared_value_update_duration(&self, duration: Duration);
	/// Observe the length of the local node cache at flush time.
	fn observe_local_node_cache_length(&self, node_cache_len: usize);
	/// Observe the length of the local value cache at flush time.
	fn observe_local_value_cache_length(&self, value_cache_len: usize);
	/// Observe the inline size of the shared node cache.
	fn observe_node_cache_inline_size(&self, cache_size: usize);
	/// Observe the inline size of the shared value cache.
	fn observe_value_cache_inline_size(&self, cache_size: usize);
	/// Observe the heap size of the shared node cache.
	fn observe_node_cache_heap_size(&self, cache_size: usize);
	/// Observe the heap size of the shared value cache.
	fn observe_value_cache_heap_size(&self, cache_size: usize);
	/// Observe the hit stats from an instance of a local cache.
	fn observe_hits_stats(&self, stats: &TrieHitStatsSnapshot);
}

/// A snapshot of the hit/miss stats.
#[derive(Default, Copy, Clone, Debug)]
pub struct HitStatsSnapshot {
	pub shared_hits: u64,
	pub shared_fetch_attempts: u64,
	pub local_hits: u64,
	pub local_fetch_attempts: u64,
}

impl std::fmt::Display for HitStatsSnapshot {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		let shared_hits = self.shared_hits;
		let shared_fetch_attempts = self.shared_fetch_attempts;
		let local_hits = self.local_hits;
		let local_fetch_attempts = self.local_fetch_attempts;

		if shared_fetch_attempts == 0 && local_hits == 0 {
			write!(fmt, "empty")
		} else {
			let percent_local = (local_hits as f32 / local_fetch_attempts as f32) * 100.0;
			let percent_shared = (shared_hits as f32 / shared_fetch_attempts as f32) * 100.0;
			write!(
				fmt,
				"local hit rate = {}% [{}/{}], shared hit rate = {}% [{}/{}]",
				percent_local as u32,
				local_hits,
				local_fetch_attempts,
				percent_shared as u32,
				shared_hits,
				shared_fetch_attempts
			)
		}
	}
}

/// Snapshot of the hit/miss stats for the node cache and the value cache.
#[derive(Default, Debug, Clone, Copy)]
pub struct TrieHitStatsSnapshot {
	pub node_cache: HitStatsSnapshot,
	pub value_cache: HitStatsSnapshot,
}
