// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

/// The declaration of the `Runtime` type is done by the `construct_runtime!` macro in a real
/// runtime.
struct Runtime {}

subsoil::api::decl_runtime_apis! {
	pub trait Api {
		fn test(data: u64);
	}
}

subsoil::api::impl_runtime_apis! {
	impl self::Api for Runtime {
		fn test(data: u64) {
			unimplemented!()
		}
	}
}

fn main() {}
