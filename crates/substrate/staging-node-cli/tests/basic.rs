// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use codec::{Decode, Encode, Joiner};
use topsoil_support::{
	dispatch::{DispatchClass, GetDispatchInfo},
	traits::Currency,
	weights::Weight,
};
use topsoil_system::{self, AccountInfo, DispatchEventInfo, EventRecord, Phase};
use soil_core::{storage::well_known_keys, traits::Externalities};
use soil_runtime::{
	traits::Hash as HashT, transaction_validity::InvalidTransaction, ApplyExtrinsicResult,
};

use kitchensink_runtime::{
	constants::{currency::*, time::SLOT_DURATION},
	Balances, CheckedExtrinsic, Header, Runtime, RuntimeCall, RuntimeEvent, System,
	TransactionPayment, Treasury, UncheckedExtrinsic,
};
use node_primitives::{Balance, Hash};
use node_testing::keyring::*;
use pretty_assertions::assert_eq;
pub mod common;
use self::common::{sign, *};

/// The wasm runtime binary which hasn't undergone the compacting process.
///
/// The idea here is to pass it as the current runtime code to the executor so the executor will
/// have to execute provided wasm code instead of the native equivalent. This trick is used to
/// test code paths that differ between native and wasm versions.
pub fn bloaty_code_unwrap() -> &'static [u8] {
	kitchensink_runtime::WASM_BINARY_BLOATY.expect(
		"Development wasm binary is not available. \
											 Testing is only supported with the flag disabled.",
	)
}

/// Default transfer fee. This will use the same logic that is implemented in transaction-payment
/// module.
///
/// Note that reads the multiplier from storage directly, hence to get the fee of `extrinsic`
/// at block `n`, it must be called prior to executing block `n` to do the calculation with the
/// correct multiplier.
fn transfer_fee(extrinsic: &UncheckedExtrinsic) -> Balance {
	let mut info = default_transfer_call().get_dispatch_info();
	info.extension_weight = extrinsic.extension_weight();
	TransactionPayment::compute_fee(extrinsic.encode().len() as u32, &info, 0)
}

/// Default transfer fee, same as `transfer_fee`, but with a weight refund factored in.
fn transfer_fee_with_refund(extrinsic: &UncheckedExtrinsic, weight_refund: Weight) -> Balance {
	let mut info = default_transfer_call().get_dispatch_info();
	info.extension_weight = extrinsic.extension_weight();
	let post_info = (Some(info.total_weight().saturating_sub(weight_refund)), info.pays_fee).into();
	TransactionPayment::compute_actual_fee(extrinsic.encode().len() as u32, &info, &post_info, 0)
}

fn xt() -> UncheckedExtrinsic {
	sign(CheckedExtrinsic {
		format: soil_runtime::generic::ExtrinsicFormat::Signed(alice(), tx_ext(0, 0)),
		function: RuntimeCall::Balances(default_transfer_call()),
	})
}

fn set_heap_pages<E: Externalities>(ext: &mut E, heap_pages: u64) {
	ext.place_storage(well_known_keys::HEAP_PAGES.to_vec(), Some(heap_pages.encode()));
}

fn changes_trie_block() -> (Vec<u8>, Hash) {
	let time = 42 * 1000;
	construct_block(
		&mut new_test_ext(compact_code_unwrap()),
		1,
		GENESIS_HASH.into(),
		vec![
			CheckedExtrinsic {
				format: soil_runtime::generic::ExtrinsicFormat::Bare,
				function: RuntimeCall::Timestamp(topsoil_timestamp::Call::set { now: time }),
			},
			CheckedExtrinsic {
				format: soil_runtime::generic::ExtrinsicFormat::Signed(alice(), tx_ext(0, 0)),
				function: RuntimeCall::Balances(topsoil_balances::Call::transfer_allow_death {
					dest: bob().into(),
					value: 69 * DOLLARS,
				}),
			},
		],
		(time / SLOT_DURATION).into(),
	)
}

