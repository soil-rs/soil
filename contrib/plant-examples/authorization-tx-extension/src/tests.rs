// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: MIT-0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Tests for topsoil-example-authorization-tx-extension.

use codec::Encode;
use subsoil::keyring::Sr25519Keyring;
use subsoil::runtime::{
	generic::ExtensionVersion,
	traits::{Applyable, Checkable, IdentityLookup, TransactionExtension},
	MultiSignature, MultiSigner,
};
use topsoil_support::{
	assert_noop,
	dispatch::GetDispatchInfo,
	pallet_prelude::{InvalidTransaction, TransactionValidityError},
};
use plant_verify_signature::VerifySignature;

use crate::{extensions::AuthorizeCoownership, mock::*, plant_assets};

#[test]
fn create_asset_works() {
	new_test_ext().execute_with(|| {
		let alice_keyring = Sr25519Keyring::Alice;
		let alice_account = AccountId::from(alice_keyring.public());
		// Simple call to create asset with Id `42`.
		let create_asset_call =
			RuntimeCall::Assets(plant_assets::Call::create_asset { asset_id: 42 });
		let ext_version: ExtensionVersion = 0;
		// Create extension that will be used for dispatch.
		let initial_nonce = 23;
		let tx_ext = (
			topsoil_system::CheckNonce::<Runtime>::from(initial_nonce),
			AuthorizeCoownership::<Runtime, MultiSigner, MultiSignature>::default(),
			topsoil_system::CheckGenesis::<Runtime>::new(),
			topsoil_system::CheckTxVersion::<Runtime>::new(),
			topsoil_system::CheckEra::<Runtime>::from(subsoil::runtime::generic::Era::immortal()),
		);
		// Create the transaction signature, to be used in the top level `VerifyMultiSignature`
		// extension.
		let tx_sign = MultiSignature::Sr25519(
			(&(ext_version, &create_asset_call), &tx_ext, tx_ext.implicit().unwrap())
				.using_encoded(|e| alice_keyring.sign(&subsoil::io::hashing::blake2_256(e))),
		);
		// Add the signature to the extension.
		let tx_ext = (
			VerifySignature::new_with_signature(tx_sign, alice_account.clone()),
			topsoil_system::CheckNonce::<Runtime>::from(initial_nonce),
			AuthorizeCoownership::<Runtime, MultiSigner, MultiSignature>::default(),
			topsoil_system::CheckGenesis::<Runtime>::new(),
			topsoil_system::CheckTxVersion::<Runtime>::new(),
			topsoil_system::CheckEra::<Runtime>::from(subsoil::runtime::generic::Era::immortal()),
		);
		// Create the transaction and we're ready for dispatch.
		let uxt = UncheckedExtrinsic::new_transaction(create_asset_call, tx_ext);
		// Check Extrinsic validity and apply it.
		let uxt_info = uxt.get_dispatch_info();
		let uxt_len = uxt.using_encoded(|e| e.len());
		// Manually pay for Alice's nonce.
		topsoil_system::Account::<Runtime>::mutate(&alice_account, |info| {
			info.nonce = initial_nonce;
			info.providers = 1;
		});
		// Check should pass.
		let xt = <UncheckedExtrinsic as Checkable<IdentityLookup<AccountId>>>::check(
			uxt,
			&Default::default(),
		)
		.unwrap();
		// Apply the extrinsic.
		let res = xt.apply::<Runtime>(&uxt_info, uxt_len).unwrap();

		// Asserting the results.
		assert_eq!(
			topsoil_system::Account::<Runtime>::get(&alice_account).nonce,
			initial_nonce + 1
		);
		assert_eq!(
			plant_assets::AssetOwners::<Runtime>::get(42),
			Some(plant_assets::Owner::<AccountId>::Single(alice_account))
		);
		assert!(res.is_ok());
	});
}

