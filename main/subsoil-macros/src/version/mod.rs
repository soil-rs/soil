// This file is part of Soil.

// Copyright (C) Soil contributors.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use proc_macro::TokenStream;

mod decl_runtime_version;

pub(crate) fn runtime_version(input: TokenStream) -> TokenStream {
	decl_runtime_version::decl_runtime_version_impl(input)
}