/// block 1 and 2 must be created together to ensure transactions are only signed once (since they
/// are not guaranteed to be deterministic) and to ensure that the correct state is propagated
/// from block1's execution to block2 to derive the correct storage_root.
fn blocks() -> ((Vec<u8>, Hash), (Vec<u8>, Hash)) {
	let mut t = new_test_ext(compact_code_unwrap());
	let time1 = 42 * 1000;
	let block1 = construct_block(
		&mut t,
		1,
		GENESIS_HASH.into(),
		vec![
			CheckedExtrinsic {
				format: soil_runtime::generic::ExtrinsicFormat::Bare,
				function: RuntimeCall::Timestamp(topsoil_timestamp::Call::set { now: time1 }),
			},
			CheckedExtrinsic {
				format: soil_runtime::generic::ExtrinsicFormat::Signed(alice(), tx_ext(0, 0)),
				function: RuntimeCall::Balances(topsoil_balances::Call::transfer_allow_death {
					dest: bob().into(),
					value: 69 * DOLLARS,
				}),
			},
		],
		(time1 / SLOT_DURATION).into(),
	);
	let time2 = 52 * 1000;
	let block2 = construct_block(
		&mut t,
		2,
		block1.1,
		vec![
			CheckedExtrinsic {
				format: soil_runtime::generic::ExtrinsicFormat::Bare,
				function: RuntimeCall::Timestamp(topsoil_timestamp::Call::set { now: time2 }),
			},
			CheckedExtrinsic {
				format: soil_runtime::generic::ExtrinsicFormat::Signed(bob(), tx_ext(0, 0)),
				function: RuntimeCall::Balances(topsoil_balances::Call::transfer_allow_death {
					dest: alice().into(),
					value: 5 * DOLLARS,
				}),
			},
			CheckedExtrinsic {
				format: soil_runtime::generic::ExtrinsicFormat::Signed(alice(), tx_ext(1, 0)),
				function: RuntimeCall::Balances(topsoil_balances::Call::transfer_allow_death {
					dest: bob().into(),
					value: 15 * DOLLARS,
				}),
			},
		],
		(time2 / SLOT_DURATION).into(),
	);

	// session change => consensus authorities change => authorities change digest item appears
	let digest = Header::decode(&mut &block2.0[..]).unwrap().digest;
	assert_eq!(digest.logs().len(), 2 /* Just babe and BEEFY slots */);

	(block1, block2)
}

fn block_with_size(time: u64, nonce: u32, size: usize) -> (Vec<u8>, Hash) {
	construct_block(
		&mut new_test_ext(compact_code_unwrap()),
		1,
		GENESIS_HASH.into(),
		vec![
			CheckedExtrinsic {
				format: soil_runtime::generic::ExtrinsicFormat::Bare,
				function: RuntimeCall::Timestamp(topsoil_timestamp::Call::set { now: time * 1000 }),
			},
			CheckedExtrinsic {
				format: soil_runtime::generic::ExtrinsicFormat::Signed(alice(), tx_ext(nonce, 0)),
				function: RuntimeCall::System(topsoil_system::Call::remark { remark: vec![0; size] }),
			},
		],
		(time * 1000 / SLOT_DURATION).into(),
	)
}

#[test]
fn panic_execution_with_foreign_code_gives_error() {
	let mut t = new_test_ext(bloaty_code_unwrap());
	t.insert(
		<topsoil_system::Account<Runtime>>::hashed_key_for(alice()),
		AccountInfo::<<Runtime as topsoil_system::Config>::Nonce, _> {
			providers: 1,
			data: (69u128, 0u128, 0u128, 1u128 << 127),
			..Default::default()
		}
		.encode(),
	);
	t.insert(<topsoil_balances::TotalIssuance<Runtime>>::hashed_key().to_vec(), 69_u128.encode());
	t.insert(<topsoil_system::BlockHash<Runtime>>::hashed_key_for(0), vec![0u8; 32]);

	let r = executor_call(&mut t, "Core_initialize_block", &vec![].and(&from_block_number(1u32))).0;
	assert!(r.is_ok());
	let v = executor_call(&mut t, "BlockBuilder_apply_extrinsic", &vec![].and(&xt()))
		.0
		.unwrap();
	let r = ApplyExtrinsicResult::decode(&mut &v[..]).unwrap();
	assert_eq!(r, Err(InvalidTransaction::Payment.into()));
}

