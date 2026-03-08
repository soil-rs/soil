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

//! Useful function for inflation for nominated proof of stake.

use subsoil::arithmetic::{
	biguint::BigUint,
	traits::{SaturatedConversion, Zero},
	PerThing, Perquintill,
};

/// Compute yearly inflation using function
///
/// ```ignore
/// I(x) = for x between 0 and x_ideal: x / x_ideal,
/// for x between x_ideal and 1: 2^((x_ideal - x) / d)
/// ```
///
/// where:
/// * x is the stake rate, i.e. fraction of total issued tokens that actively staked behind
///   validators.
/// * d is the falloff or `decay_rate`
/// * x_ideal: the ideal stake rate.
///
/// The result is meant to be scaled with minimum inflation and maximum inflation.
///
/// (as detailed
/// [here](https://research.web3.foundation/Polkadot/overview/token-economics#inflation-model-with-parachains))
///
/// Arguments are:
/// * `stake`: The fraction of total issued tokens that actively staked behind validators. Known as
///   `x` in the literature. Must be between 0 and 1.
/// * `ideal_stake`: The fraction of total issued tokens that should be actively staked behind
///   validators. Known as `x_ideal` in the literature. Must be between 0 and 1.
/// * `falloff`: Known as `decay_rate` in the literature. A co-efficient dictating the strength of
///   the global incentivization to get the `ideal_stake`. A higher number results in less typical
///   inflation at the cost of greater volatility for validators. Must be more than 0.01.
pub fn compute_inflation<P: PerThing>(stake: P, ideal_stake: P, falloff: P) -> P {
	if stake < ideal_stake {
		return stake / ideal_stake;
	}

	if falloff < P::from_percent(1.into()) {
		log::error!("Invalid inflation computation: falloff less than 1% is not supported");
		return PerThing::zero();
	}

	let accuracy = {
		let mut a = BigUint::from(Into::<u128>::into(P::ACCURACY));
		a.lstrip();
		a
	};

	let mut falloff = BigUint::from(falloff.deconstruct().into());
	falloff.lstrip();

	let ln2 = {
		const LN2: u64 = 0_693_147_180_559_945_309;
		let ln2 = P::from_rational(LN2.into(), Perquintill::ACCURACY.into());
		BigUint::from(ln2.deconstruct().into())
	};

	let ln2_div_d = div_by_stripped(ln2.mul(&accuracy), &falloff);

	let inpos_param = INPoSParam {
		x_ideal: BigUint::from(ideal_stake.deconstruct().into()),
		x: BigUint::from(stake.deconstruct().into()),
		accuracy,
		ln2_div_d,
	};

	let res = compute_taylor_serie_part(&inpos_param);

	match u128::try_from(res.clone()) {
		Ok(res) if res <= Into::<u128>::into(P::ACCURACY) => P::from_parts(res.saturated_into()),
		_ => {
			log::error!("Invalid inflation computation: unexpected result {:?}", res);
			P::zero()
		},
	}
}

struct INPoSParam {
	ln2_div_d: BigUint,
	x_ideal: BigUint,
	x: BigUint,
	accuracy: BigUint,
}

fn compute_taylor_serie_part(p: &INPoSParam) -> BigUint {
	let mut last_taylor_term = p.accuracy.clone();
	let mut taylor_sum_positive = true;
	let mut taylor_sum = last_taylor_term.clone();

	for k in 1..300 {
		last_taylor_term = compute_taylor_term(k, &last_taylor_term, p);

		if last_taylor_term.is_zero() {
			break;
		}

		let last_taylor_term_positive = k % 2 == 0;

		if taylor_sum_positive == last_taylor_term_positive {
			taylor_sum = taylor_sum.add(&last_taylor_term);
		} else if taylor_sum >= last_taylor_term {
			taylor_sum = taylor_sum.sub(&last_taylor_term).unwrap_or_else(|e| e);
		} else {
			taylor_sum_positive = !taylor_sum_positive;
			taylor_sum = last_taylor_term.clone().sub(&taylor_sum).unwrap_or_else(|e| e);
		}
	}

	if !taylor_sum_positive {
		return BigUint::zero();
	}

	taylor_sum.lstrip();
	taylor_sum
}

