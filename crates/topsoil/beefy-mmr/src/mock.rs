// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
use topsoil_support::{
	construct_runtime, derive_impl, parameter_types,
	traits::{ConstU32, ConstU64},
};

use crate as topsoil_beefy_mmr;

pub use subsoil::consensus::beefy::{
	ecdsa_crypto::AuthorityId as BeefyId, mmr::BeefyDataProvider, ConsensusLog, BEEFY_ENGINE_ID,
};
use subsoil::core::offchain::{testing::TestOffchainExt, OffchainDbExt, OffchainWorkerExt};

subsoil::impl_opaque_keys! {
	pub struct MockSessionKeys {
		pub dummy: topsoil_beefy::Pallet<Test>,
	}
}

type Block = topsoil_system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test
	{
		System: topsoil_system,
		Session: topsoil_session,
		Balances: topsoil_balances,
		Mmr: topsoil_mmr,
		Beefy: topsoil_beefy,
		BeefyMmr: topsoil_beefy_mmr,
	}
);
#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Test {
	type AccountData = topsoil_balances::AccountData<u64>;
	type Block = Block;
}

#[derive_impl(topsoil_balances::config_preludes::TestDefaultConfig)]
impl topsoil_balances::Config for Test {
	type AccountStore = System;
}

impl topsoil_session::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = u64;
	type ValidatorIdOf = ConvertInto;
	type ShouldEndSession = topsoil_session::PeriodicSessions<ConstU64<1>, ConstU64<0>>;
	type NextSessionRotation = topsoil_session::PeriodicSessions<ConstU64<1>, ConstU64<0>>;
	type SessionManager = MockSessionManager;
	type SessionHandler = <MockSessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type Keys = MockSessionKeys;
	type DisablingStrategy = ();
	type WeightInfo = ();
	type Currency = Balances;
	type KeyDeposit = ();
}

pub type MmrLeaf = subsoil::consensus::beefy::mmr::MmrLeaf<
	topsoil_system::pallet_prelude::BlockNumberFor<Test>,
	<Test as topsoil_system::Config>::Hash,
	crate::MerkleRootOf<Test>,
	Vec<u8>,
>;

impl topsoil_mmr::Config for Test {
	const INDEXING_PREFIX: &'static [u8] = b"mmr";

	type Hashing = Keccak256;

	type LeafData = BeefyMmr;

	type OnNewRoot = topsoil_beefy_mmr::DepositBeefyDigest<Test>;

	type BlockHashProvider = topsoil_mmr::DefaultBlockHashProvider<Test>;

	type WeightInfo = ();

	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

impl topsoil_beefy::Config for Test {
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

impl topsoil_beefy_mmr::Config for Test {
	type LeafVersion = LeafVersion;

	type BeefyAuthorityToMerkleLeaf = topsoil_beefy_mmr::BeefyEcdsaToEthereum;

	type LeafExtra = Vec<u8>;

	type BeefyDataProvider = DummyDataProvider;
	type WeightInfo = ();
}

pub struct DummyDataProvider;
impl BeefyDataProvider<Vec<u8>> for DummyDataProvider {
	fn extra_data() -> Vec<u8> {
		let mut col = vec![(15, vec![1, 2, 3]), (5, vec![4, 5, 6])];
		col.sort();
		subsoil::binary_merkle_tree::merkle_root::<<Test as topsoil_mmr::Config>::Hashing, _>(
			col.into_iter().map(|pair| pair.encode()),
		)
		.as_ref()
		.to_vec()
	}
}

pub struct MockSessionManager;
impl topsoil_session::SessionManager<u64> for MockSessionManager {
	fn end_session(_: soil_staking::SessionIndex) {}
	fn start_session(_: soil_staking::SessionIndex) {}
	fn new_session(idx: soil_staking::SessionIndex) -> Option<Vec<u64>> {
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
	let mut t = topsoil_system::GenesisConfig::<Test>::default().build_storage().unwrap();

	let session_keys: Vec<_> = authorities
		.iter()
		.enumerate()
		.map(|(_, id)| (id.0 as u64, id.0 as u64, MockSessionKeys { dummy: id.1.clone() }))
		.collect();

	BasicExternalities::execute_with_storage(&mut t, || {
		for (ref id, ..) in &session_keys {
			topsoil_system::Pallet::<Test>::inc_providers(id);
		}
	});

	topsoil_session::GenesisConfig::<Test> { keys: session_keys, ..Default::default() }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext: TestExternalities = t.into();
	let (offchain, _offchain_state) = TestOffchainExt::with_offchain_db(ext.offchain_db());
	ext.register_extension(OffchainDbExt::new(offchain.clone()));
	ext.register_extension(OffchainWorkerExt::new(offchain));

	ext
}