#[test]
fn bad_extrinsic_with_native_equivalent_code_gives_error() {
	let mut t = new_test_ext(compact_code_unwrap());
	t.insert(
		<topsoil_system::Account<Runtime>>::hashed_key_for(alice()),
		AccountInfo::<<Runtime as topsoil_system::Config>::Nonce, _> {
			providers: 1,
			data: (69u128, 0u128, 0u128, 1u128 << 127),
			..Default::default()
		}
		.encode(),
	);
	t.insert(<topsoil_balances::TotalIssuance<Runtime>>::hashed_key().to_vec(), 69u128.encode());
	t.insert(<topsoil_system::BlockHash<Runtime>>::hashed_key_for(0), vec![0u8; 32]);

	let r = executor_call(&mut t, "Core_initialize_block", &vec![].and(&from_block_number(1u32))).0;
	assert!(r.is_ok());
	let v = executor_call(&mut t, "BlockBuilder_apply_extrinsic", &vec![].and(&xt()))
		.0
		.unwrap();
	let r = ApplyExtrinsicResult::decode(&mut &v[..]).unwrap();
	assert_eq!(r, Err(InvalidTransaction::Payment.into()));
}

#[test]
fn successful_execution_with_native_equivalent_code_gives_ok() {
	let mut t = new_test_ext(compact_code_unwrap());
	t.insert(
		<topsoil_system::Account<Runtime>>::hashed_key_for(alice()),
		AccountInfo::<<Runtime as topsoil_system::Config>::Nonce, _> {
			providers: 1,
			data: (111 * DOLLARS, 0u128, 0u128, 1u128 << 127),
			..Default::default()
		}
		.encode(),
	);
	t.insert(
		<topsoil_system::Account<Runtime>>::hashed_key_for(bob()),
		AccountInfo::<
			<Runtime as topsoil_system::Config>::Nonce,
			<Runtime as topsoil_system::Config>::AccountData,
		>::default()
		.encode(),
	);
	t.insert(
		<topsoil_balances::TotalIssuance<Runtime>>::hashed_key().to_vec(),
		(111 * DOLLARS).encode(),
	);
	t.insert(<topsoil_system::BlockHash<Runtime>>::hashed_key_for(0), vec![0u8; 32]);

	let r = executor_call(&mut t, "Core_initialize_block", &vec![].and(&from_block_number(1u32))).0;
	assert!(r.is_ok());

	let weight_refund = Weight::zero();
	let fees_after_refund = t.execute_with(|| transfer_fee_with_refund(&xt(), weight_refund));

	let r = executor_call(&mut t, "BlockBuilder_apply_extrinsic", &vec![].and(&xt())).0;
	assert!(r.is_ok());

	t.execute_with(|| {
		assert_eq!(Balances::total_balance(&alice()), 42 * DOLLARS - fees_after_refund);
		assert_eq!(Balances::total_balance(&bob()), 69 * DOLLARS);
	});
}

#[test]
fn successful_execution_with_foreign_code_gives_ok() {
	let mut t = new_test_ext(bloaty_code_unwrap());
	t.insert(
		<topsoil_system::Account<Runtime>>::hashed_key_for(alice()),
		AccountInfo::<<Runtime as topsoil_system::Config>::Nonce, _> {
			providers: 1,
			data: (111 * DOLLARS, 0u128, 0u128, 1u128 << 127),
			..Default::default()
		}
		.encode(),
	);
	t.insert(
		<topsoil_system::Account<Runtime>>::hashed_key_for(bob()),
		AccountInfo::<
			<Runtime as topsoil_system::Config>::Nonce,
			<Runtime as topsoil_system::Config>::AccountData,
		>::default()
		.encode(),
	);
	t.insert(
		<topsoil_balances::TotalIssuance<Runtime>>::hashed_key().to_vec(),
		(111 * DOLLARS).encode(),
	);
	t.insert(<topsoil_system::BlockHash<Runtime>>::hashed_key_for(0), vec![0u8; 32]);

	let r = executor_call(&mut t, "Core_initialize_block", &vec![].and(&from_block_number(1u32))).0;
	assert!(r.is_ok());

	let weight_refund = Weight::zero();
	let fees_after_refund = t.execute_with(|| transfer_fee_with_refund(&xt(), weight_refund));

	let r = executor_call(&mut t, "BlockBuilder_apply_extrinsic", &vec![].and(&xt())).0;
	assert!(r.is_ok());

	t.execute_with(|| {
		assert_eq!(Balances::total_balance(&alice()), 42 * DOLLARS - fees_after_refund);
		assert_eq!(Balances::total_balance(&bob()), 69 * DOLLARS);
	});
}

