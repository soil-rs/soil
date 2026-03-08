// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

mod mmr;
pub mod storage;

pub use self::mmr::{is_ancestry_proof_optimal, verify_ancestry_proof, verify_leaves_proof, Mmr};
use crate::primitives::{mmr_lib, DataOrHash, FullLeaf};
use topsoil::traits;

/// Node type for runtime `T`.
pub type NodeOf<T, I, L> = Node<<T as crate::Config<I>>::Hashing, L>;

/// A node stored in the MMR.
pub type Node<H, L> = DataOrHash<H, L>;

/// Default Merging & Hashing behavior for MMR.
pub struct Hasher<H, L>(core::marker::PhantomData<(H, L)>);

impl<H: traits::Hash, L: FullLeaf> mmr_lib::Merge for Hasher<H, L> {
	type Item = Node<H, L>;

	fn merge(left: &Self::Item, right: &Self::Item) -> mmr_lib::Result<Self::Item> {
		let mut concat = left.hash().as_ref().to_vec();
		concat.extend_from_slice(right.hash().as_ref());

		Ok(Node::Hash(<H as traits::Hash>::hash(&concat)))
	}
}
