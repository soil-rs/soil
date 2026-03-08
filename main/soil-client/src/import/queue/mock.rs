// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use super::*;

mockall::mock! {
	pub ImportQueueHandle<B: BlockT> {}

	impl<B: BlockT> ImportQueueService<B> for ImportQueueHandle<B> {
		fn import_blocks(&mut self, origin: BlockOrigin, blocks: Vec<IncomingBlock<B>>);
		fn import_justifications(
			&mut self,
			who: RuntimeOrigin,
			hash: B::Hash,
			number: NumberFor<B>,
			justifications: Justifications,
		);
	}
}

mockall::mock! {
	pub ImportQueue<B: BlockT> {}

	#[async_trait::async_trait]
	impl<B: BlockT> ImportQueue<B> for ImportQueue<B> {
		fn service(&self) -> Box<dyn ImportQueueService<B>>;
		fn service_ref(&mut self) -> &mut dyn ImportQueueService<B>;
		fn poll_actions<'a>(&mut self, cx: &mut futures::task::Context<'a>, link: &dyn Link<B>);
		async fn run(self, link: &'__mockall_link dyn Link<B>);
	}
}