#[docify::export]
#[test]
fn create_coowned_asset_works() {
	new_test_ext().execute_with(|| {
		let alice_keyring = Sr25519Keyring::Alice;
		let bob_keyring = Sr25519Keyring::Bob;
		let charlie_keyring = Sr25519Keyring::Charlie;
		let alice_account = AccountId::from(alice_keyring.public());
		let bob_account = AccountId::from(bob_keyring.public());
		let charlie_account = AccountId::from(charlie_keyring.public());
		// Simple call to create asset with Id `42`.
		let create_asset_call =
			RuntimeCall::Assets(plant_assets::Call::create_asset { asset_id: 42 });
		let ext_version: ExtensionVersion = 0;
		// Create the inner transaction extension, to be signed by our coowners, Alice and Bob.
		let inner_ext: InnerTxExtension = (
			topsoil_system::CheckGenesis::<Runtime>::new(),
			topsoil_system::CheckTxVersion::<Runtime>::new(),
			topsoil_system::CheckEra::<Runtime>::from(subsoil::runtime::generic::Era::immortal()),
		);
		// Create the payload Alice and Bob need to sign.
		let inner_payload =
			(&(ext_version, &create_asset_call), &inner_ext, inner_ext.implicit().unwrap());
		// Create Alice's signature.
		let alice_inner_sig = MultiSignature::Sr25519(
			inner_payload
				.using_encoded(|e| alice_keyring.sign(&subsoil::io::hashing::blake2_256(e))),
		);
		// Create Bob's signature.
		let bob_inner_sig = MultiSignature::Sr25519(
			inner_payload.using_encoded(|e| bob_keyring.sign(&subsoil::io::hashing::blake2_256(e))),
		);
		// Create the transaction extension, to be signed by the submitter of the extrinsic, let's
		// have it be Charlie.
		let initial_nonce = 23;
		let tx_ext = (
			topsoil_system::CheckNonce::<Runtime>::from(initial_nonce),
			AuthorizeCoownership::<Runtime, MultiSigner, MultiSignature>::new(
				(alice_keyring.into(), alice_inner_sig.clone()),
				(bob_keyring.into(), bob_inner_sig.clone()),
			),
			topsoil_system::CheckGenesis::<Runtime>::new(),
			topsoil_system::CheckTxVersion::<Runtime>::new(),
			topsoil_system::CheckEra::<Runtime>::from(subsoil::runtime::generic::Era::immortal()),
		);
		// Create Charlie's transaction signature, to be used in the top level
		// `VerifyMultiSignature` extension.
		let tx_sign = MultiSignature::Sr25519(
			(&(ext_version, &create_asset_call), &tx_ext, tx_ext.implicit().unwrap())
				.using_encoded(|e| charlie_keyring.sign(&subsoil::io::hashing::blake2_256(e))),
		);
		// Add the signature to the extension.
		let tx_ext = (
			VerifySignature::new_with_signature(tx_sign, charlie_account.clone()),
			topsoil_system::CheckNonce::<Runtime>::from(initial_nonce),
			AuthorizeCoownership::<Runtime, MultiSigner, MultiSignature>::new(
				(alice_keyring.into(), alice_inner_sig),
				(bob_keyring.into(), bob_inner_sig),
			),
			topsoil_system::CheckGenesis::<Runtime>::new(),
			topsoil_system::CheckTxVersion::<Runtime>::new(),
			topsoil_system::CheckEra::<Runtime>::from(subsoil::runtime::generic::Era::immortal()),
		);
		// Create the transaction and we're ready for dispatch.
		let uxt = UncheckedExtrinsic::new_transaction(create_asset_call, tx_ext);
		// Check Extrinsic validity and apply it.
		let uxt_info = uxt.get_dispatch_info();
		let uxt_len = uxt.using_encoded(|e| e.len());
		// Manually pay for Charlie's nonce.
		topsoil_system::Account::<Runtime>::mutate(&charlie_account, |info| {
			info.nonce = initial_nonce;
			info.providers = 1;
		});
		// Check should pass.
		let xt = <UncheckedExtrinsic as Checkable<IdentityLookup<AccountId>>>::check(
			uxt,
			&Default::default(),
		)
		.unwrap();
		// Apply the extrinsic.
		let res = xt.apply::<Runtime>(&uxt_info, uxt_len).unwrap();

		// Asserting the results.
		assert!(res.is_ok());
		assert_eq!(
			topsoil_system::Account::<Runtime>::get(charlie_account).nonce,
			initial_nonce + 1
		);
		assert_eq!(
			plant_assets::AssetOwners::<Runtime>::get(42),
			Some(plant_assets::Owner::<AccountId>::Double(alice_account, bob_account))
		);
	});
}

