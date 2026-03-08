// This file is part of Soil.

// Copyright (C) Soil contributors.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[rustversion::before(1.68)]
fn main() {
	if !cfg!(feature = "std") {
		println!("cargo:rustc-cfg=enable_alloc_error_handler");
	}
}

#[rustversion::since(1.68)]
fn main() {}
