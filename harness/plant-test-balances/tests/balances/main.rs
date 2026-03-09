// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Tests.

use plant_balances::{
	AccountData, Config, CreditOf, Error, Pallet, TotalIssuance, DEFAULT_ADDRESS_URI,
};
use codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use std::collections::BTreeSet;
use subsoil::core::{hexdisplay::HexDisplay, sr25519::Pair as SrPair, Pair};
use subsoil::runtime::{
	traits::{BadOrigin, Zero},
	ArithmeticError, BuildStorage, Debug, DispatchError, DispatchResult, FixedPointNumber,
	TokenError,
};
use topsoil_core::{
	assert_err, assert_noop, assert_ok, assert_storage_noop, derive_impl,
	dispatch::{DispatchInfo, GetDispatchInfo},
	parameter_types,
	traits::{
		fungible, ConstU32, ConstU8, Imbalance as ImbalanceT, OnUnbalanced, StorageMapShim,
		StoredMap, VariantCount, VariantCountOf, WhitelistedStorageKeys,
	},
	weights::{IdentityFee, Weight},
};
use topsoil_core::system::{self as system, RawOrigin};
use plant_transaction_payment::{ChargeTransactionPayment, FungibleAdapter, Multiplier};

mod consumer_limit_tests;
mod currency_tests;
mod dispatchable_tests;
mod fungible_and_currency;
mod fungible_conformance_tests;
mod fungible_tests;
mod general_tests;
mod reentrancy_tests;

type Block = topsoil_core::system::mocking::MockBlock<Test>;

#[derive(
	Encode,
	Decode,
	DecodeWithMemTracking,
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	MaxEncodedLen,
	TypeInfo,
	Debug,
)]
pub enum TestId {
	Foo,
	Bar,
	Baz,
}

impl VariantCount for TestId {
	const VARIANT_COUNT: u32 = 3;
}

pub(crate) type AccountId = <Test as topsoil_core::system::Config>::AccountId;
pub(crate) type Balance = <Test as Config>::Balance;

topsoil_core::construct_runtime!(
	pub enum Test {
		System: topsoil_core::system,
		Balances: plant_balances,
		TransactionPayment: plant_transaction_payment,
	}
);

parameter_types! {
	pub BlockWeights: topsoil_core::system::limits::BlockWeights =
		topsoil_core::system::limits::BlockWeights::simple_max(
			topsoil_core::weights::Weight::from_parts(1024, u64::MAX),
		);
	pub static ExistentialDeposit: u64 = 1;
}

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type Block = Block;
	type AccountData = plant_balances::AccountData<u64>;
}

#[derive_impl(plant_transaction_payment::config_preludes::TestDefaultConfig)]
impl plant_transaction_payment::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = FungibleAdapter<Pallet<Test>, ()>;
	type OperationalFeeMultiplier = ConstU8<5>;
	type WeightToFee = IdentityFee<u64>;
	type LengthToFee = IdentityFee<u64>;
}

