#![cfg_attr(not(feature = "std"), no_std)]

//! Re-export of [`subsoil::txpool`].

pub mod runtime_api {
	pub use subsoil::txpool::runtime_api::*;
}
