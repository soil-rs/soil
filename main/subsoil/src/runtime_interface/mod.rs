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

//! Substrate runtime interface
//!
//! This module provides types, traits and macros around runtime interfaces. A runtime interface is
//! a fixed interface between a Substrate runtime (also called the "guest") and a Substrate node
//! (also called the "host"). For a native runtime the interface maps to direct function calls of
//! the implementation. For a non-native runtime the interface maps to an external function call.
//! These external functions are exported by the runtime and they map to the same implementation
//! as the native calls, just with some extra code to marshal them through the FFI boundary.
//!
//! # Using a type in a runtime interface
//!
//! Every argument type and return type must be wrapped in a marker newtype specifying the
//! marshalling strategy used to pass the value through the FFI boundary between the host
//! and the runtime. The only exceptions to this rule are a couple of basic, primitive types
//! which can be passed directly through the FFI boundary and which don't require any special
//! handling besides a straightforward, direct conversion.
//!
//! You can find the strategy wrapper types in the [`pass_by`] module.
//!
//! The newtype wrappers are automatically stripped away when the function is called
//! and applied when the function returns by the `runtime_interface` macro.
//!
//! # Declaring a runtime interface
//!
//! Declaring a runtime interface is similar to declaring a trait in Rust:
//!
//! ```ignore
//! # mod wrapper {
//! # use subsoil::runtime_interface::pass_by::PassFatPointerAndRead;
//!
//! #[subsoil::runtime_interface::runtime_interface]
//! trait RuntimeInterface {
//!     fn some_function(value: PassFatPointerAndRead<&[u8]>) -> bool {
//!         value.iter().all(|v| *v > 125)
//!     }
//! }
//! # }
//! ```
//!
//! For more information on declaring a runtime interface, see
//! [`#[runtime_interface]`](attr.runtime_interface.html).

#[doc(hidden)]
#[cfg(not(substrate_runtime))]
pub use crate::wasm_interface;

#[doc(hidden)]
pub use crate::std;

