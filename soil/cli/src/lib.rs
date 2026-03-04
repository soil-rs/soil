// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate CLI library.
//!
//! To see a full list of commands available, see [`commands`].

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_imports)]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use clap::{CommandFactory, FromArgMatches, Parser};
#[cfg(feature = "std")]
use log::warn;
#[cfg(feature = "std")]
use soil_service::Configuration;

#[cfg(feature = "std")]
pub mod arg_enums;
#[cfg(feature = "std")]
pub mod commands;
#[cfg(feature = "std")]
mod config;
#[cfg(feature = "std")]
mod error;
#[cfg(feature = "std")]
mod params;
#[cfg(feature = "std")]
mod runner;
#[cfg(feature = "std")]
mod signals;

#[cfg(feature = "std")]
pub use arg_enums::*;
#[cfg(feature = "std")]
pub use clap;
#[cfg(feature = "std")]
pub use commands::*;
#[cfg(feature = "std")]
pub use config::*;
#[cfg(feature = "std")]
pub use error::*;
#[cfg(feature = "std")]
pub use params::*;
#[cfg(feature = "std")]
pub use runner::*;
#[cfg(feature = "std")]
pub use soil_service::{ChainSpec, Role};
#[cfg(feature = "std")]
pub use sc_tracing::logging::LoggerBuilder;
#[cfg(feature = "std")]
pub use signals::Signals;
#[cfg(feature = "std")]
pub use soil_version::RuntimeVersion;

/// Substrate client CLI
///
/// This trait needs to be implemented on the root CLI struct of the application. It will provide
/// the implementation `name`, `version`, `executable name`, `description`, `author`, `support_url`,
/// `copyright start year` and most importantly: how to load the chain spec.
#[cfg(feature = "std")]
pub trait SubstrateCli: Sized {
	/// Implementation name.
#[cfg(feature = "std")]
	fn impl_name() -> String;

	/// Implementation version.
	///
	/// By default, it will look like this:
	///
	/// `2.0.0-b950f731c`
	///
	/// Where the hash is the short hash of the commit in the Git repository.
#[cfg(feature = "std")]
	fn impl_version() -> String;

	/// Executable file name.
	///
	/// Extracts the file name from `std::env::current_exe()`.
	/// Resorts to the env var `CARGO_PKG_NAME` in case of Error.
#[cfg(feature = "std")]
	fn executable_name() -> String {
		std::env::current_exe()
			.ok()
			.and_then(|e| e.file_name().map(|s| s.to_os_string()))
			.and_then(|w| w.into_string().ok())
			.unwrap_or_else(|| env!("CARGO_PKG_NAME").into())
	}

	/// Executable file description.
#[cfg(feature = "std")]
	fn description() -> String;

	/// Executable file author.
#[cfg(feature = "std")]
	fn author() -> String;

	/// Support URL.
#[cfg(feature = "std")]
	fn support_url() -> String;

	/// Copyright starting year (x-current year)
#[cfg(feature = "std")]
	fn copyright_start_year() -> i32;

	/// Chain spec factory
#[cfg(feature = "std")]
	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn ChainSpec>, String>;

	/// Helper function used to parse the command line arguments. This is the equivalent of
	/// [`clap::Parser::parse()`].
	///
	/// To allow running the node without subcommand, it also sets a few more settings:
	/// [`clap::Command::propagate_version`], [`clap::Command::args_conflicts_with_subcommands`],
	/// [`clap::Command::subcommand_negates_reqs`].
	///
	/// Creates `Self` from the command line arguments. Print the
	/// error message and quit the program in case of failure.
#[cfg(feature = "std")]
	fn from_args() -> Self
	where
		Self: Parser + Sized,
	{
		<Self as SubstrateCli>::from_iter(&mut std::env::args_os())
	}

	/// Helper function used to parse the command line arguments. This is the equivalent of
	/// [`clap::Parser::parse_from`].
	///
	/// To allow running the node without subcommand, it also sets a few more settings:
	/// [`clap::Command::propagate_version`], [`clap::Command::args_conflicts_with_subcommands`],
	/// [`clap::Command::subcommand_negates_reqs`].
	///
	/// Creates `Self` from any iterator over arguments.
	/// Print the error message and quit the program in case of failure.
#[cfg(feature = "std")]
	fn from_iter<I>(iter: I) -> Self
	where
		Self: Parser + Sized,
		I: IntoIterator,
		I::Item: Into<std::ffi::OsString> + Clone,
	{
		let app = <Self as CommandFactory>::command();
		let app = Self::setup_command(app);

		let matches = app.try_get_matches_from(iter).unwrap_or_else(|e| e.exit());

		<Self as FromArgMatches>::from_arg_matches(&matches).unwrap_or_else(|e| e.exit())
	}

