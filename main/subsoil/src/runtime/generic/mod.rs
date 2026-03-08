// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Generic implementations of [`crate::traits::Header`], [`crate::traits::Block`] and
//! [`crate::traits::ExtrinsicLike`].

mod block;
mod checked_extrinsic;
mod digest;
mod era;
mod header;
#[cfg(test)]
mod tests;
mod unchecked_extrinsic;

pub use self::{
	block::{Block, BlockId, LazyBlock, SignedBlock},
	checked_extrinsic::{CheckedExtrinsic, ExtrinsicFormat},
	digest::{Digest, DigestItem, DigestItemRef, OpaqueDigestItemId},
	era::{Era, Phase},
	header::Header,
	unchecked_extrinsic::{
		CallAndMaybeEncoded, ExtensionVersion, Preamble, SignedPayload, UncheckedExtrinsic,
		EXTRINSIC_FORMAT_VERSION,
	},
};
pub use unchecked_extrinsic::UncheckedSignaturePayload;
