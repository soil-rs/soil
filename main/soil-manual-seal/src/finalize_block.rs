// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Block finalization utilities

use crate::rpc;
use soil_client::client_api::backend::{Backend as ClientBackend, Finalizer};
use std::{marker::PhantomData, sync::Arc};
use subsoil::runtime::{traits::Block as BlockT, Justification};

/// params for block finalization.
pub struct FinalizeBlockParams<B: BlockT, F, CB> {
	/// hash of the block
	pub hash: <B as BlockT>::Hash,
	/// sender to report errors/success to the rpc.
	pub sender: rpc::Sender<()>,
	/// finalization justification
	pub justification: Option<Justification>,
	/// Finalizer trait object.
	pub finalizer: Arc<F>,
	/// phantom type to pin the Backend type
	pub _phantom: PhantomData<CB>,
}

/// finalizes a block in the backend with the given params.
pub async fn finalize_block<B, F, CB>(params: FinalizeBlockParams<B, F, CB>)
where
	B: BlockT,
	F: Finalizer<B, CB>,
	CB: ClientBackend<B>,
{
	let FinalizeBlockParams { hash, mut sender, justification, finalizer, .. } = params;

	match finalizer.finalize_block(hash, justification, true) {
		Err(e) => {
			log::warn!("Failed to finalize block {}", e);
			rpc::send_result(&mut sender, Err(e.into()))
		},
		Ok(()) => {
			log::info!("✅ Successfully finalized block: {}", hash);
			rpc::send_result(&mut sender, Ok(()))
		},
	}
}
