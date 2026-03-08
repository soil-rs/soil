// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::*;
use core::ops::Add;
use mock::*;
use subsoil::io::hashing::blake2_256;
use subsoil::keyring::Sr25519Keyring;
use subsoil::runtime::{
	generic::Era,
	traits::{Applyable, Checkable, Hash, IdentityLookup},
	DispatchErrorWithPostInfo, MultiSignature,
};
use topsoil_support::traits::tokens::fungible::Inspect;
type VerifySignatureExt = plant_verify_signature::VerifySignature<Runtime>;

fn create_tx_bare_ext(account: AccountId) -> TxBareExtension {
	(
		topsoil_system::CheckNonZeroSender::<Runtime>::new(),
		topsoil_system::CheckSpecVersion::<Runtime>::new(),
		topsoil_system::CheckTxVersion::<Runtime>::new(),
		topsoil_system::CheckGenesis::<Runtime>::new(),
		topsoil_system::CheckMortality::<Runtime>::from(Era::immortal()),
		topsoil_system::CheckNonce::<Runtime>::from(
			topsoil_system::Pallet::<Runtime>::account(&account).nonce,
		),
		topsoil_system::CheckWeight::<Runtime>::new(),
		plant_transaction_payment::ChargeTransactionPayment::<Runtime>::from(0),
	)
}

pub fn create_meta_tx_bare_ext(account: AccountId) -> MetaTxBareExtension {
	(
		MetaTxMarker::new(),
		topsoil_system::CheckNonZeroSender::<Runtime>::new(),
		topsoil_system::CheckSpecVersion::<Runtime>::new(),
		topsoil_system::CheckTxVersion::<Runtime>::new(),
		topsoil_system::CheckGenesis::<Runtime>::new(),
		topsoil_system::CheckMortality::<Runtime>::from(Era::immortal()),
		topsoil_system::CheckNonce::<Runtime>::from(
			topsoil_system::Pallet::<Runtime>::account(&account).nonce,
		),
	)
}

fn create_signature<Call: Encode, Ext: Encode + TransactionExtension<RuntimeCall>>(
	call: Call,
	ext: Ext,
	signer: Sr25519Keyring,
) -> MultiSignature {
	MultiSignature::Sr25519(
		(META_EXTENSION_VERSION, call, ext.clone(), ext.implicit().unwrap())
			.using_encoded(|e| signer.sign(&blake2_256(e))),
	)
}

fn force_set_balance(account: AccountId) -> Balance {
	let balance = Balances::minimum_balance() * 100;
	Balances::force_set_balance(RuntimeOrigin::root(), account.into(), balance).unwrap();
	balance
}

fn apply_extrinsic(uxt: UncheckedExtrinsic) -> DispatchResultWithPostInfo {
	let uxt_info = uxt.get_dispatch_info();
	let uxt_len = uxt.using_encoded(|e| e.len());
	let xt = <UncheckedExtrinsic as Checkable<IdentityLookup<AccountId>>>::check(
		uxt,
		&Default::default(),
	)
	.unwrap();
	xt.apply::<Runtime>(&uxt_info, uxt_len).unwrap()
}

