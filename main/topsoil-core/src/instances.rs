// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Some instance placeholder to be used in [`topsoil_core::pallet`] attribute macro.
//!
//! [`topsoil_core::pallet`] attribute macro does only requires the instance generic `I` to be
//! static (contrary to `decl_*` macro which requires instance generic to implement
//! [`topsoil_core::traits::Instance`]).
//!
//! Thus support provides some instance types to be used, This allow some instantiable pallet to
//! depend on specific instance of another:
//! ```
//! # mod another_pallet { pub trait Config<I: 'static = ()> {} }
//! pub trait Config<I: 'static = ()>: another_pallet::Config<I> {}
//! ```
//!
//! NOTE: [`topsoil_core::pallet`] will reexport them inside the module, in order to make them
//! accessible to [`topsoil_core::construct_runtime`].

/// `Instance1` to be used for instantiable pallets defined with the
/// [`#[pallet]`](`topsoil_core::pallet`) macro. Instances 2-16 are also available but are hidden
/// from docs.
#[derive(Clone, Copy, PartialEq, Eq, crate::DebugNoBound)]
pub struct Instance1;

/// `Instance2` to be used for instantiable pallets defined with the `#[pallet]` macro.
#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq, crate::DebugNoBound)]
pub struct Instance2;

/// `Instance3` to be used for instantiable pallets defined with the `#[pallet]` macro.
#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq, crate::DebugNoBound)]
pub struct Instance3;

/// `Instance4` to be used for instantiable pallets defined with the `#[pallet]` macro.
#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq, crate::DebugNoBound)]
pub struct Instance4;

/// `Instance5` to be used for instantiable pallets defined with the `#[pallet]` macro.
#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq, crate::DebugNoBound)]
pub struct Instance5;

/// `Instance6` to be used for instantiable pallets defined with the `#[pallet]` macro.
#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq, crate::DebugNoBound)]
pub struct Instance6;

/// `Instance7` to be used for instantiable pallets defined with the `#[pallet]` macro.
#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq, crate::DebugNoBound)]
pub struct Instance7;

/// `Instance8` to be used for instantiable pallets defined with the `#[pallet]` macro.
#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq, crate::DebugNoBound)]
pub struct Instance8;

/// `Instance9` to be used for instantiable pallets defined with the `#[pallet]` macro.
#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq, crate::DebugNoBound)]
pub struct Instance9;

/// `Instance10` to be used for instantiable pallets defined with the `#[pallet]` macro.
#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq, crate::DebugNoBound)]
pub struct Instance10;

/// `Instance11` to be used for instantiable pallets defined with the `#[pallet]` macro.
#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq, crate::DebugNoBound)]
pub struct Instance11;

/// `Instance12` to be used for instantiable pallets defined with the `#[pallet]` macro.
#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq, crate::DebugNoBound)]
pub struct Instance12;

/// `Instance13` to be used for instantiable pallets defined with the `#[pallet]` macro.
#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq, crate::DebugNoBound)]
pub struct Instance13;

/// `Instance14` to be used for instantiable pallets defined with the `#[pallet]` macro.
#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq, crate::DebugNoBound)]
pub struct Instance14;

/// `Instance15` to be used for instantiable pallets defined with the `#[pallet]` macro.
#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq, crate::DebugNoBound)]
pub struct Instance15;

/// `Instance16` to be used for instantiable pallets defined with the `#[pallet]` macro.
#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq, crate::DebugNoBound)]
pub struct Instance16;
