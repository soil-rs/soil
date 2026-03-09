// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use std::vec;

use codec::Encode;
use subsoil::consensus::beefy::mmr::MmrLeafVersion;
use subsoil::io::TestExternalities;
use subsoil::runtime::{
	app_crypto::ecdsa::Public,
	traits::{ConvertInto, Keccak256, OpaqueKeys},
	BuildStorage,
};
use subsoil::state_machine::BasicExternalities;
use topsoil_core::{
	construct_runtime, derive_impl, parameter_types,
	traits::{ConstU32, ConstU64},
};

use crate as plant_beefy_mmr;

pub use subsoil::consensus::beefy::{
	ecdsa_crypto::AuthorityId as BeefyId, mmr::BeefyDataProvider, ConsensusLog, BEEFY_ENGINE_ID,
};
use subsoil::core::offchain::{testing::TestOffchainExt, OffchainDbExt, OffchainWorkerExt};

subsoil::impl_opaque_keys! {
	pub struct MockSessionKeys {
		pub dummy: plant_beefy::Pallet<Test>,
	}
}

type Block = topsoil_core::system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test
	{
		System: topsoil_core::system,
		Session: plant_session,
		Balances: plant_balances,
		Mmr: plant_mmr,
		Beefy: plant_beefy,
		BeefyMmr: plant_beefy_mmr,
	}
);
#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type AccountData = plant_balances::AccountData<u64>;
	type Block = Block;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Test {
	type AccountStore = System;
}

impl plant_session::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = u64;
	type ValidatorIdOf = ConvertInto;
	type ShouldEndSession = plant_session::PeriodicSessions<ConstU64<1>, ConstU64<0>>;
	type NextSessionRotation = plant_session::PeriodicSessions<ConstU64<1>, ConstU64<0>>;
	type SessionManager = MockSessionManager;
	type SessionHandler = <MockSessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type Keys = MockSessionKeys;
	type DisablingStrategy = ();
	type WeightInfo = ();
	type Currency = Balances;
	type KeyDeposit = ();
}

pub type MmrLeaf = subsoil::consensus::beefy::mmr::MmrLeaf<
	topsoil_core::system::pallet_prelude::BlockNumberFor<Test>,
	<Test as topsoil_core::system::Config>::Hash,
	crate::MerkleRootOf<Test>,
	Vec<u8>,
>;

impl plant_mmr::Config for Test {
	const INDEXING_PREFIX: &'static [u8] = b"mmr";

	type Hashing = Keccak256;

	type LeafData = BeefyMmr;

	type OnNewRoot = plant_beefy_mmr::DepositBeefyDigest<Test>;

	type BlockHashProvider = plant_mmr::DefaultBlockHashProvider<Test>;

	type WeightInfo = ();

	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

impl plant_beefy::Config for Test {
	type BeefyId = BeefyId;
	type MaxAuthorities = ConstU32<100>;
	type MaxNominators = ConstU32<1000>;
	type MaxSetIdSessionEntries = ConstU64<100>;
	type OnNewValidatorSet = BeefyMmr;
	type AncestryHelper = BeefyMmr;
	type WeightInfo = ();
	type KeyOwnerProof = subsoil::core::Void;
	type EquivocationReportSystem = ();
}

parameter_types! {
	pub LeafVersion: MmrLeafVersion = MmrLeafVersion::new(1, 5);
}

impl plant_beefy_mmr::Config for Test {
	type LeafVersion = LeafVersion;

	type BeefyAuthorityToMerkleLeaf = plant_beefy_mmr::BeefyEcdsaToEthereum;

	type LeafExtra = Vec<u8>;

	type BeefyDataProvider = DummyDataProvider;
	type WeightInfo = ();
}

pub struct DummyDataProvider;
impl BeefyDataProvider<Vec<u8>> for DummyDataProvider {
	fn extra_data() -> Vec<u8> {
		let mut col = vec![(15, vec![1, 2, 3]), (5, vec![4, 5, 6])];
		col.sort();
		subsoil::binary_merkle_tree::merkle_root::<<Test as plant_mmr::Config>::Hashing, _>(
			col.into_iter().map(|pair| pair.encode()),
		)
		.as_ref()
		.to_vec()
	}
}

pub struct MockSessionManager;
impl plant_session::SessionManager<u64> for MockSessionManager {
	fn end_session(_: subsoil::staking::SessionIndex) {}
	fn start_session(_: subsoil::staking::SessionIndex) {}
	fn new_session(idx: subsoil::staking::SessionIndex) -> Option<Vec<u64>> {
		if idx == 0 || idx == 1 {
			Some(vec![1, 2])
		} else if idx == 2 {
			Some(vec![3, 4])
		} else {
			None
		}
	}
}

// Note, that we can't use `UintAuthorityId` here. Reason is that the implementation
// of `to_public_key()` assumes, that a public key is 32 bytes long. This is true for
// ed25519 and sr25519 but *not* for ecdsa. A compressed ecdsa public key is 33 bytes,
// with the first one containing information to reconstruct the uncompressed key.
pub fn mock_beefy_id(id: u8) -> BeefyId {
	let mut buf: [u8; 33] = [id; 33];
	// Set to something valid.
	buf[0] = 0x02;
	let pk = Public::from_raw(buf);
	BeefyId::from(pk)
}

pub fn mock_authorities(vec: Vec<u8>) -> Vec<(u64, BeefyId)> {
	vec.into_iter().map(|id| ((id as u64), mock_beefy_id(id))).collect()
}

pub fn new_test_ext(ids: Vec<u8>) -> TestExternalities {
	new_test_ext_raw_authorities(mock_authorities(ids))
}

pub fn new_test_ext_raw_authorities(authorities: Vec<(u64, BeefyId)>) -> TestExternalities {
	let mut t = topsoil_core::system::GenesisConfig::<Test>::default().build_storage().unwrap();

	let session_keys: Vec<_> = authorities
		.iter()
		.enumerate()
		.map(|(_, id)| (id.0 as u64, id.0 as u64, MockSessionKeys { dummy: id.1.clone() }))
		.collect();

	BasicExternalities::execute_with_storage(&mut t, || {
		for (ref id, ..) in &session_keys {
			topsoil_core::system::Pallet::<Test>::inc_providers(id);
		}
	});

	plant_session::GenesisConfig::<Test> { keys: session_keys, ..Default::default() }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext: TestExternalities = t.into();
	let (offchain, _offchain_state) = TestOffchainExt::with_offchain_db(ext.offchain_db());
	ext.register_extension(OffchainDbExt::new(offchain.clone()));
	ext.register_extension(OffchainWorkerExt::new(offchain));

	ext
}