fn compute_taylor_term(k: u32, previous_taylor_term: &BigUint, p: &INPoSParam) -> BigUint {
	let x_minus_x_ideal =
		p.x.clone().sub(&p.x_ideal).unwrap_or_else(|_| BigUint::zero());

	let res = previous_taylor_term.clone().mul(&x_minus_x_ideal).mul(&p.ln2_div_d).div_unit(k);
	let res = div_by_stripped(res, &p.accuracy);
	let mut res = div_by_stripped(res, &p.accuracy);

	res.lstrip();
	res
}

fn div_by_stripped(mut a: BigUint, b: &BigUint) -> BigUint {
	a.lstrip();

	if b.len() == 0 {
		log::error!("Computation error: Invalid division");
		return BigUint::zero();
	}

	if b.len() == 1 {
		return a.div_unit(b.checked_get(0).unwrap_or(1));
	}

	if b.len() > a.len() {
		return BigUint::zero();
	}

	if b.len() == a.len() {
		let mut new_a = a.mul(&BigUint::from(100_000u64.pow(2)));
		new_a.lstrip();

		debug_assert!(new_a.len() > b.len());
		return new_a
			.div(b, false)
			.map(|res| res.0)
			.unwrap_or_else(BigUint::zero)
			.div_unit(100_000)
			.div_unit(100_000);
	}

	a.div(b, false).map(|res| res.0).unwrap_or_else(BigUint::zero)
}

#[cfg(test)]
mod tests {
	use super::compute_inflation;
	use subsoil::arithmetic::{PerThing, PerU16, Perbill, Percent, Perquintill};

	fn test_precision<P: PerThing>(stake: P, ideal_stake: P, falloff: P) {
		let accuracy_f64 = Into::<u128>::into(P::ACCURACY) as f64;
		let res = compute_inflation(stake, ideal_stake, falloff);
		let res = Into::<u128>::into(res.deconstruct()) as f64 / accuracy_f64;
		let expect = float_i_npos(stake, ideal_stake, falloff);
		let error = (res - expect).abs();

		assert!(
			error <= 8f64 / accuracy_f64 || error <= 8.0 * f64::EPSILON,
			"stake: {:?}, ideal_stake: {:?}, falloff: {:?}, res: {}, expect: {}",
			stake,
			ideal_stake,
			falloff,
			res,
			expect,
		);
	}

	fn float_i_npos<P: PerThing>(stake: P, ideal_stake: P, falloff: P) -> f64 {
		let accuracy_f64 = Into::<u128>::into(P::ACCURACY) as f64;
		let ideal_stake = Into::<u128>::into(ideal_stake.deconstruct()) as f64 / accuracy_f64;
		let stake = Into::<u128>::into(stake.deconstruct()) as f64 / accuracy_f64;
		let falloff = Into::<u128>::into(falloff.deconstruct()) as f64 / accuracy_f64;

		if stake < ideal_stake {
			stake / ideal_stake
		} else {
			2_f64.powf((ideal_stake - stake) / falloff)
		}
	}

	#[test]
	fn test_precision_for_minimum_falloff() {
		fn test_falloff_precision_for_minimum_falloff<P: PerThing>() {
			for stake in 0..1_000 {
				let stake = P::from_rational(stake, 1_000);
				let ideal_stake = P::zero();
				let falloff = P::from_rational(1, 100);
				test_precision(stake, ideal_stake, falloff);
			}
		}

		test_falloff_precision_for_minimum_falloff::<Perquintill>();
		test_falloff_precision_for_minimum_falloff::<PerU16>();
		test_falloff_precision_for_minimum_falloff::<Perbill>();
		test_falloff_precision_for_minimum_falloff::<Percent>();
	}

	#[test]
	fn compute_inflation_works() {
		fn compute_inflation_works<P: PerThing>() {
			for stake in 0..100 {
				for ideal_stake in 0..10 {
					for falloff in 1..10 {
						let stake = P::from_rational(stake, 100);
						let ideal_stake = P::from_rational(ideal_stake, 10);
						let falloff = P::from_rational(falloff, 100);
						test_precision(stake, ideal_stake, falloff);
					}
				}
			}
		}

		compute_inflation_works::<Perquintill>();
		compute_inflation_works::<PerU16>();
		compute_inflation_works::<Perbill>();
		compute_inflation_works::<Percent>();
	}
}
