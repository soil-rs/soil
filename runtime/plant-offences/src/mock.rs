// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test utilities

#![cfg(test)]

use crate as offences;
use crate::Config;
use codec::Encode;
use subsoil::staking::{
	offence::{self, Kind, OffenceDetails},
	SessionIndex,
};
use subsoil::runtime::{traits::IdentityLookup, BuildStorage, Perbill};
use topsoil_core::{
	derive_impl, parameter_types,
	traits::ConstU32,
	weights::{constants::RocksDbWeight, Weight},
};

pub struct OnOffenceHandler;

parameter_types! {
	pub static OnOffencePerbill: Vec<Perbill> = Default::default();
	pub static OffenceWeight: Weight = Default::default();
}

impl<Reporter, Offender> offence::OnOffenceHandler<Reporter, Offender, Weight>
	for OnOffenceHandler
{
	fn on_offence(
		_offenders: &[OffenceDetails<Reporter, Offender>],
		slash_fraction: &[Perbill],
		_offence_session: SessionIndex,
	) -> Weight {
		OnOffencePerbill::mutate(|f| {
			*f = slash_fraction.to_vec();
		});

		OffenceWeight::get()
	}
}

pub fn with_on_offence_fractions<R, F: FnOnce(&mut Vec<Perbill>) -> R>(f: F) -> R {
	OnOffencePerbill::mutate(|fractions| f(fractions))
}

type Block = topsoil_core::system::mocking::MockBlock<Runtime>;

topsoil_core::construct_runtime!(
	pub enum Runtime {
		System: topsoil_core::system,
		Offences: offences,
	}
);

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Runtime {
	type Nonce = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type DbWeight = RocksDbWeight;
	type MaxConsumers = ConstU32<16>;
}

impl Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type IdentificationTuple = u64;
	type OnOffenceHandler = OnOffenceHandler;
}

pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let t = topsoil_core::system::GenesisConfig::<Runtime>::default().build_storage().unwrap();
	let mut ext = subsoil::io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub const KIND: [u8; 16] = *b"test_report_1234";

/// Returns all offence details for the specific `kind` happened at the specific time slot.
pub fn offence_reports(kind: Kind, time_slot: u128) -> Vec<OffenceDetails<u64, u64>> {
	<crate::ConcurrentReportsIndex<Runtime>>::get(&kind, &time_slot.encode())
		.into_iter()
		.map(|report_id| {
			<crate::Reports<Runtime>>::get(&report_id)
				.expect("dangling report id is found in ConcurrentReportsIndex")
		})
		.collect()
}

#[derive(Clone)]
pub struct Offence {
	pub validator_set_count: u32,
	pub offenders: Vec<u64>,
	pub time_slot: u128,
}

impl offence::Offence<u64> for Offence {
	const ID: offence::Kind = KIND;
	type TimeSlot = u128;

	fn offenders(&self) -> Vec<u64> {
		self.offenders.clone()
	}

	fn validator_set_count(&self) -> u32 {
		self.validator_set_count
	}

	fn time_slot(&self) -> u128 {
		self.time_slot
	}

	fn session_index(&self) -> SessionIndex {
		1
	}

	fn slash_fraction(&self, offenders_count: u32) -> Perbill {
		Perbill::from_percent(5 + offenders_count * 100 / self.validator_set_count)
	}
}
