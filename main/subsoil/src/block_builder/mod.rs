// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! The block builder runtime api.

#[cfg(feature = "std")]
mod client_side;

#[cfg(feature = "std")]
pub use client_side::*;

use crate::inherents::{CheckInherentsResult, InherentData};
use crate::runtime::{traits::Block as BlockT, ApplyExtrinsicResult};

crate::api::decl_runtime_apis! {
	/// The `BlockBuilder` api trait that provides the required functionality for building a block.
	#[api_version(6)]
	pub trait BlockBuilder {
		/// Apply the given extrinsic.
		///
		/// Returns an inclusion outcome which specifies if this extrinsic is included in
		/// this block or not.
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult;

		#[changed_in(6)]
		fn apply_extrinsic(
			extrinsic: <Block as BlockT>::Extrinsic,
		) -> crate::runtime::legacy::byte_sized_error::ApplyExtrinsicResult;

		/// Finish the current block.
		#[renamed("finalise_block", 3)]
		fn finalize_block() -> <Block as BlockT>::Header;

		/// Generate inherent extrinsics. The inherent data will vary from chain to chain.
		fn inherent_extrinsics(
			inherent: InherentData,
		) -> alloc::vec::Vec<<Block as BlockT>::Extrinsic>;

		/// Check that the inherents are valid. The inherent data will vary from chain to chain.
		fn check_inherents(block: <Block as BlockT>::LazyBlock, data: InherentData) -> CheckInherentsResult;
	}
}
