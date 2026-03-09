// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use subsoil::runtime::{BuildStorage, Perbill};
use topsoil_core::{derive_impl, dispatch::DispatchClass};
#[topsoil_core::pallet]
mod module {
	use topsoil_core::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {
		#[allow(deprecated)]
		type RuntimeEvent: From<Event> + IsType<<Self as topsoil_core::system::Config>::RuntimeEvent>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event {
		Complex(Vec<u8>, u32, u16, u128),
	}
}

type Block = topsoil_core::system::mocking::MockBlock<Runtime>;

topsoil_core::construct_runtime!(
	pub enum Runtime {
		System: topsoil_core::system,
		Module: module,
	}
);

const MAX_BLOCK_LENGTH: u32 = 4 * 1024 * 1024;
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

topsoil_core::parameter_types! {
	pub BlockLength: topsoil_core::system::limits::BlockLength =
		topsoil_core::system::limits::BlockLength::builder()
			.max_length(MAX_BLOCK_LENGTH)
			.modify_max_length_for_class(DispatchClass::Normal, |m| {
				*m = NORMAL_DISPATCH_RATIO * MAX_BLOCK_LENGTH
			})
			.build();
}

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Runtime {
	type Nonce = u64;
	type Block = Block;
}

impl module::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

fn new_test_ext() -> subsoil::io::TestExternalities {
	topsoil_core::system::GenesisConfig::<Runtime>::default()
		.build_storage()
		.unwrap()
		.into()
}

fn deposit_events(n: usize) {
	let mut t = new_test_ext();
	t.execute_with(|| {
		for _ in 0..n {
			module::Pallet::<Runtime>::deposit_event(module::Event::Complex(
				vec![1, 2, 3],
				2,
				3,
				899,
			));
		}
	});
}

fn sr_system_benchmark(c: &mut Criterion) {
	c.bench_function("deposit 100 events", |b| b.iter(|| deposit_events(black_box(100))));
}

criterion_group!(benches, sr_system_benchmark);
criterion_main!(benches);
