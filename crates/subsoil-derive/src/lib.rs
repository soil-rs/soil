//! Unified procedural macros for the Subsoil runtime.

#![recursion_limit = "512"]

use proc_macro::TokenStream;

mod api;
mod crypto_hashing;
mod debug_derive;
mod runtime_interface;
mod tracing_macro;
mod version;

// --- api (3 proc-macro functions) ---

#[proc_macro]
pub fn impl_runtime_apis(input: TokenStream) -> TokenStream {
	api::impl_runtime_apis_impl(input)
}

#[proc_macro]
pub fn mock_impl_runtime_apis(input: TokenStream) -> TokenStream {
	api::mock_impl_runtime_apis_impl(input)
}

#[proc_macro]
pub fn decl_runtime_apis(input: TokenStream) -> TokenStream {
	api::decl_runtime_apis_impl(input)
}

// --- crypto_hashing (8 proc-macro functions) ---

#[proc_macro]
pub fn blake2b_64(input: TokenStream) -> TokenStream {
	crypto_hashing::blake2b_64(input)
}

#[proc_macro]
pub fn blake2b_256(input: TokenStream) -> TokenStream {
	crypto_hashing::blake2b_256(input)
}

#[proc_macro]
pub fn blake2b_512(input: TokenStream) -> TokenStream {
	crypto_hashing::blake2b_512(input)
}

#[proc_macro]
pub fn twox_64(input: TokenStream) -> TokenStream {
	crypto_hashing::twox_64(input)
}

#[proc_macro]
pub fn twox_128(input: TokenStream) -> TokenStream {
	crypto_hashing::twox_128(input)
}

#[proc_macro]
pub fn keccak_256(input: TokenStream) -> TokenStream {
	crypto_hashing::keccak_256(input)
}

#[proc_macro]
pub fn keccak_512(input: TokenStream) -> TokenStream {
	crypto_hashing::keccak_512(input)
}

#[proc_macro]
pub fn sha2_256(input: TokenStream) -> TokenStream {
	crypto_hashing::sha2_256(input)
}

// --- debug_derive (1 derive macro) ---

#[proc_macro_derive(RuntimeDebug)]
pub fn runtime_debug_derive(input: TokenStream) -> TokenStream {
	debug_derive::runtime_debug_derive(input)
}

// --- runtime_interface (1 attribute macro) ---

#[proc_macro_attribute]
pub fn runtime_interface(attrs: TokenStream, input: TokenStream) -> TokenStream {
	runtime_interface::runtime_interface(attrs, input)
}

// --- tracing (1 attribute macro) ---

#[proc_macro_attribute]
pub fn prefix_logs_with(arg: TokenStream, item: TokenStream) -> TokenStream {
	tracing_macro::prefix_logs_with(arg, item)
}

// --- version (1 attribute macro) ---

#[proc_macro_attribute]
pub fn runtime_version(_: TokenStream, input: TokenStream) -> TokenStream {
	version::runtime_version(input)
}
