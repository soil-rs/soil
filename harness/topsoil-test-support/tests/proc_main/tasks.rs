// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[cfg(test)]
use super::{
	assert_ok,
	topsoil_core::system::{Numbers, Total},
	Runtime, RuntimeOrigin, RuntimeTask, System,
};
use topsoil_core::pallet_macros::pallet_section;

#[pallet_section]
mod tasks_example {
	#[docify::export(tasks_example)]
	#[pallet::tasks_experimental]
	impl<T: Config> Pallet<T> {
		/// Add a pair of numbers into the totals and remove them.
		#[pallet::task_list(Numbers::<T>::iter_keys())]
		#[pallet::task_condition(|i| Numbers::<T>::contains_key(i))]
		#[pallet::task_weight(0.into())]
		#[pallet::task_index(0)]
		pub fn add_number_into_total(i: u32) -> DispatchResult {
			let v = Numbers::<T>::take(i).ok_or(Error::<T>::NotFound)?;
			Total::<T>::mutate(|(total_keys, total_values)| {
				*total_keys += i;
				*total_values += v;
			});
			Ok(())
		}
	}
}

#[docify::export]
#[test]
fn tasks_work() {
	super::new_test_ext().execute_with(|| {
		Numbers::<Runtime>::insert(0, 1);

		let task =
			RuntimeTask::System(super::topsoil_core::system::Task::<Runtime>::AddNumberIntoTotal {
				i: 0u32,
			});

		assert_ok!(System::do_task(RuntimeOrigin::signed(1), task.clone(),));
		assert_eq!(Numbers::<Runtime>::get(0), None);
		assert_eq!(Total::<Runtime>::get(), (0, 1));
	});
}
