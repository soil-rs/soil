// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::Config;
use codec::{Decode, DecodeWithMemTracking, Encode};
use core::marker::PhantomData;
use scale_info::TypeInfo;
use subsoil::runtime::{
	traits::{DispatchInfoOf, TransactionExtension},
	transaction_validity::InvalidTransaction,
};
use topsoil_support::{pallet_prelude::TransactionSource, traits::OriginTrait, DefaultNoBound};

/// Check to ensure that the sender is not the zero address.
#[derive(Encode, Decode, DecodeWithMemTracking, DefaultNoBound, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct CheckNonZeroSender<T>(PhantomData<T>);

impl<T: Config + Send + Sync> core::fmt::Debug for CheckNonZeroSender<T> {
	#[cfg(feature = "std")]
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "CheckNonZeroSender")
	}

	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut core::fmt::Formatter) -> core::fmt::Result {
		Ok(())
	}
}

impl<T: Config + Send + Sync> CheckNonZeroSender<T> {
	/// Create new `TransactionExtension` to check runtime version.
	pub fn new() -> Self {
		Self(core::marker::PhantomData)
	}
}

impl<T: Config + Send + Sync> TransactionExtension<T::RuntimeCall> for CheckNonZeroSender<T> {
	const IDENTIFIER: &'static str = "CheckNonZeroSender";
	type Implicit = ();
	type Val = ();
	type Pre = ();

	fn weight(&self, _: &T::RuntimeCall) -> subsoil::weights::Weight {
		<T::ExtensionsWeightInfo as super::WeightInfo>::check_non_zero_sender()
	}

	fn validate(
		&self,
		origin: <T as Config>::RuntimeOrigin,
		_call: &T::RuntimeCall,
		_info: &DispatchInfoOf<T::RuntimeCall>,
		_len: usize,
		_self_implicit: Self::Implicit,
		_inherited_implication: &impl Encode,
		_source: TransactionSource,
	) -> subsoil::runtime::traits::ValidateResult<Self::Val, T::RuntimeCall> {
		if let Some(who) = origin.as_signer() {
			if who.using_encoded(|d| d.iter().all(|x| *x == 0)) {
				return Err(InvalidTransaction::BadSigner.into());
			}
		}
		Ok((Default::default(), (), origin))
	}
	subsoil::impl_tx_ext_default!(T::RuntimeCall; prepare);
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::{new_test_ext, Test, CALL};
	use subsoil::runtime::{
		traits::{AsTransactionAuthorizedOrigin, DispatchTransaction, TxBaseImplication},
		transaction_validity::{TransactionSource::External, TransactionValidityError},
	};
	use topsoil_support::{assert_ok, dispatch::DispatchInfo};

	#[test]
	fn zero_account_ban_works() {
		new_test_ext().execute_with(|| {
			let info = DispatchInfo::default();
			let len = 0_usize;
			assert_eq!(
				CheckNonZeroSender::<Test>::new()
					.validate_only(Some(0).into(), CALL, &info, len, External, 0)
					.unwrap_err(),
				TransactionValidityError::from(InvalidTransaction::BadSigner)
			);
			assert_ok!(CheckNonZeroSender::<Test>::new().validate_only(
				Some(1).into(),
				CALL,
				&info,
				len,
				External,
				0,
			));
		})
	}

	#[test]
	fn unsigned_origin_works() {
		new_test_ext().execute_with(|| {
			let info = DispatchInfo::default();
			let len = 0_usize;
			let (_, _, origin) = CheckNonZeroSender::<Test>::new()
				.validate(None.into(), CALL, &info, len, (), &TxBaseImplication(CALL), External)
				.unwrap();
			assert!(!origin.is_transaction_authorized());
		})
	}
}
