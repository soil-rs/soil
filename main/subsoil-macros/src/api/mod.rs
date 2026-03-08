// This file is part of Soil.

// Copyright (C) Soil contributors.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use proc_macro::TokenStream;

mod common;
mod decl_runtime_apis;
mod impl_runtime_apis;
mod mock_impl_runtime_apis;
mod runtime_metadata;
mod utils;

pub(crate) fn impl_runtime_apis_impl(input: TokenStream) -> TokenStream {
	impl_runtime_apis::impl_runtime_apis_impl(input)
}

pub(crate) fn mock_impl_runtime_apis_impl(input: TokenStream) -> TokenStream {
	mock_impl_runtime_apis::mock_impl_runtime_apis_impl(input)
}

pub(crate) fn decl_runtime_apis_impl(input: TokenStream) -> TokenStream {
	decl_runtime_apis::decl_runtime_apis_impl(input)
}
