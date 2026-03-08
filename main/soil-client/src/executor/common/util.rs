// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Utilities used by all backends

use super::error::Result;
use std::ops::Range;
use subsoil::wasm_interface::Pointer;

/// Construct a range from an offset to a data length after the offset.
/// Returns None if the end of the range would exceed some maximum offset.
pub fn checked_range(offset: usize, len: usize, max: usize) -> Option<Range<usize>> {
	let end = offset.checked_add(len)?;
	(end <= max).then(|| offset..end)
}

/// Provides safe memory access interface using an external buffer
pub trait MemoryTransfer {
	/// Read data from a slice of memory into a newly allocated buffer.
	///
	/// Returns an error if the read would go out of the memory bounds.
	fn read(&self, source_addr: Pointer<u8>, size: usize) -> Result<Vec<u8>>;

	/// Read data from a slice of memory into a destination buffer.
	///
	/// Returns an error if the read would go out of the memory bounds.
	fn read_into(&self, source_addr: Pointer<u8>, destination: &mut [u8]) -> Result<()>;

	/// Write data to a slice of memory.
	///
	/// Returns an error if the write would go out of the memory bounds.
	fn write_from(&self, dest_addr: Pointer<u8>, source: &[u8]) -> Result<()>;
}
