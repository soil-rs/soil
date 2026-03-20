// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! *Vesta* types and host functions.

use super::utils::{self, HostcallResult, FAIL_MSG};
use crate::runtime_interface::{
	pass_by::{PassFatPointerAndRead, PassFatPointerAndWrite},
	runtime_interface,
};
use alloc::vec::Vec;
use ark_ec::{AffineRepr, CurveConfig, CurveGroup};
use ark_vesta_ext::CurveHooks;

/// Group configuration.
pub type VestaConfig = ark_vesta_ext::VestaConfig<HostHooks>;
/// Short Weierstrass form point affine representation.
pub type Affine = ark_vesta_ext::Affine<HostHooks>;
/// Short Weierstrass form point projective representation.
pub type Projective = ark_vesta_ext::Projective<HostHooks>;

/// Group scalar field (Fr).
pub type ScalarField = <VestaConfig as CurveConfig>::ScalarField;

/// Curve hooks jumping into [`host_calls`] host functions.
#[derive(Copy, Clone)]
pub struct HostHooks;

impl CurveHooks for HostHooks {
	fn msm(bases: &[Affine], scalars: &[ScalarField]) -> Projective {
		let mut out = utils::buffer_for::<Affine>();
		host_calls::vesta_msm(&utils::encode(bases), &utils::encode(scalars), &mut out)
			.and_then(|_| utils::decode::<Affine>(&out))
			.expect(FAIL_MSG)
			.into_group()
	}

	fn mul_projective(base: &Projective, scalar: &[u64]) -> Projective {
		let mut out = utils::buffer_for::<Affine>();
		host_calls::vesta_mul(&utils::encode(base.into_affine()), &utils::encode(scalar), &mut out)
			.and_then(|_| utils::decode::<Affine>(&out))
			.expect(FAIL_MSG)
			.into_group()
	}
}

/// Interfaces for working with *Arkworks* *Vesta* elliptic curve related types
/// from within the runtime.
///
/// All types are (de-)serialized through the wrapper types from `ark-scale`.
///
/// `ArkScale`'s `Usage` generic parameter is expected to be set to "not-validated"
/// and "not-compressed".
#[runtime_interface]
pub trait HostCalls {
	/// Short Weierstrass multi scalar multiplication for *Vesta*.
	///
	/// Receives encoded:
	/// - `bases`: `Vec<Affine>`.
	/// - `scalars`: `Vec<ScalarField>`.
	/// Writes encoded `Affine` to `out`.
	fn vesta_msm(
		bases: PassFatPointerAndRead<&[u8]>,
		scalars: PassFatPointerAndRead<&[u8]>,
		out: PassFatPointerAndWrite<&mut [u8]>,
	) -> HostcallResult {
		utils::msm_sw::<ark_vesta::VestaConfig>(bases, scalars, out)
	}

	/// Short Weierstrass affine multiplication for *Vesta*.
	///
	/// Receives encoded:
	/// - `base`: `Affine`.
	/// - `scalar`: `BigInteger`.
	/// Writes encoded `Affine` to `out`.
	fn vesta_mul(
		base: PassFatPointerAndRead<&[u8]>,
		scalar: PassFatPointerAndRead<&[u8]>,
		out: PassFatPointerAndWrite<&mut [u8]>,
	) -> HostcallResult {
		utils::mul_sw::<ark_vesta::VestaConfig>(base, scalar, out)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::crypto_ec_utils::utils::testing::*;

	#[test]
	fn mul_works() {
		mul_test::<Affine, ark_vesta::Affine>();
	}

	#[test]
	fn msm_works() {
		msm_test::<Affine, ark_vesta::Affine>();
	}
}