/// Attribute macro for transforming a trait declaration into a runtime interface.
///
/// A runtime interface is a fixed interface between a Substrate compatible runtime and the
/// native node. This interface is callable from a native and a wasm runtime. The macro will
/// generate the corresponding code for the native implementation and the code for calling from
/// the wasm side to the native implementation.
///
/// The macro expects the runtime interface declaration as trait declaration:
///
/// ```ignore
/// # mod wrapper {
/// # use subsoil::runtime_interface::runtime_interface;
/// # use subsoil::runtime_interface::pass_by::{PassFatPointerAndDecode, PassFatPointerAndRead, AllocateAndReturnFatPointer};
///
/// #[runtime_interface]
/// trait Interface {
///     /// A function that can be called from native/wasm.
///     ///
///     /// The implementation given to this function is only compiled on native.
///     fn call(data: PassFatPointerAndRead<&[u8]>) -> AllocateAndReturnFatPointer<Vec<u8>> {
///         // Here you could call some rather complex code that only compiles on native or
///         // is way faster in native than executing it in wasm.
///         Vec::new()
///     }
///     /// Call function, but different version.
///     ///
///     /// For new runtimes, only function with latest version is reachable.
///     /// But old version (above) is still accessible for old runtimes.
///     /// Default version is 1.
///     #[version(2)]
///     fn call(data: PassFatPointerAndRead<&[u8]>) -> AllocateAndReturnFatPointer<Vec<u8>> {
///         // Here you could call some rather complex code that only compiles on native or
///         // is way faster in native than executing it in wasm.
///         [17].to_vec()
///     }
///
///     /// Call function, different version and only being registered.
///     ///
///     /// This `register_only` version is only being registered, aka exposed to the runtime,
///     /// but the runtime will still use the version 2 of this function. This is useful for when
///     /// new host functions should be introduced. Adding new host functions requires that all
///     /// nodes have the host functions available, because otherwise they fail at instantiation
///     /// of the runtime. With `register_only` the function will not be used when compiling the
///     /// runtime, but it will already be there for a future version of the runtime that will
///     /// switch to using these host function.
///     #[version(3, register_only)]
///     fn call(data: PassFatPointerAndRead<&[u8]>) -> AllocateAndReturnFatPointer<Vec<u8>> {
///         // Here you could call some rather complex code that only compiles on native or
///         // is way faster in native than executing it in wasm.
///         [18].to_vec()
///     }
///
///     /// A function can take a `&self` or `&mut self` argument to get access to the
///     /// `Externalities`. (The generated method does not require
///     /// this argument, so the function can be called just with the `optional` argument)
///     fn set_or_clear(&mut self, optional: PassFatPointerAndDecode<Option<Vec<u8>>>) {
///         match optional {
///             Some(value) => self.set_storage([1, 2, 3, 4].to_vec(), value),
///             None => self.clear_storage(&[1, 2, 3, 4]),
///         }
///     }
///
///     /// A function can be gated behind a configuration (`cfg`) attribute.
///     /// To prevent ambiguity and confusion about what will be the final exposed host
///     /// functions list, conditionally compiled functions can't be versioned.
///     /// That is, conditionally compiled functions with `version`s greater than 1
///     /// are not allowed.
///     #[cfg(feature = "experimental-function")]
///     fn gated_call(data: PassFatPointerAndRead<&[u8]>) -> AllocateAndReturnFatPointer<Vec<u8>> {
///         [42].to_vec()
///     }
/// }
/// # }
/// ```
///
/// # Argument and return types
///
/// Every argument type and return type must be wrapped in a marker newtype specifying the
/// marshalling strategy used to pass the value through the FFI boundary between the host
/// and the runtime. The only exceptions to this rule are a couple of basic, primitive types
/// which can be passed directly through the FFI boundary and which don't require any special
/// handling besides a straightforward, direct conversion.
///
/// The following table documents those types which can be passed between the host and the
/// runtime without a marshalling strategy wrapper:
///
/// | Type | FFI type | Conversion |
/// |----|----|----|
/// | `u8` | `u32` | zero-extended to 32-bits |
/// | `u16` | `u32` | zero-extended to 32-bits |
/// | `u32` | `u32` | `Identity` |
/// | `u64` | `u64` | `Identity` |
/// | `i8` | `i32` | sign-extended to 32-bits |
/// | `i16` | `i32` | sign-extended to 32-bits |
/// | `i32` | `i32` | `Identity` |
/// | `i64` | `i64` | `Identity` |
/// | `bool` | `u32` | `if v { 1 } else { 0 }` |
/// | `*const T` | `u32` | `Identity` |
///
/// `Identity` means that the value is passed as-is directly in a bit-exact fashion.
///
/// You can find the strategy wrapper types in the [`pass_by`] module.
///
/// The newtype wrappers are automatically stripped away when the function is called
/// and applied when the function returns by the `runtime_interface` macro.
///
/// # Wasm only interfaces
///
/// Some interfaces are only required from within the wasm runtime e.g. the allocator
/// interface. To support this, the macro can be called like `#[runtime_interface(wasm_only)]`.
/// This instructs the macro to make two significant changes to the generated code:
///
/// 1. The generated functions are not callable from the native side.
/// 2. The trait as shown above is not implemented for [`Externalities`] and is instead
/// implemented for `FunctionContext` (from `sp-wasm-interface`).
///
/// # Disable tracing
/// By adding `no_tracing` to the list of options you can prevent the wasm-side interface from
/// generating the default `sp-tracing`-calls. Note that this is rarely needed but only meant
/// for the case when that would create a circular dependency. You usually _do not_ want to add
/// this flag, as tracing doesn't cost you anything by default anyways (it is added as a no-op)
/// but is super useful for debugging later.
pub use subsoil_macros::runtime_interface;

#[doc(hidden)]
#[cfg(not(substrate_runtime))]
pub use crate::externalities::{
	set_and_run_with_externalities, with_externalities, ExtensionStore, Externalities,
	ExternalitiesExt,
};

#[doc(hidden)]
pub use codec;

pub use alloc;

#[cfg(all(any(target_arch = "riscv32", target_arch = "riscv64"), substrate_runtime))]
pub mod polkavm;

#[cfg(not(substrate_runtime))]
pub mod host;
pub(crate) mod impls;
pub mod pass_by;
#[cfg(any(substrate_runtime, doc))]
pub mod wasm;

mod util;

pub use util::{pack_ptr_and_len, unpack_ptr_and_len};

/// Something that can be used by the runtime interface as type to communicate between the runtime
/// and the host.
///
/// Every type that should be used in a runtime interface function signature needs to implement
/// this trait.
pub trait RIType: Sized {
	/// The raw FFI type that is used to pass `Self` through the host <-> runtime boundary.
	#[cfg(not(substrate_runtime))]
	type FFIType: crate::wasm_interface::IntoValue
		+ crate::wasm_interface::TryFromValue
		+ crate::wasm_interface::WasmTy;

	#[cfg(substrate_runtime)]
	type FFIType;

	/// The inner type without any serialization strategy wrapper.
	type Inner;
}

/// A raw pointer that can be used in a runtime interface function signature.
#[cfg(substrate_runtime)]
pub type Pointer<T> = *mut T;

/// A raw pointer that can be used in a runtime interface function signature.
#[cfg(not(substrate_runtime))]
pub type Pointer<T> = crate::wasm_interface::Pointer<T>;
