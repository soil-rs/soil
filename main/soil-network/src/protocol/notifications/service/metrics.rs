// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::{service::metrics::NotificationMetrics, types::ProtocolName};

/// Register opened substream to Prometheus.
pub fn register_substream_opened(metrics: &Option<NotificationMetrics>, protocol: &ProtocolName) {
	if let Some(metrics) = metrics {
		metrics.register_substream_opened(&protocol);
	}
}

/// Register closed substream to Prometheus.
pub fn register_substream_closed(metrics: &Option<NotificationMetrics>, protocol: &ProtocolName) {
	if let Some(metrics) = metrics {
		metrics.register_substream_closed(&protocol);
	}
}

/// Register sent notification to Prometheus.
pub fn register_notification_sent(
	metrics: &Option<std::sync::Arc<NotificationMetrics>>,
	protocol: &ProtocolName,
	size: usize,
) {
	if let Some(metrics) = metrics {
		metrics.register_notification_sent(protocol, size);
	}
}

/// Register received notification to Prometheus.
pub fn register_notification_received(
	metrics: &Option<NotificationMetrics>,
	protocol: &ProtocolName,
	size: usize,
) {
	if let Some(metrics) = metrics {
		metrics.register_notification_received(protocol, size);
	}
}