#[test]
fn inner_authorization_works() {
	new_test_ext().execute_with(|| {
		let alice_keyring = Sr25519Keyring::Alice;
		let bob_keyring = Sr25519Keyring::Bob;
		let charlie_keyring = Sr25519Keyring::Charlie;
		let charlie_account = AccountId::from(charlie_keyring.public());
		// Simple call to create asset with Id `42`.
		let create_asset_call =
			RuntimeCall::Assets(plant_assets::Call::create_asset { asset_id: 42 });
		let ext_version: ExtensionVersion = 0;
		// Create the inner transaction extension, to be signed by our coowners, Alice and Bob. They
		// are going to sign this transaction as a mortal one.
		let inner_ext: InnerTxExtension = (
			topsoil_system::CheckGenesis::<Runtime>::new(),
			topsoil_system::CheckTxVersion::<Runtime>::new(),
			// Sign with mortal era check.
			topsoil_system::CheckEra::<Runtime>::from(subsoil::runtime::generic::Era::mortal(4, 0)),
		);
		// Create the payload Alice and Bob need to sign.
		let inner_payload = (&create_asset_call, &inner_ext, inner_ext.implicit().unwrap());
		// Create Alice's signature.
		let alice_inner_sig = MultiSignature::Sr25519(
			inner_payload
				.using_encoded(|e| alice_keyring.sign(&subsoil::io::hashing::blake2_256(e))),
		);
		// Create Bob's signature.
		let bob_inner_sig = MultiSignature::Sr25519(
			inner_payload.using_encoded(|e| bob_keyring.sign(&subsoil::io::hashing::blake2_256(e))),
		);
		// Create the transaction extension, to be signed by the submitter of the extrinsic, let's
		// have it be Charlie.
		let initial_nonce = 23;
		let tx_ext = (
			topsoil_system::CheckNonce::<Runtime>::from(initial_nonce),
			AuthorizeCoownership::<Runtime, MultiSigner, MultiSignature>::new(
				(alice_keyring.into(), alice_inner_sig.clone()),
				(bob_keyring.into(), bob_inner_sig.clone()),
			),
			topsoil_system::CheckGenesis::<Runtime>::new(),
			topsoil_system::CheckTxVersion::<Runtime>::new(),
			// Construct the transaction as immortal with a different era check.
			topsoil_system::CheckEra::<Runtime>::from(subsoil::runtime::generic::Era::immortal()),
		);
		// Create Charlie's transaction signature, to be used in the top level
		// `VerifyMultiSignature` extension.
		let tx_sign = MultiSignature::Sr25519(
			(&(ext_version, &create_asset_call), &tx_ext, tx_ext.implicit().unwrap())
				.using_encoded(|e| charlie_keyring.sign(&subsoil::io::hashing::blake2_256(e))),
		);
		// Add the signature to the extension that Charlie signed.
		let tx_ext = (
			VerifySignature::new_with_signature(tx_sign, charlie_account.clone()),
			topsoil_system::CheckNonce::<Runtime>::from(initial_nonce),
			AuthorizeCoownership::<Runtime, MultiSigner, MultiSignature>::new(
				(alice_keyring.into(), alice_inner_sig),
				(bob_keyring.into(), bob_inner_sig),
			),
			topsoil_system::CheckGenesis::<Runtime>::new(),
			topsoil_system::CheckTxVersion::<Runtime>::new(),
			// Construct the transaction as immortal with a different era check.
			topsoil_system::CheckEra::<Runtime>::from(subsoil::runtime::generic::Era::immortal()),
		);
		// Create the transaction and we're ready for dispatch.
		let uxt = UncheckedExtrinsic::new_transaction(create_asset_call, tx_ext);
		// Check Extrinsic validity and apply it.
		let uxt_info = uxt.get_dispatch_info();
		let uxt_len = uxt.using_encoded(|e| e.len());
		// Manually pay for Charlie's nonce.
		topsoil_system::Account::<Runtime>::mutate(charlie_account, |info| {
			info.nonce = initial_nonce;
			info.providers = 1;
		});
		// Check should pass.
		let xt = <UncheckedExtrinsic as Checkable<IdentityLookup<AccountId>>>::check(
			uxt,
			&Default::default(),
		)
		.unwrap();
		// The extrinsic should fail as the signature for the `AuthorizeCoownership` doesn't work
		// for the provided payload with the changed transaction mortality.
		assert_noop!(
			xt.apply::<Runtime>(&uxt_info, uxt_len),
			TransactionValidityError::Invalid(InvalidTransaction::Custom(100))
		);
	});
}
