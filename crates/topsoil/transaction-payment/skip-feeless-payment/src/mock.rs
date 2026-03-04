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

use super::*;
use crate as topsoil_skip_feeless_payment;

use topsoil_support::{derive_impl, parameter_types};
use topsoil_system as system;
use soil_runtime::{
	traits::{DispatchOriginOf, TransactionExtension},
	transaction_validity::ValidTransaction,
};

type Block = topsoil_system::mocking::MockBlock<Runtime>;

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Runtime {
	type Block = Block;
}

impl Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
	pub static PrepareCount: u32 = 0;
	pub static ValidateCount: u32 = 0;
}

#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, DecodeWithMemTracking, TypeInfo)]
pub struct DummyExtension;

impl TransactionExtension<RuntimeCall> for DummyExtension {
	const IDENTIFIER: &'static str = "DummyExtension";
	type Implicit = ();
	type Val = ();
	type Pre = ();

	fn weight(&self, _: &RuntimeCall) -> Weight {
		Weight::zero()
	}

	fn validate(
		&self,
		origin: DispatchOriginOf<RuntimeCall>,
		_call: &RuntimeCall,
		_info: &DispatchInfoOf<RuntimeCall>,
		_len: usize,
		_self_implicit: Self::Implicit,
		_inherited_implication: &impl Encode,
		_source: TransactionSource,
	) -> ValidateResult<Self::Val, RuntimeCall> {
		ValidateCount::mutate(|c| *c += 1);
		Ok((ValidTransaction::default(), (), origin))
	}

	fn prepare(
		self,
		_val: Self::Val,
		_origin: &DispatchOriginOf<RuntimeCall>,
		_call: &RuntimeCall,
		_info: &DispatchInfoOf<RuntimeCall>,
		_len: usize,
	) -> Result<Self::Pre, TransactionValidityError> {
		PrepareCount::mutate(|c| *c += 1);
		Ok(())
	}
}

#[topsoil_support::pallet(dev_mode)]
pub mod pallet_dummy {
	use topsoil_support::pallet_prelude::*;
	use topsoil_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: topsoil_system::Config {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::feeless_if(|_origin: &OriginFor<T>, data: &u32| -> bool {
			*data == 0
		})]
		pub fn aux(_origin: OriginFor<T>, #[pallet::compact] _data: u32) -> DispatchResult {
			unreachable!()
		}
	}
}

impl pallet_dummy::Config for Runtime {}

topsoil_support::construct_runtime!(
	pub enum Runtime {
		System: system,
		SkipFeeless: topsoil_skip_feeless_payment,
		DummyPallet: pallet_dummy,
	}
);
