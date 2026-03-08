// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use futures::{future::FutureExt, ready, stream::Stream};
use futures_timer::Delay;
use std::{
	pin::Pin,
	task::{Context, Poll},
	time::Duration,
};

/// Exponentially increasing interval
///
/// Doubles interval duration on each tick until the configured maximum is reached.
pub struct ExpIncInterval {
	start: Duration,
	max: Duration,
	next: Duration,
	delay: Delay,
}

impl ExpIncInterval {
	/// Create a new [`ExpIncInterval`].
	pub fn new(start: Duration, max: Duration) -> Self {
		let delay = Delay::new(start);
		Self { start, max, next: start * 2, delay }
	}

	/// Fast forward the exponentially increasing interval to the configured maximum, if not already
	/// set.
	pub fn set_to_max(&mut self) {
		if self.next == self.max {
			return;
		}

		self.next = self.max;
		self.delay = Delay::new(self.next);
	}

	/// Rewind the exponentially increasing interval to the configured start, if not already set.
	pub fn set_to_start(&mut self) {
		if self.next == self.start * 2 {
			return;
		}

		self.next = self.start * 2;
		self.delay = Delay::new(self.start);
	}
}

impl Stream for ExpIncInterval {
	type Item = ();

	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
		ready!(self.delay.poll_unpin(cx));
		self.delay = Delay::new(self.next);
		self.next = std::cmp::min(self.max, self.next * 2);

		Poll::Ready(Some(()))
	}
}
