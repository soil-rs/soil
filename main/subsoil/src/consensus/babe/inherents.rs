// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Inherents for BABE

use crate::inherents::{Error, InherentData, InherentIdentifier};

/// The BABE inherent identifier.
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"babeslot";

/// The type of the BABE inherent.
pub type InherentType = crate::consensus::slots::Slot;

/// Create inherent data providers for BABE with timestamp.
#[cfg(feature = "std")]
pub type BabeCreateInherentDataProviders<Block> = std::sync::Arc<
	dyn crate::inherents::CreateInherentDataProviders<
		Block,
		(),
		InherentDataProviders = (InherentDataProvider, crate::timestamp::InherentDataProvider),
	>,
>;

/// Auxiliary trait to extract BABE inherent data.
pub trait BabeInherentData {
	/// Get BABE inherent data.
	fn babe_inherent_data(&self) -> Result<Option<InherentType>, Error>;
	/// Replace BABE inherent data.
	fn babe_replace_inherent_data(&mut self, new: InherentType);
}

impl BabeInherentData for InherentData {
	fn babe_inherent_data(&self) -> Result<Option<InherentType>, Error> {
		self.get_data(&INHERENT_IDENTIFIER)
	}

	fn babe_replace_inherent_data(&mut self, new: InherentType) {
		self.replace_data(INHERENT_IDENTIFIER, &new);
	}
}

/// Provides the slot duration inherent data for BABE.
// TODO: Remove in the future. https://github.com/paritytech/substrate/issues/8029
#[cfg(feature = "std")]
pub struct InherentDataProvider {
	slot: InherentType,
}

#[cfg(feature = "std")]
impl InherentDataProvider {
	/// Create new inherent data provider from the given `slot`.
	pub fn new(slot: InherentType) -> Self {
		Self { slot }
	}

	/// Creates the inherent data provider by calculating the slot from the given
	/// `timestamp` and `duration`.
	pub fn from_timestamp_and_slot_duration(
		timestamp: crate::timestamp::Timestamp,
		slot_duration: crate::consensus::slots::SlotDuration,
	) -> Self {
		let slot = InherentType::from_timestamp(timestamp, slot_duration);

		Self { slot }
	}

	/// Returns the `slot` of this inherent data provider.
	pub fn slot(&self) -> InherentType {
		self.slot
	}
}

#[cfg(feature = "std")]
impl core::ops::Deref for InherentDataProvider {
	type Target = InherentType;

	fn deref(&self) -> &Self::Target {
		&self.slot
	}
}

#[cfg(feature = "std")]
#[async_trait::async_trait]
impl crate::inherents::InherentDataProvider for InherentDataProvider {
	async fn provide_inherent_data(&self, inherent_data: &mut InherentData) -> Result<(), Error> {
		inherent_data.put_data(INHERENT_IDENTIFIER, &self.slot)
	}

	async fn try_handle_error(
		&self,
		_: &InherentIdentifier,
		_: &[u8],
	) -> Option<Result<(), Error>> {
		// There is no error anymore
		None
	}
}
