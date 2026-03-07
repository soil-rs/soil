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

//! Mock setup for tests.

#![cfg(any(test, feature = "runtime-benchmarks"))]

use crate as plant_meta_tx;
use crate::*;
use subsoil::core::ConstU8;
use subsoil::keystore::{testing::MemoryKeystore, KeystoreExt};
use subsoil::runtime::{
	traits::{IdentifyAccount, IdentityLookup, Verify},
	MultiSignature,
};
use topsoil_support::{
	construct_runtime, derive_impl,
	weights::{FixedFee, NoFee},
};

pub type Balance = u64;

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

#[cfg(feature = "runtime-benchmarks")]
pub type MetaTxExtension = crate::benchmarking::types::WeightlessExtension<Runtime>;

#[cfg(not(feature = "runtime-benchmarks"))]
pub use tx_ext::*;

#[cfg(not(feature = "runtime-benchmarks"))]
mod tx_ext {
	use super::*;

	pub type UncheckedExtrinsic = subsoil::runtime::generic::UncheckedExtrinsic<
		AccountId,
		RuntimeCall,
		Signature,
		TxExtension,
	>;

	/// Transaction extension.
	pub type TxExtension = (plant_verify_signature::VerifySignature<Runtime>, TxBareExtension);

	/// Transaction extension without signature information.
	///
	/// Helper type used to decode the part of the extension which should be signed.
	pub type TxBareExtension = (
		topsoil_system::CheckNonZeroSender<Runtime>,
		topsoil_system::CheckSpecVersion<Runtime>,
		topsoil_system::CheckTxVersion<Runtime>,
		topsoil_system::CheckGenesis<Runtime>,
		topsoil_system::CheckMortality<Runtime>,
		topsoil_system::CheckNonce<Runtime>,
		topsoil_system::CheckWeight<Runtime>,
		topsoil_transaction_payment::ChargeTransactionPayment<Runtime>,
	);

	pub const META_EXTENSION_VERSION: ExtensionVersion = 0;

	/// Meta transaction extension.
	pub type MetaTxExtension =
		(plant_verify_signature::VerifySignature<Runtime>, MetaTxBareExtension);

	/// Meta transaction extension without signature information.
	///
	/// Helper type used to decode the part of the extension which should be signed.
	pub type MetaTxBareExtension = (
		MetaTxMarker<Runtime>,
		topsoil_system::CheckNonZeroSender<Runtime>,
		topsoil_system::CheckSpecVersion<Runtime>,
		topsoil_system::CheckTxVersion<Runtime>,
		topsoil_system::CheckGenesis<Runtime>,
		topsoil_system::CheckMortality<Runtime>,
		topsoil_system::CheckNonce<Runtime>,
	);
}

impl Config for Runtime {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type Extension = MetaTxExtension;
}

impl plant_verify_signature::Config for Runtime {
	type Signature = MultiSignature;
	type AccountIdentifier = <Signature as Verify>::Signer;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Runtime {
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = topsoil_system::mocking::MockBlock<Runtime>;
	type AccountData = topsoil_balances::AccountData<<Self as topsoil_balances::Config>::Balance>;
}

#[derive_impl(topsoil_balances::config_preludes::TestDefaultConfig)]
impl topsoil_balances::Config for Runtime {
	type ReserveIdentifier = [u8; 8];
	type AccountStore = System;
}

pub const TX_FEE: u32 = 10;

impl topsoil_transaction_payment::Config for Runtime {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = topsoil_transaction_payment::FungibleAdapter<Balances, ()>;
	type OperationalFeeMultiplier = ConstU8<1>;
	type WeightToFee = FixedFee<TX_FEE, Balance>;
	type LengthToFee = NoFee<Balance>;
	type FeeMultiplierUpdate = ();
}

construct_runtime!(
	pub enum Runtime {
		System: topsoil_system,
		Balances: topsoil_balances,
		MetaTx: plant_meta_tx,
		TxPayment: topsoil_transaction_payment,
		VerifySignature: plant_verify_signature,
	}
);

pub(crate) fn new_test_ext() -> subsoil::io::TestExternalities {
	let mut ext = subsoil::io::TestExternalities::new(Default::default());
	ext.execute_with(|| {
		topsoil_system::GenesisConfig::<Runtime>::default().build();
		System::set_block_number(1);
	});
	ext.register_extension(KeystoreExt::new(MemoryKeystore::new()));
	ext
}
