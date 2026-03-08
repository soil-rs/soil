// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Substrate client interfaces.
#![warn(missing_docs)]

pub mod backend;
pub mod call_executor;
pub mod client;
pub mod execution_extensions;
pub mod in_mem;
pub mod leaves;
pub mod notifications;
pub mod proof_provider;

pub use crate::blockchain;
pub use crate::blockchain::HeaderBackend;
pub use backend::*;
pub use call_executor::*;
pub use client::*;
pub use notifications::*;
pub use proof_provider::*;

pub use subsoil::state_machine::{CompactProof, StorageProof};
pub use subsoil::storage::{ChildInfo, PrefixedStorageKey, StorageData, StorageKey};

/// Usage Information Provider interface
pub trait UsageProvider<Block: subsoil::runtime::traits::Block> {
	/// Get usage info about current client.
	fn usage_info(&self) -> ClientInfo<Block>;
}

/// Utility methods for the client.
pub mod utils {
	use crate::blockchain::{Error, HeaderBackend, HeaderMetadata};
	use subsoil::runtime::traits::Block as BlockT;

	/// Returns a function for checking block ancestry, the returned function will
	/// return `true` if the given hash (second parameter) is a descendent of the
	/// base (first parameter). If the `current` parameter is defined, it should
	/// represent the current block `hash` and its `parent hash`, if given the
	/// function that's returned will assume that `hash` isn't part of the local DB
	/// yet, and all searches in the DB will instead reference the parent.
	pub fn is_descendent_of<Block: BlockT, T>(
		client: &T,
		current: Option<(Block::Hash, Block::Hash)>,
	) -> impl Fn(&Block::Hash, &Block::Hash) -> Result<bool, Error> + '_
	where
		T: HeaderBackend<Block> + HeaderMetadata<Block, Error = Error>,
	{
		move |base, hash| {
			if base == hash {
				return Ok(false);
			}

			let mut hash = hash;
			if let Some((current_hash, current_parent_hash)) = &current {
				if base == current_hash {
					return Ok(false);
				}
				if hash == current_hash {
					if base == current_parent_hash {
						return Ok(true);
					} else {
						hash = current_parent_hash;
					}
				}
			}

			let ancestor = crate::blockchain::lowest_common_ancestor(client, *hash, *base)?;

			Ok(ancestor.hash == *base)
		}
	}
}
