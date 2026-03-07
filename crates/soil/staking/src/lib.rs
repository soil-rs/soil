#![cfg_attr(not(feature = "std"), no_std)]

//! Re-export of [`subsoil::staking`].

pub use subsoil::staking::*;

pub mod currency_to_vote {
	pub use subsoil::staking::currency_to_vote::*;
}

pub mod offence {
	pub use subsoil::staking::offence::*;
}