	/// Helper function used to parse the command line arguments. This is the equivalent of
	/// [`clap::Parser::try_parse_from`]
	///
	/// To allow running the node without subcommand, it also sets a few more settings:
	/// [`clap::Command::propagate_version`], [`clap::Command::args_conflicts_with_subcommands`],
	/// [`clap::Command::subcommand_negates_reqs`].
	///
	/// Creates `Self` from any iterator over arguments.
	/// Print the error message and quit the program in case of failure.
	///
	/// **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are
	/// used. It will return a [`clap::Error`], where the [`clap::Error::kind`] is a
	/// [`clap::error::ErrorKind::DisplayHelp`] or [`clap::error::ErrorKind::DisplayVersion`]
	/// respectively. You must call [`clap::Error::exit`] or perform a [`std::process::exit`].
#[cfg(feature = "std")]
	fn try_from_iter<I>(iter: I) -> clap::error::Result<Self>
	where
		Self: Parser + Sized,
		I: IntoIterator,
		I::Item: Into<std::ffi::OsString> + Clone,
	{
		let app = <Self as CommandFactory>::command();
		let app = Self::setup_command(app);

		let matches = app.try_get_matches_from(iter)?;

		<Self as FromArgMatches>::from_arg_matches(&matches)
	}

	/// Returns the client ID: `{impl_name}/v{impl_version}`
#[cfg(feature = "std")]
	fn client_id() -> String {
		format!("{}/v{}", Self::impl_name(), Self::impl_version())
	}

	/// Only create a Configuration for the command provided in argument
#[cfg(feature = "std")]
	fn create_configuration<T: CliConfiguration<DVC>, DVC: DefaultConfigurationValues>(
		&self,
		command: &T,
		tokio_handle: tokio::runtime::Handle,
	) -> error::Result<Configuration> {
		command.create_configuration(self, tokio_handle)
	}

	/// Create a runner for the command provided in argument. This will create a Configuration and
	/// a tokio runtime
#[cfg(feature = "std")]
	fn create_runner<T: CliConfiguration<DVC>, DVC: DefaultConfigurationValues>(
		&self,
		command: &T,
	) -> Result<Runner<Self>> {
		self.create_runner_with_logger_hook(command, |_, _| {})
	}

	/// Create a runner for the command provided in argument. The `logger_hook` can be used to setup
	/// a custom profiler or update the logger configuration before it is initialized.
	///
	/// Example:
	/// ```
	/// use sc_tracing::{SpanDatum, TraceEvent};
	/// struct TestProfiler;
	///
	/// impl sc_tracing::TraceHandler for TestProfiler {
	///  	fn handle_span(&self, sd: &SpanDatum) {}
	/// 		fn handle_event(&self, _event: &TraceEvent) {}
	/// };
	///
	/// fn logger_hook() -> impl FnOnce(&mut soil_cli::LoggerBuilder, &soil_service::Configuration) -> () {
	/// 	|logger_builder, config| {
	/// 			logger_builder.with_custom_profiling(Box::new(TestProfiler{}));
	/// 	}
	/// }
	/// ```
#[cfg(feature = "std")]
	fn create_runner_with_logger_hook<
		T: CliConfiguration<DVC>,
		DVC: DefaultConfigurationValues,
		F,
	>(
		&self,
		command: &T,
		logger_hook: F,
	) -> Result<Runner<Self>>
	where
		F: FnOnce(&mut LoggerBuilder, &Configuration),
	{
		let tokio_runtime = build_runtime()?;

		// `capture` needs to be called in a tokio context.
		// Also capture them as early as possible.
		let signals = tokio_runtime.block_on(async { Signals::capture() })?;

		let config = command.create_configuration(self, tokio_runtime.handle().clone())?;

		command.init(&Self::support_url(), &Self::impl_version(), |logger_builder| {
			logger_hook(logger_builder, &config)
		})?;

		Runner::new(config, tokio_runtime, signals)
	}
	/// Augments a `clap::Command` with standard metadata like name, version, author, description,
	/// etc.
	///
	/// This is used internally in `from_iter`, `try_from_iter` and can be used externally
	/// to manually set up a command with Substrate CLI defaults.
#[cfg(feature = "std")]
	fn setup_command(mut cmd: clap::Command) -> clap::Command {
		let mut full_version = Self::impl_version();
		full_version.push('\n');

		cmd = cmd
			.name(Self::executable_name())
			.version(full_version)
			.author(Self::author())
			.about(Self::description())
			.long_about(Self::description())
			.after_help(format!("Support: {}", Self::support_url()))
			.propagate_version(true)
			.args_conflicts_with_subcommands(true)
			.subcommand_negates_reqs(true);

		cmd
	}
}
