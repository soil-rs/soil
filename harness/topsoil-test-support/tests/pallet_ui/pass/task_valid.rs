// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_support::pallet(dev_mode)]
pub mod pallet {
	use topsoil_support::{ensure, pallet_prelude::DispatchResult};

	#[pallet::config]
	pub trait Config: topsoil_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

    #[pallet::tasks_experimental]
	impl<T: Config> Pallet<T> {
		#[pallet::task_index(0)]
		#[pallet::task_condition(|i, j| i == 0u32 && j == 2u64)]
		#[pallet::task_list(vec![(0u32, 2u64), (2u32, 4u64)].iter())]
		#[pallet::task_weight(0.into())]
		fn foo(i: u32, j: u64) -> DispatchResult {
			ensure!(i == 0, "i must be 0");
			ensure!(j == 2, "j must be 2");
			Ok(())
		}
	}
}

#[topsoil_support::pallet(dev_mode)]
pub mod pallet_with_instance {
	use topsoil_support::pallet_prelude::{ValueQuery, StorageValue};

	#[pallet::config]
	pub trait Config<I: 'static = ()>: topsoil_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(_);

	#[pallet::storage]
	pub type SomeStorage<T, I = ()> = StorageValue<_, u32, ValueQuery>;

    #[pallet::tasks_experimental]
	impl<T: Config<I>, I> Pallet<T, I> {
		#[pallet::task_index(0)]
		#[pallet::task_condition(|i, j| i == 0u32 && j == 2u64)]
		#[pallet::task_list(vec![(0u32, 2u64), (2u32, 4u64)].iter())]
		#[pallet::task_weight(0.into())]
		fn foo(_i: u32, _j: u64) -> topsoil_support::pallet_prelude::DispatchResult {
			<SomeStorage<T, I>>::get();
			Ok(())
		}
	}
}

fn main() {
}
