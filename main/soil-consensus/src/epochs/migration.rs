// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Migration types for epoch changes.

use super::{Epoch, EpochChanges, PersistedEpoch, PersistedEpochHeader};
use codec::{Decode, Encode};
use soil_fork_tree::ForkTree;
use std::collections::BTreeMap;
use subsoil::runtime::traits::{Block as BlockT, NumberFor};

/// Legacy definition of epoch changes.
#[derive(Clone, Encode, Decode)]
pub struct EpochChangesV0<Hash, Number, E: Epoch> {
	inner: ForkTree<Hash, Number, PersistedEpoch<E>>,
}

/// Legacy definition of epoch changes.
#[derive(Clone, Encode, Decode)]
pub struct EpochChangesV1<Hash, Number, E: Epoch> {
	inner: ForkTree<Hash, Number, PersistedEpochHeader<E>>,
	epochs: BTreeMap<(Hash, Number), PersistedEpoch<E>>,
}

/// Type alias for v0 definition of epoch changes.
pub type EpochChangesV0For<Block, Epoch> =
	EpochChangesV0<<Block as BlockT>::Hash, NumberFor<Block>, Epoch>;
/// Type alias for v1 and v2 definition of epoch changes.
pub type EpochChangesV1For<Block, Epoch> =
	EpochChangesV1<<Block as BlockT>::Hash, NumberFor<Block>, Epoch>;

impl<Hash, Number, E: Epoch> EpochChangesV0<Hash, Number, E>
where
	Hash: PartialEq + Ord + Copy,
	Number: Ord + Copy,
{
	/// Create a new value of this type from raw.
	pub fn from_raw(inner: ForkTree<Hash, Number, PersistedEpoch<E>>) -> Self {
		Self { inner }
	}

	/// Migrate the type into current epoch changes definition.
	pub fn migrate(self) -> EpochChanges<Hash, Number, E> {
		let mut epochs = BTreeMap::new();

		let inner = self.inner.map(&mut |hash, number, data| {
			let header = PersistedEpochHeader::from(&data);
			epochs.insert((*hash, *number), data);
			header
		});

		EpochChanges { inner, epochs }
	}
}

impl<Hash, Number, E: Epoch> EpochChangesV1<Hash, Number, E>
where
	Hash: PartialEq + Ord + Copy,
	Number: Ord + Copy,
{
	/// Migrate the type into current epoch changes definition.
	pub fn migrate(self) -> EpochChanges<Hash, Number, E> {
		EpochChanges { inner: self.inner, epochs: self.epochs }
	}
}
