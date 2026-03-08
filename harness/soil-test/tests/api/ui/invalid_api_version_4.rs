// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

subsoil::api::decl_runtime_apis! {
	pub trait Api {
		#[api_version("1")]
		fn test(data: u64);
	}
}

fn main() {}
