// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_support::*;

pub trait Animal {
	type Locomotion;
	type Diet;
	type SleepingStrategy;
	type Environment;

	fn animal_name() -> &'static str;
}

pub type RunsOnFourLegs = (usize, usize, usize, usize);
pub type RunsOnTwoLegs = (usize, usize);
pub type Swims = isize;
pub type Diurnal = bool;
pub type Nocturnal = Option<bool>;
pub type Omnivore = char;
pub type Land = ((), ());
pub type Sea = ((), (), ());
pub type Carnivore = (char, char);

pub struct FourLeggedAnimal {}

#[register_default_impl(FourLeggedAnimal)]
impl Animal for FourLeggedAnimal {
	type Locomotion = RunsOnFourLegs;
	type Diet = Omnivore;
	type SleepingStrategy = Diurnal;
	type Environment = Land;

	fn animal_name() -> &'static str {
		"A Four-Legged Animal"
	}
}

pub struct AquaticMammal {}

#[derive_impl(FourLeggedAnimal as Animal)]
struct Something {}

fn main() {}