#[test]
fn full_native_block_import_works() {
	let mut t = new_test_ext(compact_code_unwrap());

	let (block1, block2) = blocks();

	let mut alice_last_known_balance: Balance = Default::default();
	let mut fees = t.execute_with(|| transfer_fee(&xt()));
	let extension_weight = xt().extension_weight();
	let weight_refund = Weight::zero();
	let fees_after_refund = t.execute_with(|| transfer_fee_with_refund(&xt(), weight_refund));

	let transfer_weight = default_transfer_call().get_dispatch_info().call_weight.saturating_add(
		<Runtime as topsoil_system::Config>::BlockWeights::get()
			.get(DispatchClass::Normal)
			.base_extrinsic,
	);
	let timestamp_weight = topsoil_timestamp::Call::set::<Runtime> { now: Default::default() }
		.get_dispatch_info()
		.call_weight
		.saturating_add(
			<Runtime as topsoil_system::Config>::BlockWeights::get()
				.get(DispatchClass::Mandatory)
				.base_extrinsic,
		);

	executor_call(&mut t, "Core_execute_block", &block1.0).0.unwrap();

	t.execute_with(|| {
		assert_eq!(Balances::total_balance(&alice()), 42 * DOLLARS - fees_after_refund);
		assert_eq!(Balances::total_balance(&bob()), 169 * DOLLARS);
		alice_last_known_balance = Balances::total_balance(&alice());
		let events = vec![
			EventRecord {
				phase: Phase::ApplyExtrinsic(0),
				event: RuntimeEvent::System(topsoil_system::Event::ExtrinsicSuccess {
					dispatch_info: DispatchEventInfo {
						weight: timestamp_weight,
						class: DispatchClass::Mandatory,
						pays_fee: Default::default(),
					},
				}),
				topics: vec![],
			},
			EventRecord {
				phase: Phase::ApplyExtrinsic(1),
				event: RuntimeEvent::Balances(topsoil_balances::Event::Withdraw {
					who: alice().into(),
					amount: fees,
				}),
				topics: vec![],
			},
			EventRecord {
				phase: Phase::ApplyExtrinsic(1),
				event: RuntimeEvent::Balances(topsoil_balances::Event::Transfer {
					from: alice().into(),
					to: bob().into(),
					amount: 69 * DOLLARS,
				}),
				topics: vec![],
			},
			EventRecord {
				phase: Phase::ApplyExtrinsic(1),
				event: RuntimeEvent::Balances(topsoil_balances::Event::Deposit {
					who: topsoil_treasury::Pallet::<Runtime>::account_id(),
					amount: fees_after_refund,
				}),
				topics: vec![],
			},
			EventRecord {
				phase: Phase::ApplyExtrinsic(1),
				event: RuntimeEvent::TransactionPayment(
					topsoil_transaction_payment::Event::TransactionFeePaid {
						who: alice().into(),
						actual_fee: fees_after_refund,
						tip: 0,
					},
				),
				topics: vec![],
			},
			EventRecord {
				phase: Phase::ApplyExtrinsic(1),
				event: RuntimeEvent::System(topsoil_system::Event::ExtrinsicSuccess {
					dispatch_info: DispatchEventInfo {
						weight: transfer_weight
							.saturating_add(extension_weight.saturating_sub(weight_refund)),
						..Default::default()
					},
				}),
				topics: vec![],
			},
		];
		let filtered_events: Vec<_> = System::events()
			.into_iter()
			.filter(|ev| {
				!matches!(
					ev.event,
					RuntimeEvent::VoterList(
						topsoil_bags_list::Event::<Runtime, _>::ScoreUpdated { .. }
					)
				)
			})
			.collect();

		assert_eq!(filtered_events, events);
	});

	fees = t.execute_with(|| transfer_fee(&xt()));
	let pot = t.execute_with(|| Treasury::pot());
	let extension_weight = xt().extension_weight();
	let weight_refund = Weight::zero();
	let fees_after_refund = t.execute_with(|| transfer_fee_with_refund(&xt(), weight_refund));

	executor_call(&mut t, "Core_execute_block", &block2.0).0.unwrap();

	t.execute_with(|| {
		assert_eq!(
			Balances::total_balance(&alice()),
			alice_last_known_balance - 10 * DOLLARS - fees_after_refund,
		);
		assert_eq!(Balances::total_balance(&bob()), 179 * DOLLARS - fees_after_refund);
		let events = vec![
			EventRecord {
				phase: Phase::Initialization,
				event: RuntimeEvent::Treasury(topsoil_treasury::Event::UpdatedInactive {
					reactivated: 0,
					deactivated: pot,
				}),
				topics: vec![],
			},
			EventRecord {
				phase: Phase::ApplyExtrinsic(0),
				event: RuntimeEvent::System(topsoil_system::Event::ExtrinsicSuccess {
					dispatch_info: DispatchEventInfo {
						weight: timestamp_weight,
						class: DispatchClass::Mandatory,
						pays_fee: Default::default(),
					},
				}),
				topics: vec![],
			},
			EventRecord {
				phase: Phase::ApplyExtrinsic(1),
				event: RuntimeEvent::Balances(topsoil_balances::Event::Withdraw {
					who: bob().into(),
					amount: fees,
				}),
				topics: vec![],
			},
			EventRecord {
				phase: Phase::ApplyExtrinsic(1),
				event: RuntimeEvent::Balances(topsoil_balances::Event::Transfer {
					from: bob().into(),
					to: alice().into(),
					amount: 5 * DOLLARS,
				}),
				topics: vec![],
			},
			EventRecord {
				phase: Phase::ApplyExtrinsic(1),
				event: RuntimeEvent::Balances(topsoil_balances::Event::Deposit {
					who: topsoil_treasury::Pallet::<Runtime>::account_id(),
					amount: fees_after_refund,
				}),
				topics: vec![],
			},
			EventRecord {
				phase: Phase::ApplyExtrinsic(1),
				event: RuntimeEvent::TransactionPayment(
					topsoil_transaction_payment::Event::TransactionFeePaid {
						who: bob().into(),
						actual_fee: fees_after_refund,
						tip: 0,
					},
				),
				topics: vec![],
			},
			EventRecord {
				phase: Phase::ApplyExtrinsic(1),
				event: RuntimeEvent::System(topsoil_system::Event::ExtrinsicSuccess {
					dispatch_info: DispatchEventInfo {
						weight: transfer_weight
							.saturating_add(extension_weight.saturating_sub(weight_refund)),
						..Default::default()
					},
				}),
				topics: vec![],
			},
			EventRecord {
				phase: Phase::ApplyExtrinsic(2),
				event: RuntimeEvent::Balances(topsoil_balances::Event::Withdraw {
					who: alice().into(),
					amount: fees,
				}),
				topics: vec![],
			},
			EventRecord {
				phase: Phase::ApplyExtrinsic(2),
				event: RuntimeEvent::Balances(topsoil_balances::Event::Transfer {
					from: alice().into(),
					to: bob().into(),
					amount: 15 * DOLLARS,
				}),
				topics: vec![],
			},
			EventRecord {
				phase: Phase::ApplyExtrinsic(2),
				event: RuntimeEvent::Balances(topsoil_balances::Event::Deposit {
					who: topsoil_treasury::Pallet::<Runtime>::account_id(),
					amount: fees_after_refund,
				}),
				topics: vec![],
			},
			EventRecord {
				phase: Phase::ApplyExtrinsic(2),
				event: RuntimeEvent::TransactionPayment(
					topsoil_transaction_payment::Event::TransactionFeePaid {
						who: alice().into(),
						actual_fee: fees_after_refund,
						tip: 0,
					},
				),
				topics: vec![],
			},
			EventRecord {
				phase: Phase::ApplyExtrinsic(2),
				event: RuntimeEvent::System(topsoil_system::Event::ExtrinsicSuccess {
					dispatch_info: DispatchEventInfo {
						weight: transfer_weight
							.saturating_add(extension_weight.saturating_sub(weight_refund)),
						..Default::default()
					},
				}),
				topics: vec![],
			},
		];
		let all_events = System::events();
		// Ensure that all expected events (`events`) are present in the full event log
		// (`all_events`). We use this instead of strict equality since some events (like
		// VoterList::ScoreUpdated) may be emitted non-deterministically depending on runtime
		// internals or auto-rebagging logic.
		for expected_event in &events {
			assert!(
				all_events.contains(expected_event),
				"Expected event {:?} not found in actual events",
				expected_event
			);
		}
	});
}

