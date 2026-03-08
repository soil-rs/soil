// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Aura consensus data provider, This allows manual seal author blocks that are valid for
//! runtimes that expect the aura-specific digests.

use crate::{ConsensusDataProvider, Error};
use soil_client::client_api::{AuxStore, UsageProvider};
use soil_client::import::BlockImportParams;
use std::{marker::PhantomData, sync::Arc};
use subsoil::api::{ProvideRuntimeApi, StorageProof};
use subsoil::consensus::aura::{
	digests::CompatibleDigestItem,
	sr25519::{AuthorityId, AuthoritySignature},
	AuraApi, Slot, SlotDuration,
};
use subsoil::inherents::InherentData;
use subsoil::runtime::{traits::Block as BlockT, Digest, DigestItem};
use subsoil::timestamp::TimestampInherentData;

/// Consensus data provider for Aura. This allows to use manual-seal driven nodes to author valid
/// AURA blocks. It will inspect incoming [`InherentData`] and look for included timestamps. Based
/// on these timestamps, the [`AuraConsensusDataProvider`] will emit fitting digest items.
pub struct AuraConsensusDataProvider<B> {
	// slot duration
	slot_duration: SlotDuration,
	// phantom data for required generics
	_phantom: PhantomData<B>,
}

impl<B> AuraConsensusDataProvider<B>
where
	B: BlockT,
{
	/// Creates a new instance of the [`AuraConsensusDataProvider`], requires that `client`
	/// implements [`subsoil::consensus::aura::AuraApi`]
	pub fn new<C>(client: Arc<C>) -> Self
	where
		C: AuxStore + ProvideRuntimeApi<B> + UsageProvider<B>,
		C::Api: AuraApi<B, AuthorityId>,
	{
		let slot_duration =
			soil_aura::slot_duration(&*client).expect("slot_duration is always present; qed.");

		Self { slot_duration, _phantom: PhantomData }
	}

	/// Creates a new instance of the [`AuraConsensusDataProvider`]
	pub fn new_with_slot_duration(slot_duration: SlotDuration) -> Self {
		Self { slot_duration, _phantom: PhantomData }
	}
}

impl<B> ConsensusDataProvider<B> for AuraConsensusDataProvider<B>
where
	B: BlockT,
{
	fn create_digest(
		&self,
		_parent: &B::Header,
		inherents: &InherentData,
	) -> Result<Digest, Error> {
		let timestamp =
			inherents.timestamp_inherent_data()?.expect("Timestamp is always present; qed");

		// we always calculate the new slot number based on the current time-stamp and the slot
		// duration.
		let digest_item = <DigestItem as CompatibleDigestItem<AuthoritySignature>>::aura_pre_digest(
			Slot::from_timestamp(timestamp, self.slot_duration),
		);

		Ok(Digest { logs: vec![digest_item] })
	}

	fn append_block_import(
		&self,
		_parent: &B::Header,
		_params: &mut BlockImportParams<B>,
		_inherents: &InherentData,
		_proof: StorageProof,
	) -> Result<(), Error> {
		Ok(())
	}
}
