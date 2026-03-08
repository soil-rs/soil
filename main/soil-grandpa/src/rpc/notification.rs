// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::GrandpaJustification;
use codec::Encode;
use serde::{Deserialize, Serialize};
use subsoil::runtime::traits::Block as BlockT;

/// An encoded justification proving that the given header has been finalized
#[derive(Clone, Serialize, Deserialize)]
pub struct JustificationNotification(subsoil::core::Bytes);

impl<Block: BlockT> From<GrandpaJustification<Block>> for JustificationNotification {
	fn from(notification: GrandpaJustification<Block>) -> Self {
		JustificationNotification(notification.encode().into())
	}
}
