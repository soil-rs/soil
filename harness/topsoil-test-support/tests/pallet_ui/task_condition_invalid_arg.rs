// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_core::pallet(dev_mode)]
mod pallet {
	use topsoil_core::pallet_prelude::DispatchResult;
	use topsoil_core::system::pallet_prelude::OriginFor;

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

    #[pallet::tasks_experimental]
	impl<T: Config> Pallet<T> {
		#[pallet::task_index(0)]
		#[pallet::task_condition(|flag: bool| flag)]
		#[pallet::task_list(vec![1, 2].iter())]
		#[pallet::task_weight(0.into())]
		fn foo(_i: u32) -> DispatchResult {
			Ok(())
		}
	}
}

fn main() {
}