#[docify::export]
#[test]
fn sign_and_execute_meta_tx() {
	new_test_ext().execute_with(|| {
		// meta tx signer
		let alice_keyring = Sr25519Keyring::Alice;
		// meta tx relayer
		let bob_keyring = Sr25519Keyring::Bob;

		let alice_account: AccountId = alice_keyring.public().into();
		let bob_account: AccountId = bob_keyring.public().into();

		let tx_fee: Balance = (2 * TX_FEE).into(); // base tx fee + weight fee
		let alice_balance = force_set_balance(alice_account.clone());
		let bob_balance = force_set_balance(bob_account.clone());

		// Alice builds a meta transaction.

		let remark_call =
			RuntimeCall::System(topsoil_system::Call::remark_with_event { remark: vec![1] });
		let meta_tx_bare_ext = create_meta_tx_bare_ext(alice_account.clone());
		let meta_tx_sig =
			create_signature(remark_call.clone(), meta_tx_bare_ext.clone(), alice_keyring);
		let meta_tx_ext = (
			VerifySignatureExt::new_with_signature(meta_tx_sig, alice_account.clone()),
			// append signed part.
			meta_tx_bare_ext,
		);

		let meta_tx = MetaTxFor::<Runtime>::new(
			remark_call.clone(),
			META_EXTENSION_VERSION,
			meta_tx_ext.clone(),
		);

		// Encode and share with the world.
		let meta_tx_encoded = meta_tx.encode();

		// Bob acts as meta transaction relayer.

		let meta_tx = MetaTxFor::<Runtime>::decode(&mut &meta_tx_encoded[..]).unwrap();
		let call = RuntimeCall::MetaTx(Call::dispatch {
			meta_tx: Box::new(meta_tx.clone()),
			meta_tx_encoded_len: meta_tx.encoded_size() as u32,
		});
		let tx_bare_ext = create_tx_bare_ext(bob_account.clone());
		let tx_sig = create_signature(call.clone(), tx_bare_ext.clone(), bob_keyring);
		let tx_ext = (
			VerifySignatureExt::new_with_signature(tx_sig, bob_account.clone()),
			// append signed part
			tx_bare_ext,
		);

		let uxt = UncheckedExtrinsic::new_transaction(call.clone(), tx_ext.clone());

		// Check Extrinsic validity and apply it.
		let result = apply_extrinsic(uxt);

		// Asserting the results and make sure the weight is correct.

		let tx_weight = tx_ext.weight(&call)
			+ <Runtime as Config>::WeightInfo::bare_dispatch(meta_tx.encoded_size() as u32);
		let meta_tx_weight = remark_call
			.get_dispatch_info()
			.call_weight
			.add(meta_tx_ext.weight(&remark_call));

		assert_eq!(
			result,
			Ok(PostDispatchInfo {
				actual_weight: Some(meta_tx_weight + tx_weight),
				pays_fee: Pays::Yes,
			})
		);

		System::assert_has_event(RuntimeEvent::MetaTx(crate::Event::Dispatched {
			result: Ok(PostDispatchInfo {
				actual_weight: Some(meta_tx_weight),
				pays_fee: Pays::Yes,
			}),
		}));

		System::assert_has_event(RuntimeEvent::System(topsoil_system::Event::Remarked {
			sender: alice_account.clone(),
			hash: <Runtime as topsoil_system::Config>::Hashing::hash(&[1]),
		}));

		// Alice balance is unchanged, Bob paid the transaction fee.
		assert_eq!(alice_balance, Balances::free_balance(alice_account));
		assert_eq!(bob_balance - tx_fee, Balances::free_balance(bob_account));
	});
}

