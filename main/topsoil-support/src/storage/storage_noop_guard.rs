// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg(any(feature = "std", feature = "runtime-benchmarks", feature = "try-runtime", test))]

//! Contains the [`crate::StorageNoopGuard`] for conveniently asserting
//! that no storage mutation has been made by a whole code block.

/// Asserts that no storage changes took place between con- and destruction of [`Self`].
///
/// This is easier than wrapping the whole code-block inside a `assert_storage_noop!`.
///
/// # Example
///
/// ```should_panic
/// use topsoil_support::{StorageNoopGuard, storage::unhashed::put};
///
/// subsoil::io::TestExternalities::default().execute_with(|| {
/// 	let _guard = topsoil_support::StorageNoopGuard::default();
/// 	put(b"key", b"value");
/// 	// Panics since there are storage changes.
/// });
/// ```
#[must_use]
pub struct StorageNoopGuard<'a> {
	storage_root: alloc::vec::Vec<u8>,
	error_message: &'a str,
}

impl<'a> Default for StorageNoopGuard<'a> {
	fn default() -> Self {
		Self {
			storage_root: subsoil::io::storage::root(subsoil::runtime::StateVersion::V1),
			error_message: "`StorageNoopGuard` detected an attempted storage change.",
		}
	}
}

impl<'a> StorageNoopGuard<'a> {
	/// Alias to `default()`.
	pub fn new() -> Self {
		Self::default()
	}

	/// Creates a new [`StorageNoopGuard`] with a custom error message.
	pub fn from_error_message(error_message: &'a str) -> Self {
		Self {
			storage_root: subsoil::io::storage::root(subsoil::runtime::StateVersion::V1),
			error_message,
		}
	}

	/// Sets a custom error message for a [`StorageNoopGuard`].
	pub fn set_error_message(&mut self, error_message: &'a str) {
		self.error_message = error_message;
	}
}

impl<'a> Drop for StorageNoopGuard<'a> {
	fn drop(&mut self) {
		// No need to double panic, eg. inside a test assertion failure.
		#[cfg(feature = "std")]
		if std::thread::panicking() {
			return;
		}
		assert_eq!(
			subsoil::io::storage::root(subsoil::runtime::StateVersion::V1),
			self.storage_root,
			"{}",
			self.error_message,
		);
	}
}

#[cfg(test)]
mod tests {
	use subsoil::io::TestExternalities;

	use super::*;

	#[test]
	#[should_panic(expected = "`StorageNoopGuard` detected an attempted storage change.")]
	fn storage_noop_guard_panics_on_changed() {
		TestExternalities::default().execute_with(|| {
			let _guard = StorageNoopGuard::default();
			topsoil_support::storage::unhashed::put(b"key", b"value");
		});
	}

	#[test]
	fn storage_noop_guard_works_on_unchanged() {
		TestExternalities::default().execute_with(|| {
			let _guard = StorageNoopGuard::default();
			topsoil_support::storage::unhashed::put(b"key", b"value");
			topsoil_support::storage::unhashed::kill(b"key");
		});
	}

	#[test]
	#[should_panic(expected = "`StorageNoopGuard` detected an attempted storage change.")]
	fn storage_noop_guard_panics_on_early_drop() {
		TestExternalities::default().execute_with(|| {
			let guard = StorageNoopGuard::default();
			topsoil_support::storage::unhashed::put(b"key", b"value");
			std::mem::drop(guard);
			topsoil_support::storage::unhashed::kill(b"key");
		});
	}

	#[test]
	fn storage_noop_guard_works_on_changed_forget() {
		TestExternalities::default().execute_with(|| {
			let guard = StorageNoopGuard::default();
			topsoil_support::storage::unhashed::put(b"key", b"value");
			std::mem::forget(guard);
		});
	}

	#[test]
	#[should_panic(expected = "Something else")]
	fn storage_noop_guard_does_not_double_panic() {
		TestExternalities::default().execute_with(|| {
			let _guard = StorageNoopGuard::default();
			topsoil_support::storage::unhashed::put(b"key", b"value");
			panic!("Something else");
		});
	}

	#[test]
	#[should_panic(expected = "`StorageNoopGuard` found unexpected storage changes.")]
	fn storage_noop_guard_panics_created_from_error_message() {
		TestExternalities::default().execute_with(|| {
			let _guard = StorageNoopGuard::from_error_message(
				"`StorageNoopGuard` found unexpected storage changes.",
			);
			topsoil_support::storage::unhashed::put(b"key", b"value");
		});
	}

	#[test]
	#[should_panic(expected = "`StorageNoopGuard` found unexpected storage changes.")]
	fn storage_noop_guard_panics_with_set_error_message() {
		TestExternalities::default().execute_with(|| {
			let mut guard = StorageNoopGuard::default();
			guard.set_error_message("`StorageNoopGuard` found unexpected storage changes.");
			topsoil_support::storage::unhashed::put(b"key", b"value");
		});
	}

	#[test]
	#[should_panic(expected = "`StorageNoopGuard` detected an attempted storage change.")]
	fn storage_noop_guard_panics_new_alias() {
		TestExternalities::default().execute_with(|| {
			let _guard = StorageNoopGuard::new();
			topsoil_support::storage::unhashed::put(b"key", b"value");
		});
	}
}
