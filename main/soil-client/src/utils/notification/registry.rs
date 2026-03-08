// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use std::collections::HashSet;

use crate::utils::{
	id_sequence::SeqID,
	pubsub::{Dispatch, Subscribe, Unsubscribe},
};

/// The shared structure to keep track on subscribers.
#[derive(Debug, Default)]
pub(super) struct Registry {
	pub(super) subscribers: HashSet<SeqID>,
}

impl Subscribe<()> for Registry {
	fn subscribe(&mut self, _subs_key: (), subs_id: SeqID) {
		self.subscribers.insert(subs_id);
	}
}
impl Unsubscribe for Registry {
	fn unsubscribe(&mut self, subs_id: SeqID) {
		self.subscribers.remove(&subs_id);
	}
}

impl<MakePayload, Payload, Error> Dispatch<MakePayload> for Registry
where
	MakePayload: FnOnce() -> Result<Payload, Error>,
	Payload: Clone,
{
	type Item = Payload;
	type Ret = Result<(), Error>;

	fn dispatch<F>(&mut self, make_payload: MakePayload, mut dispatch: F) -> Self::Ret
	where
		F: FnMut(&SeqID, Self::Item),
	{
		if !self.subscribers.is_empty() {
			let payload = make_payload()?;
			for subs_id in &self.subscribers {
				dispatch(subs_id, payload.clone());
			}
		}
		Ok(())
	}
}
