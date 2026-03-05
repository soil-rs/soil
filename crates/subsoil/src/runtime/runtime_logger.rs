// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
// subsoil (dev-dep) -> substrate-test-runtime-client -> substrate-test-runtime -> subsoil (WASM)
// This creates two compilations of subsoil with different features, causing trait mismatches.