#[test]
fn full_wasm_block_import_works() {
	let mut t = new_test_ext(compact_code_unwrap());

	let (block1, block2) = blocks();

	let mut alice_last_known_balance: Balance = Default::default();
	let weight_refund = Weight::zero();
	let fees_after_refund = t.execute_with(|| transfer_fee_with_refund(&xt(), weight_refund));

	executor_call(&mut t, "Core_execute_block", &block1.0).0.unwrap();

	t.execute_with(|| {
		assert_eq!(Balances::total_balance(&alice()), 42 * DOLLARS - fees_after_refund);
		assert_eq!(Balances::total_balance(&bob()), 169 * DOLLARS);
		alice_last_known_balance = Balances::total_balance(&alice());
	});

	let weight_refund = Weight::zero();
	let fees_after_refund = t.execute_with(|| transfer_fee_with_refund(&xt(), weight_refund));

	executor_call(&mut t, "Core_execute_block", &block2.0).0.unwrap();

	t.execute_with(|| {
		assert_eq!(
			Balances::total_balance(&alice()),
			alice_last_known_balance - 10 * DOLLARS - fees_after_refund,
		);
		assert_eq!(Balances::total_balance(&bob()), 179 * DOLLARS - 1 * fees_after_refund);
	});
}

#[test]
fn wasm_big_block_import_fails() {
	let mut t = new_test_ext(compact_code_unwrap());

	set_heap_pages(&mut t.ext(), 4);

	let result = executor_call(&mut t, "Core_execute_block", &block_with_size(42, 0, 120_000).0).0;
	assert!(result.is_err()); // Err(Wasmi(Trap(Trap { kind: Host(AllocatorOutOfSpace) })))
}