parameter_types! {
	pub FooReason: TestId = TestId::Foo;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl Config for Test {
	type DustRemoval = DustTrap;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = TestAccountStore;
	type MaxReserves = ConstU32<2>;
	type ReserveIdentifier = TestId;
	type RuntimeHoldReason = TestId;
	type RuntimeFreezeReason = TestId;
	type FreezeIdentifier = TestId;
	type MaxFreezes = VariantCountOf<TestId>;
}

#[derive(Clone)]
pub struct ExtBuilder {
	existential_deposit: u64,
	monied: bool,
	dust_trap: Option<u64>,
}
impl Default for ExtBuilder {
	fn default() -> Self {
		Self { existential_deposit: 1, monied: false, dust_trap: None }
	}
}
impl ExtBuilder {
	pub fn existential_deposit(mut self, existential_deposit: u64) -> Self {
		self.existential_deposit = existential_deposit;
		self
	}
	pub fn monied(mut self, monied: bool) -> Self {
		self.monied = monied;
		if self.existential_deposit == 0 {
			self.existential_deposit = 1;
		}
		self
	}
	pub fn dust_trap(mut self, account: u64) -> Self {
		self.dust_trap = Some(account);
		self
	}
	#[cfg(feature = "try-runtime")]
	pub fn auto_try_state(self, auto_try_state: bool) -> Self {
		AutoTryState::set(auto_try_state);
		self
	}
	pub fn set_associated_consts(&self) {
		DUST_TRAP_TARGET.with(|v| v.replace(self.dust_trap));
		EXISTENTIAL_DEPOSIT.with(|v| v.replace(self.existential_deposit));
	}
	pub fn build(self) -> subsoil::io::TestExternalities {
		self.set_associated_consts();
		let mut t = topsoil_core::system::GenesisConfig::<Test>::default().build_storage().unwrap();
		plant_balances::GenesisConfig::<Test> {
			balances: if self.monied {
				vec![
					(1, 10 * self.existential_deposit),
					(2, 20 * self.existential_deposit),
					(3, 30 * self.existential_deposit),
					(4, 40 * self.existential_deposit),
					(12, 10 * self.existential_deposit),
				]
			} else {
				vec![]
			},
			dev_accounts: Some((
				1000,
				self.existential_deposit,
				Some(DEFAULT_ADDRESS_URI.to_string()),
			)),
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let mut ext = subsoil::io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
	pub fn build_and_execute_with(self, f: impl Fn()) {
		let other = self.clone();
		UseSystem::set(false);
		other.build().execute_with(|| {
			f();
			#[cfg(feature = "try-runtime")]
			if AutoTryState::get() {
				Balances::do_try_state(System::block_number()).unwrap();
			}
		});
		UseSystem::set(true);
		self.build().execute_with(|| {
			f();
			#[cfg(feature = "try-runtime")]
			if AutoTryState::get() {
				Balances::do_try_state(System::block_number()).unwrap();
			}
		});
	}
}

parameter_types! {
	static DustTrapTarget: Option<u64> = None;
}

pub struct DustTrap;

impl OnUnbalanced<CreditOf<Test, ()>> for DustTrap {
	fn on_nonzero_unbalanced(amount: CreditOf<Test, ()>) {
		match DustTrapTarget::get() {
			None => drop(amount),
			Some(a) => {
				let result = <Balances as fungible::Balanced<_>>::resolve(&a, amount);
				debug_assert!(result.is_ok());
			},
		}
	}
}

parameter_types! {
	pub static UseSystem: bool = false;
	pub static AutoTryState: bool = true;
}

type BalancesAccountStore = StorageMapShim<plant_balances::Account<Test>, u64, plant_balances::AccountData<u64>>;
type SystemAccountStore = topsoil_core::system::Pallet<Test>;

pub struct TestAccountStore;
impl StoredMap<u64, plant_balances::AccountData<u64>> for TestAccountStore {
	fn get(k: &u64) -> plant_balances::AccountData<u64> {
		if UseSystem::get() {
			<SystemAccountStore as StoredMap<_, _>>::get(k)
		} else {
			<BalancesAccountStore as StoredMap<_, _>>::get(k)
		}
	}
	fn try_mutate_exists<R, E: From<DispatchError>>(
		k: &u64,
		f: impl FnOnce(&mut Option<plant_balances::AccountData<u64>>) -> Result<R, E>,
	) -> Result<R, E> {
		if UseSystem::get() {
			<SystemAccountStore as StoredMap<_, _>>::try_mutate_exists(k, f)
		} else {
			<BalancesAccountStore as StoredMap<_, _>>::try_mutate_exists(k, f)
		}
	}
	fn mutate<R>(
		k: &u64,
		f: impl FnOnce(&mut plant_balances::AccountData<u64>) -> R,
	) -> Result<R, DispatchError> {
		if UseSystem::get() {
			<SystemAccountStore as StoredMap<_, _>>::mutate(k, f)
		} else {
			<BalancesAccountStore as StoredMap<_, _>>::mutate(k, f)
		}
	}
	fn mutate_exists<R>(
		k: &u64,
		f: impl FnOnce(&mut Option<plant_balances::AccountData<u64>>) -> R,
	) -> Result<R, DispatchError> {
		if UseSystem::get() {
			<SystemAccountStore as StoredMap<_, _>>::mutate_exists(k, f)
		} else {
			<BalancesAccountStore as StoredMap<_, _>>::mutate_exists(k, f)
		}
	}
	fn insert(k: &u64, t: plant_balances::AccountData<u64>) -> Result<(), DispatchError> {
		if UseSystem::get() {
			<SystemAccountStore as StoredMap<_, _>>::insert(k, t)
		} else {
			<BalancesAccountStore as StoredMap<_, _>>::insert(k, t)
		}
	}
	fn remove(k: &u64) -> Result<(), DispatchError> {
		if UseSystem::get() {
			<SystemAccountStore as StoredMap<_, _>>::remove(k)
		} else {
			<BalancesAccountStore as StoredMap<_, _>>::remove(k)
		}
	}
}

pub fn events() -> Vec<RuntimeEvent> {
	let evt = System::events().into_iter().map(|evt| evt.event).collect::<Vec<_>>();
	System::reset_events();
	evt
}

/// create a transaction info struct from weight. Handy to avoid building the whole struct.
pub fn info_from_weight(w: Weight) -> DispatchInfo {
	DispatchInfo { call_weight: w, ..Default::default() }
}

/// Check that the total-issuance matches the sum of all accounts' total balances.
pub fn ensure_ti_valid() {
	let mut sum = 0;

	// Fetch the dev accounts from Account Storage.
	let dev_accounts = (1000u64, EXISTENTIAL_DEPOSIT, DEFAULT_ADDRESS_URI.to_string());
	let (num_accounts, _balance, ref derivation) = dev_accounts;

	// Generate the dev account public keys.
	let dev_account_ids: Vec<_> = (0..num_accounts)
		.map(|index| {
			let derivation_string = derivation.replace("{}", &index.to_string());
			let pair: SrPair =
				Pair::from_string(&derivation_string, None).expect("Invalid derivation string");
			<Test as topsoil_core::system::Config>::AccountId::decode(
				&mut &pair.public().encode()[..],
			)
			.unwrap()
		})
		.collect();

	// Iterate over all account keys (i.e., the account IDs).
	for acc in topsoil_core::system::Account::<Test>::iter_keys() {
		// Skip dev accounts by checking if the account is in the dev_account_ids list.
		// This also proves dev_accounts exists in storage.
		if dev_account_ids.contains(&acc) {
			continue;
		}

		// Check if we are using the system pallet or some other custom storage for accounts.
		if UseSystem::get() {
			let data = topsoil_core::system::Pallet::<Test>::account(acc);
			sum += data.data.total();
		} else {
			let data = plant_balances::Account::<Test>::get(acc);
			sum += data.total();
		}
	}

	// Ensure the total issuance matches the sum of the account balances
	assert_eq!(TotalIssuance::<Test>::get(), sum, "Total Issuance is incorrect");
}

#[test]
fn weights_sane() {
	let info = plant_balances::Call::<Test>::transfer_allow_death { dest: 10, value: 4 }.get_dispatch_info();
	assert_eq!(<() as plant_balances::WeightInfo>::transfer_allow_death(), info.call_weight);

	let info = plant_balances::Call::<Test>::force_unreserve { who: 10, amount: 4 }.get_dispatch_info();
	assert_eq!(<() as plant_balances::WeightInfo>::force_unreserve(), info.call_weight);
}

#[test]
fn check_whitelist() {
	let whitelist: BTreeSet<String> = AllPalletsWithSystem::whitelisted_storage_keys()
		.iter()
		.map(|s| HexDisplay::from(&s.key).to_string())
		.collect();
	// Inactive Issuance
	assert!(whitelist.contains("c2261276cc9d1f8598ea4b6a74b15c2f1ccde6872881f893a21de93dfe970cd5"));
	// Total Issuance
	assert!(whitelist.contains("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80"));
}

/// This pallet runs tests twice, once with system as `type AccountStore` and once this pallet. This
/// function will return the right value based on the `UseSystem` flag.
pub(crate) fn get_test_account_data(who: AccountId) -> AccountData<Balance> {
	if UseSystem::get() {
		<SystemAccountStore as StoredMap<_, _>>::get(&who)
	} else {
		<BalancesAccountStore as StoredMap<_, _>>::get(&who)
	}
}

/// Same as `get_test_account_data`, but returns a `topsoil_core::system::AccountInfo` with the data filled
/// in.
pub(crate) fn get_test_account(
	who: AccountId,
) -> topsoil_core::system::AccountInfo<u32, AccountData<Balance>> {
	let mut system_account = topsoil_core::system::Account::<Test>::get(&who);
	let account_data = get_test_account_data(who);
	system_account.data = account_data;
	system_account
}
