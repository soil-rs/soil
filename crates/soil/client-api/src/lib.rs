// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate client interfaces.
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
pub mod backend;
#[cfg(feature = "std")]
pub mod call_executor;
#[cfg(feature = "std")]
pub mod client;
#[cfg(feature = "std")]
pub mod execution_extensions;
#[cfg(feature = "std")]
pub mod in_mem;
#[cfg(feature = "std")]
pub mod leaves;
#[cfg(feature = "std")]
pub mod notifications;
#[cfg(feature = "std")]
pub mod proof_provider;

#[cfg(feature = "std")]
pub use backend::*;
#[cfg(feature = "std")]
pub use call_executor::*;
#[cfg(feature = "std")]
pub use client::*;
#[cfg(feature = "std")]
pub use notifications::*;
#[cfg(feature = "std")]
pub use proof_provider::*;
#[cfg(feature = "std")]
pub use soil_blockchain as blockchain;
#[cfg(feature = "std")]
pub use soil_blockchain::HeaderBackend;

#[cfg(feature = "std")]
pub use soil_state_machine::{CompactProof, StorageProof};
#[cfg(feature = "std")]
pub use subsoil::storage::{ChildInfo, PrefixedStorageKey, StorageData, StorageKey};

/// Usage Information Provider interface
#[cfg(feature = "std")]
pub trait UsageProvider<Block: soil_runtime::traits::Block> {
	/// Get usage info about current client.
	fn usage_info(&self) -> ClientInfo<Block>;
}

/// Utility methods for the client.
#[cfg(feature = "std")]
pub mod utils {
	use soil_blockchain::{Error, HeaderBackend, HeaderMetadata};
	use soil_runtime::traits::Block as BlockT;

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

			let ancestor = soil_blockchain::lowest_common_ancestor(client, *hash, *base)?;

			Ok(ancestor.hash == *base)
		}
	}
}