#[test]
fn native_big_block_import_succeeds() {
	let mut t = new_test_ext(compact_code_unwrap());

	executor_call(&mut t, "Core_execute_block", &block_with_size(42, 0, 120_000).0)
		.0
		.unwrap();
}

#[test]
fn native_big_block_import_fails_on_fallback() {
	let mut t = new_test_ext(compact_code_unwrap());

	// We set the heap pages to 8 because we know that should give an OOM in WASM with the given
	// block.
	set_heap_pages(&mut t.ext(), 8);

	assert!(executor_call(&mut t, "Core_execute_block", &block_with_size(42, 0, 120_000).0)
		.0
		.is_err());
}

#[test]
fn panic_execution_gives_error() {
	let mut t = new_test_ext(bloaty_code_unwrap());
	t.insert(
		<topsoil_system::Account<Runtime>>::hashed_key_for(alice()),
		AccountInfo::<<Runtime as topsoil_system::Config>::Nonce, _> {
			data: (0 * DOLLARS, 0u128, 0u128, 0u128),
			..Default::default()
		}
		.encode(),
	);
	t.insert(<topsoil_balances::TotalIssuance<Runtime>>::hashed_key().to_vec(), 0_u128.encode());
	t.insert(<topsoil_system::BlockHash<Runtime>>::hashed_key_for(0), vec![0u8; 32]);

	let r = executor_call(&mut t, "Core_initialize_block", &vec![].and(&from_block_number(1u32))).0;
	assert!(r.is_ok());
	let r = executor_call(&mut t, "BlockBuilder_apply_extrinsic", &vec![].and(&xt()))
		.0
		.unwrap();
	let r = ApplyExtrinsicResult::decode(&mut &r[..]).unwrap();
	assert_eq!(r, Err(InvalidTransaction::Payment.into()));
}

