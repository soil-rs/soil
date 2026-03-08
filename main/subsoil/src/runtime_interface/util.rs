// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Various utilities that help interfacing with wasm runtime code.

/// Pack a pointer and length into an `u64`.
pub fn pack_ptr_and_len(ptr: u32, len: u32) -> u64 {
	// The static assertions from above are changed into a runtime check.
	#[cfg(all(substrate_runtime, feature = "disable_target_static_assertions"))]
	assert_eq!(4, core::mem::size_of::<usize>());

	(u64::from(len) << 32) | u64::from(ptr)
}

/// Unpacks an `u64` into the pointer and length.
///
/// Runtime API functions return a 64-bit value which encodes a pointer in the least-significant
/// 32-bits and a length in the most-significant 32 bits. This interprets the returned value as a
/// pointer, length tuple.
pub fn unpack_ptr_and_len(val: u64) -> (u32, u32) {
	// The static assertions from above are changed into a runtime check.
	#[cfg(all(substrate_runtime, feature = "disable_target_static_assertions"))]
	assert_eq!(4, core::mem::size_of::<usize>());

	let ptr = (val & (!0u32 as u64)) as u32;
	let len = (val >> 32) as u32;

	(ptr, len)
}

#[cfg(test)]
mod tests {
	use super::{pack_ptr_and_len, unpack_ptr_and_len};

	#[test]
	fn ptr_len_packing_unpacking() {
		const PTR: u32 = 0x1337;
		const LEN: u32 = 0x7f000000;

		let packed = pack_ptr_and_len(PTR, LEN);
		let (ptr, len) = unpack_ptr_and_len(packed);

		assert_eq!(PTR, ptr);
		assert_eq!(LEN, len);
	}
}
