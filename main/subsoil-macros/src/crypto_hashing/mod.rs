mod impls;

use impls::MultipleInputBytes;
use proc_macro::TokenStream;

pub(crate) fn blake2b_64(input: TokenStream) -> TokenStream {
	impls::blake2b_64(syn::parse_macro_input!(input as MultipleInputBytes).concatenated())
}

pub(crate) fn blake2b_256(input: TokenStream) -> TokenStream {
	impls::blake2b_256(syn::parse_macro_input!(input as MultipleInputBytes).concatenated())
}

pub(crate) fn blake2b_512(input: TokenStream) -> TokenStream {
	impls::blake2b_512(syn::parse_macro_input!(input as MultipleInputBytes).concatenated())
}

pub(crate) fn twox_64(input: TokenStream) -> TokenStream {
	impls::twox_64(syn::parse_macro_input!(input as MultipleInputBytes).concatenated())
}

pub(crate) fn twox_128(input: TokenStream) -> TokenStream {
	impls::twox_128(syn::parse_macro_input!(input as MultipleInputBytes).concatenated())
}

pub(crate) fn keccak_256(input: TokenStream) -> TokenStream {
	impls::keccak_256(syn::parse_macro_input!(input as MultipleInputBytes).concatenated())
}

pub(crate) fn keccak_512(input: TokenStream) -> TokenStream {
	impls::keccak_512(syn::parse_macro_input!(input as MultipleInputBytes).concatenated())
}

pub(crate) fn sha2_256(input: TokenStream) -> TokenStream {
	impls::sha2_256(syn::parse_macro_input!(input as MultipleInputBytes).concatenated())
}
