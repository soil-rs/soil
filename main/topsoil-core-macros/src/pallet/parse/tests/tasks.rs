// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use syn::parse_quote;

#[test]
fn test_parse_pallet_with_task_enum_missing_impl() {
	assert_pallet_parse_error! {
		#[manifest_dir("../../contrib/plant-examples/basic")]
		#[error_regex("Missing `\\#\\[pallet::tasks_experimental\\]` impl")]
		#[topsoil_core::pallet]
		pub mod pallet {
			#[pallet::task_enum]
			pub enum Task<T: Config> {
				Something,
			}

			#[pallet::config]
			pub trait Config: topsoil_core::system::Config {}

			#[pallet::pallet]
			pub struct Pallet<T>(_);
		}
	}
}

#[test]
fn test_parse_pallet_with_task_enum_wrong_attribute() {
	assert_pallet_parse_error! {
		#[manifest_dir("../../contrib/plant-examples/basic")]
		#[error_regex("expected one of")]
		#[topsoil_core::pallet]
		pub mod pallet {
			#[pallet::wrong_attribute]
			pub enum Task<T: Config> {
				Something,
			}

			#[pallet::task_list]
			impl<T: Config> topsoil_core::traits::Task for Task<T>
			where
				T: TypeInfo,
			{}

			#[pallet::config]
			pub trait Config: topsoil_core::system::Config {}

			#[pallet::pallet]
			pub struct Pallet<T>(_);
		}
	}
}

#[test]
fn test_parse_pallet_missing_task_enum() {
	assert_pallet_parses! {
		#[manifest_dir("../../contrib/plant-examples/basic")]
		#[topsoil_core::pallet]
		pub mod pallet {
			#[pallet::tasks_experimental]
			#[cfg(test)] // aha, this means it's being eaten
			impl<T: Config> topsoil_core::traits::Task for Task<T>
			where
				T: TypeInfo,
			{}

			#[pallet::config]
			pub trait Config: topsoil_core::system::Config {}

			#[pallet::pallet]
			pub struct Pallet<T>(_);
		}
	};
}

#[test]
fn test_parse_pallet_task_list_in_wrong_place() {
	assert_pallet_parse_error! {
		#[manifest_dir("../../contrib/plant-examples/basic")]
		#[error_regex("can only be used on items within an `impl` statement.")]
		#[topsoil_core::pallet]
		pub mod pallet {
			pub enum MyCustomTaskEnum<T: Config> {
				Something,
			}

			#[pallet::task_list]
			pub fn something() {
				println!("hey");
			}

			#[pallet::config]
			pub trait Config: topsoil_core::system::Config {}

			#[pallet::pallet]
			pub struct Pallet<T>(_);
		}
	}
}

#[test]
fn test_parse_pallet_manual_tasks_impl_without_manual_tasks_enum() {
	assert_pallet_parse_error! {
		#[manifest_dir("../../contrib/plant-examples/basic")]
		#[error_regex(".*attribute must be attached to your.*")]
		#[topsoil_core::pallet]
		pub mod pallet {

			impl<T: Config> topsoil_core::traits::Task for Task<T>
			where
				T: TypeInfo,
			{
				type Enumeration = alloc::vec::IntoIter<Task<T>>;

				fn iter() -> Self::Enumeration {
					alloc::vec![Task::increment, Task::decrement].into_iter()
				}
			}

			#[pallet::config]
			pub trait Config: topsoil_core::system::Config {}

			#[pallet::pallet]
			pub struct Pallet<T>(_);
		}
	}
}

#[test]
fn test_parse_pallet_manual_task_enum_non_manual_impl() {
	assert_pallet_parses! {
		#[manifest_dir("../../contrib/plant-examples/basic")]
		#[topsoil_core::pallet]
		pub mod pallet {
			pub enum MyCustomTaskEnum<T: Config> {
				Something,
			}

			#[pallet::tasks_experimental]
			impl<T: Config> topsoil_core::traits::Task for MyCustomTaskEnum<T>
			where
				T: TypeInfo,
			{}

			#[pallet::config]
			pub trait Config: topsoil_core::system::Config {}

			#[pallet::pallet]
			pub struct Pallet<T>(_);
		}
	};
}

#[test]
fn test_parse_pallet_non_manual_task_enum_manual_impl() {
	assert_pallet_parses! {
		#[manifest_dir("../../contrib/plant-examples/basic")]
		#[topsoil_core::pallet]
		pub mod pallet {
			#[pallet::task_enum]
			pub enum MyCustomTaskEnum<T: Config> {
				Something,
			}

			impl<T: Config> topsoil_core::traits::Task for MyCustomTaskEnum<T>
			where
				T: TypeInfo,
			{}

			#[pallet::config]
			pub trait Config: topsoil_core::system::Config {}

			#[pallet::pallet]
			pub struct Pallet<T>(_);
		}
	};
}

#[test]
fn test_parse_pallet_manual_task_enum_manual_impl() {
	assert_pallet_parses! {
		#[manifest_dir("../../contrib/plant-examples/basic")]
		#[topsoil_core::pallet]
		pub mod pallet {
			pub enum MyCustomTaskEnum<T: Config> {
				Something,
			}

			impl<T: Config> topsoil_core::traits::Task for MyCustomTaskEnum<T>
			where
				T: TypeInfo,
			{}

			#[pallet::config]
			pub trait Config: topsoil_core::system::Config {}

			#[pallet::pallet]
			pub struct Pallet<T>(_);
		}
	};
}

#[test]
fn test_parse_pallet_manual_task_enum_mismatch_ident() {
	assert_pallet_parses! {
		#[manifest_dir("../../contrib/plant-examples/basic")]
		#[topsoil_core::pallet]
		pub mod pallet {
			pub enum WrongIdent<T: Config> {
				Something,
			}

			#[pallet::tasks_experimental]
			impl<T: Config> topsoil_core::traits::Task for MyCustomTaskEnum<T>
			where
				T: TypeInfo,
			{}

			#[pallet::config]
			pub trait Config: topsoil_core::system::Config {}

			#[pallet::pallet]
			pub struct Pallet<T>(_);
		}
	};
}
