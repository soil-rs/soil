// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! A logger that can be used to log from the runtime.
//!
//! See [`RuntimeLogger`] for more docs.

/// Runtime logger implementation - `log` crate backend.
///
/// The logger should be initialized if you want to display
/// logs inside the runtime that is not necessarily running natively.
pub struct RuntimeLogger;

impl RuntimeLogger {
	/// Initialize the logger.
	///
	/// This is a no-op when running natively (`std`).
	#[cfg(feature = "std")]
	pub fn init() {}

	/// Initialize the logger.
	///
	/// This is a no-op when running natively (`std`).
	#[cfg(not(feature = "std"))]
	pub fn init() {
		static LOGGER: RuntimeLogger = RuntimeLogger;
		let _ = log::set_logger(&LOGGER);

		// Use the same max log level as used by the host.
		log::set_max_level(crate::io::logging::max_level().into());
	}
}

impl log::Log for RuntimeLogger {
	fn enabled(&self, _: &log::Metadata) -> bool {
		// The final filtering is done by the host. This is not perfect, as we would still call into
		// the host for log lines that will be thrown away.
		true
	}

	fn log(&self, record: &log::Record) {
		use ::core::fmt::Write;
		let mut msg = alloc::string::String::default();
		let _ = ::core::write!(&mut msg, "{}", record.args());

		crate::io::logging::log(record.level().into(), record.target(), msg.as_bytes());
	}

	fn flush(&self) {}
}

// NOTE: runtime_logger integration test moved out of subsoil to avoid circular dev-dependency:
// subsoil (dev-dep) -> soil-test-node-runtime-client -> soil-test-node-runtime -> subsoil (WASM)
// This creates two compilations of subsoil with different features, causing trait mismatches.
