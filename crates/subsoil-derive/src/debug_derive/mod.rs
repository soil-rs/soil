mod impls;

use proc_macro::TokenStream;
use quote::quote;

pub(crate) fn runtime_debug_derive(input: TokenStream) -> TokenStream {
	let input: syn::DeriveInput = syn::parse_macro_input!(input);
	let name = &input.ident;

	let warning = proc_macro_warning::Warning::new_deprecated(&format!("RuntimeDebug_{}", name))
		.old("derive `RuntimeDebug`")
		.new("derive `Debug`")
		.span(input.ident.span())
		.build_or_panic();

	let debug_impl: proc_macro2::TokenStream = impls::debug_derive(input).into();

	quote!(#warning #debug_impl).into()
}
