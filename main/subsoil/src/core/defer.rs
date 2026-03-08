// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Contains the [`crate::defer!`] macro for *deferring* the execution
//! of code until the current scope is dropped.
//! This helps with *always* executing cleanup code.

/// Executes the wrapped closure on drop.
///
/// Should be used together with the [`crate::defer!`] macro.
#[must_use]
pub struct DeferGuard<F: FnOnce()>(pub Option<F>);

impl<F: FnOnce()> DeferGuard<F> {
	/// Creates a new `DeferGuard` with the given closure.
	pub fn new(f: F) -> Self {
		Self(Some(f))
	}
}

impl<F: FnOnce()> Drop for DeferGuard<F> {
	fn drop(&mut self) {
		self.0.take().map(|f| f());
	}
}

/// Executes the given code when the current scope is dropped.
///
/// Multiple calls to [`crate::defer!`] will execute the passed codes in reverse order.
/// This also applies to panic stack unwinding.
///
/// # Example
///
/// ```rust
/// use subsoil::defer;
///
/// let message = std::cell::RefCell::new("".to_string());
/// {
/// 	defer!(
/// 		message.borrow_mut().push_str("world!");
/// 	);
/// 	defer!(
/// 		message.borrow_mut().push_str("Hello ");
/// 	);
/// }
/// assert_eq!(*message.borrow(), "Hello world!");
/// ```
#[macro_export]
macro_rules! defer(
	( $( $code:tt )* ) => {
		let _guard = $crate::core::defer::DeferGuard(Some(|| { $( $code )* }));
	};
);

#[cfg(test)]
mod test {
	#[test]
	fn defer_guard_works() {
		let mut called = false;
		{
			defer!(
				called = true;
			);
		}
		assert!(called, "DeferGuard should have executed the closure");
	}

	#[test]
	/// `defer` executes the code in reverse order of being called.
	fn defer_guard_order_works() {
		let called = std::cell::RefCell::new(1);

		defer!(
			assert_eq!(*called.borrow(), 3);
		);
		defer!(
			assert_eq!(*called.borrow(), 2);
			*called.borrow_mut() = 3;
		);
		defer!({
			assert_eq!(*called.borrow(), 1);
			*called.borrow_mut() = 2;
		});
	}

	#[test]
	#[allow(unused_braces)]
	#[allow(clippy::unnecessary_operation)]
	fn defer_guard_syntax_works() {
		let called = std::cell::RefCell::new(0);
		{
			defer!(*called.borrow_mut() += 1);
			defer!(*called.borrow_mut() += 1;); // With ;
			defer!({ *called.borrow_mut() += 1 });
			defer!({ *called.borrow_mut() += 1 };); // With ;
		}
		assert_eq!(*called.borrow(), 4);
	}

	#[test]
	/// `defer` executes the code even in case of a panic.
	fn defer_guard_panic_unwind_works() {
		use std::panic::{catch_unwind, AssertUnwindSafe};
		let mut called = false;

		let should_panic = catch_unwind(AssertUnwindSafe(|| {
			defer!(called = true);
			panic!();
		}));

		assert!(should_panic.is_err(), "DeferGuard should have panicked");
		assert!(called, "DeferGuard should have executed the closure");
	}

	#[test]
	/// `defer` executes the code even in case another `defer` panics.
	fn defer_guard_defer_panics_unwind_works() {
		use std::panic::{catch_unwind, AssertUnwindSafe};
		let counter = std::cell::RefCell::new(0);

		let should_panic = catch_unwind(AssertUnwindSafe(|| {
			defer!(*counter.borrow_mut() += 1);
			defer!(
				*counter.borrow_mut() += 1;
				panic!();
			);
			defer!(*counter.borrow_mut() += 1);
		}));

		assert!(should_panic.is_err(), "DeferGuard should have panicked");
		assert_eq!(*counter.borrow(), 3, "DeferGuard should have executed the closure");
	}
}
