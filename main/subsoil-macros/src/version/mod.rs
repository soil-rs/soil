use proc_macro::TokenStream;

mod decl_runtime_version;

pub(crate) fn runtime_version(input: TokenStream) -> TokenStream {
	decl_runtime_version::decl_runtime_version_impl(input)
}