#[test]
fn invalid_signature() {
	new_test_ext().execute_with(|| {
		// meta tx signer
		let alice_keyring = Sr25519Keyring::Alice;
		// meta tx relayer
		let bob_keyring = Sr25519Keyring::Bob;

		let alice_account: AccountId = alice_keyring.public().into();
		let bob_account: AccountId = bob_keyring.public().into();

		let tx_fee: Balance = (2 * TX_FEE).into(); // base tx fee + weight fee
		let alice_balance = force_set_balance(alice_account.clone());
		let bob_balance = force_set_balance(bob_account.clone());

		// Alice builds a meta transaction.

		let remark_call =
			RuntimeCall::System(topsoil_system::Call::remark_with_event { remark: vec![1] });
		let meta_tx_bare_ext = create_meta_tx_bare_ext(alice_account.clone());
		// signature is invalid since it's signed by charlie instead of alice.
		let invalid_meta_tx_sig = create_signature(
			remark_call.clone(),
			meta_tx_bare_ext.clone(),
			Sr25519Keyring::Charlie,
		);
		let meta_tx_ext = (
			VerifySignatureExt::new_with_signature(invalid_meta_tx_sig, alice_account.clone()),
			// append signed part.
			meta_tx_bare_ext,
		);

		let meta_tx = MetaTxFor::<Runtime>::new(
			remark_call.clone(),
			META_EXTENSION_VERSION,
			meta_tx_ext.clone(),
		);

		// Encode and share with the world.
		let meta_tx_encoded = meta_tx.encode();

		// Bob acts as meta transaction relayer.

		let meta_tx = MetaTxFor::<Runtime>::decode(&mut &meta_tx_encoded[..]).unwrap();
		let call = RuntimeCall::MetaTx(Call::dispatch {
			meta_tx: Box::new(meta_tx.clone()),
			meta_tx_encoded_len: meta_tx.encoded_size() as u32,
		});
		let tx_bare_ext = create_tx_bare_ext(bob_account.clone());
		let tx_sig = create_signature(call.clone(), tx_bare_ext.clone(), bob_keyring);
		let tx_ext = (
			VerifySignatureExt::new_with_signature(tx_sig, bob_account.clone()),
			// append signed part
			tx_bare_ext,
		);

		let uxt = UncheckedExtrinsic::new_transaction(call, tx_ext);

		// Check Extrinsic validity and apply it.
		let result = apply_extrinsic(uxt);

		// Asserting the results.

		assert_eq!(result.unwrap_err().error, Error::<Runtime>::BadProof.into());

		// Alice balance is unchanged, Bob paid the transaction fee.
		assert_eq!(alice_balance, Balances::free_balance(alice_account));
		assert_eq!(bob_balance - tx_fee, Balances::free_balance(bob_account));
	});
}

#[cfg(not(feature = "runtime-benchmarks"))]
#[test]
fn meta_tx_extension_work() {
	new_test_ext().execute_with(|| {
		// meta tx signer
		let alice_keyring = Sr25519Keyring::Alice;
		// meta tx relayer
		let bob_keyring = Sr25519Keyring::Bob;

		let alice_account: AccountId = alice_keyring.public().into();
		let bob_account: AccountId = bob_keyring.public().into();

		let tx_fee: Balance = (2 * TX_FEE).into(); // base tx fee + weight fee
		let alice_balance = force_set_balance(alice_account.clone());
		let bob_balance = force_set_balance(bob_account.clone());

		// Alice builds a meta transaction.

		let remark_call =
			RuntimeCall::System(topsoil_system::Call::remark_with_event { remark: vec![1] });

		let meta_tx_bare_ext = create_meta_tx_bare_ext(alice_account.clone());
		let meta_tx_sig =
			create_signature(remark_call.clone(), meta_tx_bare_ext.clone(), alice_keyring);
		let meta_tx_ext = (
			VerifySignatureExt::new_with_signature(meta_tx_sig, alice_account.clone()),
			// append signed part.
			meta_tx_bare_ext,
		);

		let meta_tx = MetaTxFor::<Runtime>::new(remark_call, META_EXTENSION_VERSION, meta_tx_ext);

		// Encode and share with the world.
		let meta_tx_encoded = meta_tx.encode();

		// Bob acts as meta transaction relayer.

		let meta_tx = MetaTxFor::<Runtime>::decode(&mut &meta_tx_encoded[..]).unwrap();
		let call = RuntimeCall::MetaTx(Call::dispatch {
			meta_tx: Box::new(meta_tx.clone()),
			meta_tx_encoded_len: meta_tx.encoded_size() as u32,
		});
		let tx_bare_ext = create_tx_bare_ext(bob_account.clone());
		let tx_sig = create_signature(call.clone(), tx_bare_ext.clone(), bob_keyring);
		let tx_ext = (
			VerifySignatureExt::new_with_signature(tx_sig, bob_account.clone()),
			// append signed part
			tx_bare_ext,
		);

		let uxt = UncheckedExtrinsic::new_transaction(call, tx_ext);

		// increment alice's nonce to invalidate the meta tx and verify that the
		// meta tx extension works.
		topsoil_system::Pallet::<Runtime>::inc_account_nonce(alice_account.clone());

		// Check Extrinsic validity and apply it.
		let result = apply_extrinsic(uxt);

		// Asserting the results.
		assert_eq!(result.unwrap_err().error, Error::<Runtime>::Stale.into());

		// Alice balance is unchanged, Bob paid the transaction fee.
		assert_eq!(alice_balance, Balances::free_balance(alice_account));
		assert_eq!(bob_balance - tx_fee, Balances::free_balance(bob_account));
	});
}