#[test]
fn successful_execution_gives_ok() {
	let mut t = new_test_ext(compact_code_unwrap());
	t.insert(
		<topsoil_system::Account<Runtime>>::hashed_key_for(alice()),
		AccountInfo::<<Runtime as topsoil_system::Config>::Nonce, _> {
			providers: 1,
			data: (111 * DOLLARS, 0u128, 0u128, 1u128 << 127),
			..Default::default()
		}
		.encode(),
	);
	t.insert(
		<topsoil_system::Account<Runtime>>::hashed_key_for(bob()),
		AccountInfo::<
			<Runtime as topsoil_system::Config>::Nonce,
			<Runtime as topsoil_system::Config>::AccountData,
		>::default()
		.encode(),
	);
	t.insert(
		<topsoil_balances::TotalIssuance<Runtime>>::hashed_key().to_vec(),
		(111 * DOLLARS).encode(),
	);
	t.insert(<topsoil_system::BlockHash<Runtime>>::hashed_key_for(0), vec![0u8; 32]);

	let r = executor_call(&mut t, "Core_initialize_block", &vec![].and(&from_block_number(1u32))).0;
	assert!(r.is_ok());
	t.execute_with(|| {
		assert_eq!(Balances::total_balance(&alice()), 111 * DOLLARS);
	});

	let weight_refund = Weight::zero();
	let fees_after_refund = t.execute_with(|| transfer_fee_with_refund(&xt(), weight_refund));

	let r = executor_call(&mut t, "BlockBuilder_apply_extrinsic", &vec![].and(&xt()))
		.0
		.unwrap();
	ApplyExtrinsicResult::decode(&mut &r[..])
		.unwrap()
		.expect("Extrinsic could not be applied")
		.expect("Extrinsic failed");

	t.execute_with(|| {
		assert_eq!(Balances::total_balance(&alice()), 42 * DOLLARS - fees_after_refund);
		assert_eq!(Balances::total_balance(&bob()), 69 * DOLLARS);
	});
}

#[test]
fn should_import_block_with_test_client() {
	use node_testing::client::{
		soil_consensus::BlockOrigin, ClientBlockImportExt, TestClientBuilder, TestClientBuilderExt,
	};

	let client = TestClientBuilder::new().build();
	let block1 = changes_trie_block();
	let block_data = block1.0;
	let block = node_primitives::Block::decode(&mut &block_data[..]).unwrap();

	futures::executor::block_on(client.import(BlockOrigin::Own, block)).unwrap();
}

#[test]
fn default_config_as_json_works() {
	let mut t = new_test_ext(compact_code_unwrap());
	let r = executor_call(
		&mut t,
		"GenesisBuilder_get_preset",
		&None::<&soil_genesis_builder::PresetId>.encode(),
	)
	.0
	.unwrap();
	let r = Option::<Vec<u8>>::decode(&mut &r[..])
		.unwrap()
		.expect("default config is there");
	let json = String::from_utf8(r.into()).expect("returned value is json. qed.");
	let expected = include_str!("res/default_genesis_config.json").to_string();

	assert_eq!(
		serde_json::from_str::<serde_json::Value>(&expected).unwrap(),
		serde_json::from_str::<serde_json::Value>(&json).unwrap()
	);
}
