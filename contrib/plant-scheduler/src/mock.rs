// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! # Scheduler test environment.

use super::*;

use crate as scheduler;
use subsoil::runtime::{BuildStorage, Perbill};
use subsoil::weights::constants::WEIGHT_REF_TIME_PER_SECOND;
use topsoil_core::{
	derive_impl, ord_parameter_types, parameter_types,
	traits::{ConstU32, Contains, EitherOfDiverse, EqualPrivilegeOnly},
};
use topsoil_core::system::{EnsureRoot, EnsureSignedBy};

// Logger module to track execution.
#[topsoil_core::pallet]
pub mod logger {
	use super::{OriginCaller, OriginTrait};
	use topsoil_core::{pallet_prelude::*, parameter_types};
	use topsoil_core::system::pallet_prelude::*;

	parameter_types! {
		static Log: Vec<(OriginCaller, u32)> = Vec::new();
	}
	pub fn log() -> Vec<(OriginCaller, u32)> {
		Log::get().clone()
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type Threshold<T: Config> = StorageValue<_, (BlockNumberFor<T>, BlockNumberFor<T>)>;

	#[pallet::error]
	pub enum Error<T> {
		/// Under the threshold.
		TooEarly,
		/// Over the threshold.
		TooLate,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {
		#[allow(deprecated)]
		type RuntimeEvent: From<Event<Self>>
			+ IsType<<Self as topsoil_core::system::Config>::RuntimeEvent>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Logged(u32, Weight),
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		<T as topsoil_core::system::Config>::RuntimeOrigin: OriginTrait<PalletsOrigin = OriginCaller>,
	{
		#[pallet::call_index(0)]
		#[pallet::weight(*weight)]
		pub fn log(origin: OriginFor<T>, i: u32, weight: Weight) -> DispatchResult {
			Self::deposit_event(Event::Logged(i, weight));
			Log::mutate(|log| {
				log.push((origin.caller().clone(), i));
			});
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(*weight)]
		pub fn log_without_filter(origin: OriginFor<T>, i: u32, weight: Weight) -> DispatchResult {
			Self::deposit_event(Event::Logged(i, weight));
			Log::mutate(|log| {
				log.push((origin.caller().clone(), i));
			});
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(*weight)]
		pub fn timed_log(origin: OriginFor<T>, i: u32, weight: Weight) -> DispatchResult {
			let now = topsoil_core::system::Pallet::<T>::block_number();
			let (start, end) = Threshold::<T>::get().unwrap_or((0u32.into(), u32::MAX.into()));
			ensure!(now >= start, Error::<T>::TooEarly);
			ensure!(now <= end, Error::<T>::TooLate);
			Self::deposit_event(Event::Logged(i, weight));
			Log::mutate(|log| {
				log.push((origin.caller().clone(), i));
			});
			Ok(())
		}
	}
}

type Block = topsoil_core::system::mocking::MockBlock<Test>;

topsoil_core::construct_runtime!(
	pub enum Test
	{
		System: topsoil_core::system,
		Logger: logger,
		Scheduler: scheduler,
		Preimage: plant_preimage,
	}
);

// Scheduler must dispatch with root and no filter, this tests base filter is indeed not used.
pub struct BaseFilter;
impl Contains<RuntimeCall> for BaseFilter {
	fn contains(call: &RuntimeCall) -> bool {
		!matches!(call, RuntimeCall::Logger(LoggerCall::log { .. }))
	}
}

parameter_types! {
	pub BlockWeights: topsoil_core::system::limits::BlockWeights =
		topsoil_core::system::limits::BlockWeights::simple_max(
			Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND * 2, u64::MAX),
		);
}

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl system::Config for Test {
	type BaseCallFilter = BaseFilter;
	type Block = Block;
	type BlockWeights = BlockWeights;
}
impl logger::Config for Test {
	type RuntimeEvent = RuntimeEvent;
}
ord_parameter_types! {
	pub const One: u64 = 1;
}

impl plant_preimage::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Currency = ();
	type ManagerOrigin = EnsureRoot<u64>;
	type Consideration = ();
}

pub struct TestWeightInfo;
impl WeightInfo for TestWeightInfo {
	fn service_agendas_base() -> Weight {
		Weight::from_parts(0b0000_0001, 0)
	}
	fn service_agenda_base(i: u32) -> Weight {
		Weight::from_parts((i << 8) as u64 + 0b0000_0010, 0)
	}
	fn service_task_base() -> Weight {
		Weight::from_parts(0b0000_0100, 0)
	}
	fn service_task_periodic() -> Weight {
		Weight::from_parts(0b0000_1100, 0)
	}
	fn service_task_named() -> Weight {
		Weight::from_parts(0b0001_0100, 0)
	}
	fn service_task_fetched(s: u32) -> Weight {
		Weight::from_parts((s << 8) as u64 + 0b0010_0100, 0)
	}
	fn execute_dispatch_signed() -> Weight {
		Weight::from_parts(0b0100_0000, 0)
	}
	fn execute_dispatch_unsigned() -> Weight {
		Weight::from_parts(0b1000_0000, 0)
	}
	fn schedule(_s: u32) -> Weight {
		Weight::from_parts(50, 0)
	}
	fn cancel(_s: u32) -> Weight {
		Weight::from_parts(50, 0)
	}
	fn schedule_named(_s: u32) -> Weight {
		Weight::from_parts(50, 0)
	}
	fn cancel_named(_s: u32) -> Weight {
		Weight::from_parts(50, 0)
	}
	fn schedule_retry(_s: u32) -> Weight {
		Weight::from_parts(100000, 0)
	}
	fn set_retry() -> Weight {
		Weight::from_parts(50, 0)
	}
	fn set_retry_named() -> Weight {
		Weight::from_parts(50, 0)
	}
	fn cancel_retry() -> Weight {
		Weight::from_parts(50, 0)
	}
	fn cancel_retry_named() -> Weight {
		Weight::from_parts(50, 0)
	}
}
parameter_types! {
	pub storage MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
		BlockWeights::get().max_block;
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = EitherOfDiverse<EnsureRoot<u64>, EnsureSignedBy<One, u64>>;
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type MaxScheduledPerBlock = ConstU32<10>;
	type WeightInfo = TestWeightInfo;
	type Preimages = Preimage;
	type BlockNumberProvider = topsoil_core::system::Pallet<Self>;
}

pub type LoggerCall = logger::Call<Test>;

pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let t = system::GenesisConfig::<Test>::default().build_storage().unwrap();
	t.into()
}

pub fn root() -> OriginCaller {
	system::RawOrigin::Root.into()
}
