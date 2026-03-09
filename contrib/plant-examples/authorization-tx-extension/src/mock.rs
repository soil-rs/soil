// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: MIT-0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::*;
pub(crate) use example_runtime::*;
use extensions::AuthorizeCoownership;
use subsoil::runtime::{
	generic,
	traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify},
	BuildStorage, MultiSignature, MultiSigner,
};
use topsoil_core::derive_impl;
use topsoil_core::system::{CheckEra, CheckGenesis, CheckNonce, CheckTxVersion};
use plant_verify_signature::VerifySignature;

#[docify::export]
mod example_runtime {
	use super::*;

	/// Our `TransactionExtension` fit for general transactions.
	pub type TxExtension = (
		// Validate the signature of regular account transactions (substitutes the old signed
		// transaction).
		VerifySignature<Runtime>,
		// Nonce check (and increment) for the caller.
		CheckNonce<Runtime>,
		// If activated, will mutate the origin to a `pallet_coownership` origin of 2 accounts that
		// own something.
		AuthorizeCoownership<Runtime, MultiSigner, MultiSignature>,
		// Some other extensions that we want to run for every possible origin and we want captured
		// in any and all signature and authorization schemes (such as the traditional account
		// signature or the double signature in `pallet_coownership`).
		CheckGenesis<Runtime>,
		CheckTxVersion<Runtime>,
		CheckEra<Runtime>,
	);
	/// Convenience type to more easily construct the signature to be signed in case
	/// `AuthorizeCoownership` is activated.
	pub type InnerTxExtension = (CheckGenesis<Runtime>, CheckTxVersion<Runtime>, CheckEra<Runtime>);
	pub type UncheckedExtrinsic =
		generic::UncheckedExtrinsic<AccountId, RuntimeCall, Signature, TxExtension>;
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
	pub type Signature = MultiSignature;
	pub type BlockNumber = u32;

	// For testing the pallet, we construct a mock runtime.
	topsoil_core::construct_runtime!(
		pub enum Runtime
		{
			System: topsoil_core::system,
			VerifySignaturePallet: plant_verify_signature,

			Assets: plant_assets,
			Coownership: pallet_coownership,
		}
	);

	#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
	impl topsoil_core::system::Config for Runtime {
		type AccountId = AccountId;
		type Block = Block;
		type Lookup = IdentityLookup<Self::AccountId>;
	}

	impl plant_verify_signature::Config for Runtime {
		type Signature = MultiSignature;
		type AccountIdentifier = MultiSigner;
		type WeightInfo = ();
		#[cfg(feature = "runtime-benchmarks")]
		type BenchmarkHelper = ();
	}

	/// Type that enables any pallet to ask for a coowner origin.
	pub struct EnsureCoowner;
	impl EnsureOrigin<RuntimeOrigin> for EnsureCoowner {
		type Success = (AccountId, AccountId);

		fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
			match o.clone().into() {
				Ok(pallet_coownership::Origin::<Runtime>::Coowners(first, second)) => {
					Ok((first, second))
				},
				_ => Err(o),
			}
		}

		#[cfg(feature = "runtime-benchmarks")]
		fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
			unimplemented!()
		}
	}

	impl plant_assets::Config for Runtime {
		type CoownerOrigin = EnsureCoowner;
	}

	impl pallet_coownership::Config for Runtime {
		type RuntimeOrigin = RuntimeOrigin;
		type PalletsOrigin = OriginCaller;
	}
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let t = RuntimeGenesisConfig {
		// We use default for brevity, but you can configure as desired if needed.
		system: Default::default(),
	}
	.build_storage()
	.unwrap();
	t.into()
}