#[test]
fn meta_tx_call_fails() {
	new_test_ext().execute_with(|| {
		// meta tx signer
		let alice_keyring = Sr25519Keyring::Alice;
		// meta tx relayer
		let bob_keyring = Sr25519Keyring::Bob;

		let alice_account: AccountId = alice_keyring.public().into();
		let bob_account: AccountId = bob_keyring.public().into();

		let tx_fee: Balance = (2 * TX_FEE).into(); // base tx fee + weight fee
		let alice_balance = force_set_balance(alice_account.clone());
		let bob_balance = force_set_balance(bob_account.clone());

		// Alice builds a meta transaction.

		// transfer more than alice has
		let transfer_call = RuntimeCall::Balances(plant_balances::Call::transfer_allow_death {
			dest: bob_account.clone(),
			value: alice_balance * 2,
		});

		let meta_tx_bare_ext = create_meta_tx_bare_ext(alice_account.clone());
		let meta_tx_sig =
			create_signature(transfer_call.clone(), meta_tx_bare_ext.clone(), alice_keyring);
		let meta_tx_ext = (
			VerifySignatureExt::new_with_signature(meta_tx_sig, alice_account.clone()),
			// append signed part.
			meta_tx_bare_ext,
		);

		let meta_tx = MetaTxFor::<Runtime>::new(
			transfer_call.clone(),
			META_EXTENSION_VERSION,
			meta_tx_ext.clone(),
		);

		// Encode and share with the world.
		let meta_tx_encoded = meta_tx.encode();

		// Bob acts as meta transaction relayer.

		let meta_tx = MetaTxFor::<Runtime>::decode(&mut &meta_tx_encoded[..]).unwrap();
		let call = RuntimeCall::MetaTx(Call::dispatch {
			meta_tx: Box::new(meta_tx.clone()),
			meta_tx_encoded_len: meta_tx.encoded_size() as u32,
		});
		let tx_bare_ext = create_tx_bare_ext(bob_account.clone());
		let tx_sig = create_signature(call.clone(), tx_bare_ext.clone(), bob_keyring);
		let tx_ext = (
			VerifySignatureExt::new_with_signature(tx_sig, bob_account.clone()),
			// append signed part
			tx_bare_ext,
		);

		let uxt = UncheckedExtrinsic::new_transaction(call.clone(), tx_ext.clone());

		// Check Extrinsic validity and apply it.
		let result = apply_extrinsic(uxt);

		// Asserting the results and make sure the weight is correct.

		let tx_weight = tx_ext.weight(&call)
			+ <Runtime as Config>::WeightInfo::bare_dispatch(meta_tx.encoded_size() as u32);
		let meta_tx_weight = transfer_call
			.get_dispatch_info()
			.call_weight
			.add(meta_tx_ext.weight(&transfer_call));

		assert_eq!(
			result,
			Ok(PostDispatchInfo {
				actual_weight: Some(meta_tx_weight + tx_weight),
				pays_fee: Pays::Yes,
			})
		);

		System::assert_has_event(RuntimeEvent::MetaTx(crate::Event::Dispatched {
			result: Err(DispatchErrorWithPostInfo {
				post_info: PostDispatchInfo {
					actual_weight: Some(meta_tx_weight),
					pays_fee: Pays::Yes,
				},
				error: subsoil::runtime::DispatchError::Token(
					subsoil::runtime::TokenError::FundsUnavailable,
				),
			}),
		}));

		// Alice balance is unchanged, Bob paid the transaction fee.
		assert_eq!(alice_balance, Balances::free_balance(alice_account));
		assert_eq!(bob_balance - tx_fee, Balances::free_balance(bob_account));
	});
}
