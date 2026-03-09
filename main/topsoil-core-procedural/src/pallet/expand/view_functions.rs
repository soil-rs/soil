// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::pallet::{parse::view_functions::ViewFunctionDef, Def};
use proc_macro2::{Span, TokenStream};
use syn::spanned::Spanned;

pub fn expand_view_functions(def: &Def) -> TokenStream {
	let (span, where_clause, view_fns) = match def.view_functions.as_ref() {
		Some(view_fns) => {
			(view_fns.attr_span, view_fns.where_clause.clone(), view_fns.view_functions.clone())
		},
		None => (def.item.span(), def.config.where_clause.clone(), Vec::new()),
	};

	let view_function_prefix_impl =
		expand_view_function_prefix_impl(def, span, where_clause.as_ref());

	let view_fn_impls = view_fns
		.iter()
		.map(|view_fn| expand_view_function(def, span, where_clause.as_ref(), view_fn));
	let impl_dispatch_view_function =
		impl_dispatch_view_function(def, span, where_clause.as_ref(), &view_fns);
	let impl_view_function_metadata =
		impl_view_function_metadata(def, span, where_clause.as_ref(), &view_fns);

	quote::quote! {
		#view_function_prefix_impl
		#( #view_fn_impls )*
		#impl_dispatch_view_function
		#impl_view_function_metadata
	}
}

fn expand_view_function_prefix_impl(
	def: &Def,
	span: Span,
	where_clause: Option<&syn::WhereClause>,
) -> TokenStream {
	let pallet_ident = &def.pallet_struct.pallet;
	let topsoil_core = &def.topsoil_core;
	let topsoil_system = &def.topsoil_system;
	let type_impl_gen = &def.type_impl_generics(span);
	let type_use_gen = &def.type_use_generics(span);

	quote::quote! {
		impl<#type_impl_gen> #topsoil_core::view_functions::ViewFunctionIdPrefix for #pallet_ident<#type_use_gen> #where_clause {
			fn prefix() -> [::core::primitive::u8; 16usize] {
				<
					<T as #topsoil_system::Config>::PalletInfo
					as #topsoil_core::traits::PalletInfo
				>::name_hash::<Pallet<#type_use_gen>>()
					.expect("No name_hash found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.")
			}
		}
	}
}

fn expand_view_function(
	def: &Def,
	span: Span,
	where_clause: Option<&syn::WhereClause>,
	view_fn: &ViewFunctionDef,
) -> TokenStream {
	let topsoil_core = &def.topsoil_core;
	let pallet_ident = &def.pallet_struct.pallet;
	let type_impl_gen = &def.type_impl_generics(span);
	let type_decl_bounded_gen = &def.type_decl_bounded_generics(span);
	let type_use_gen = &def.type_use_generics(span);
	let capture_docs = if cfg!(feature = "no-metadata-docs") { "never" } else { "always" };

	let view_function_struct_ident = view_fn.view_function_struct_ident();
	let view_fn_name = &view_fn.name;
	let (arg_names, arg_types) = match view_fn.args_names_types() {
		Ok((arg_names, arg_types)) => (arg_names, arg_types),
		Err(e) => return e.into_compile_error(),
	};
	let return_type = &view_fn.return_type;
	let docs = &view_fn.docs;

	let view_function_id_suffix_bytes_raw = match view_fn.view_function_id_suffix_bytes() {
		Ok(view_function_id_suffix_bytes_raw) => view_function_id_suffix_bytes_raw,
		Err(e) => return e.into_compile_error(),
	};
	let view_function_id_suffix_bytes = view_function_id_suffix_bytes_raw
		.map(|byte| syn::LitInt::new(&format!("0x{:X}_u8", byte), Span::call_site()));

	quote::quote! {
		#( #[doc = #docs] )*
		#[allow(missing_docs)]
		#[derive(
			#topsoil_core::DebugNoBound,
			#topsoil_core::CloneNoBound,
			#topsoil_core::EqNoBound,
			#topsoil_core::PartialEqNoBound,
			#topsoil_core::__private::codec::Encode,
			#topsoil_core::__private::codec::Decode,
			#topsoil_core::__private::codec::DecodeWithMemTracking,
			#topsoil_core::__private::scale_info::TypeInfo,
		)]
		#[codec(encode_bound())]
		#[codec(decode_bound())]
		#[scale_info(skip_type_params(#type_use_gen), capture_docs = #capture_docs)]
		pub struct #view_function_struct_ident<#type_decl_bounded_gen> #where_clause {
			#(
				pub #arg_names: #arg_types,
			)*
			_marker: ::core::marker::PhantomData<(#type_use_gen,)>,
		}

		impl<#type_impl_gen> #view_function_struct_ident<#type_use_gen> #where_clause {
			/// Create a new [`#view_function_struct_ident`] instance.
			pub fn new(#( #arg_names: #arg_types, )*) -> Self {
				Self {
					#( #arg_names, )*
					_marker: ::core::default::Default::default()
				}
			}
		}

		impl<#type_impl_gen> #topsoil_core::view_functions::ViewFunctionIdSuffix for #view_function_struct_ident<#type_use_gen> #where_clause {
			const SUFFIX: [::core::primitive::u8; 16usize] = [ #( #view_function_id_suffix_bytes ),* ];
		}

		impl<#type_impl_gen> #topsoil_core::view_functions::ViewFunction for #view_function_struct_ident<#type_use_gen> #where_clause {
			fn id() -> #topsoil_core::view_functions::ViewFunctionId {
				#topsoil_core::view_functions::ViewFunctionId {
					prefix: <#pallet_ident<#type_use_gen> as #topsoil_core::view_functions::ViewFunctionIdPrefix>::prefix(),
					suffix: <Self as #topsoil_core::view_functions::ViewFunctionIdSuffix>::SUFFIX,
				}
			}

			type ReturnType = #return_type;

			fn invoke(self) -> Self::ReturnType {
				let Self { #( #arg_names, )* _marker } = self;
				#pallet_ident::<#type_use_gen> :: #view_fn_name( #( #arg_names, )* )
			}
		}
	}
}

fn impl_dispatch_view_function(
	def: &Def,
	span: Span,
	where_clause: Option<&syn::WhereClause>,
	view_fns: &[ViewFunctionDef],
) -> TokenStream {
	let topsoil_core = &def.topsoil_core;
	let pallet_ident = &def.pallet_struct.pallet;
	let type_impl_gen = &def.type_impl_generics(span);
	let type_use_gen = &def.type_use_generics(span);

	let query_match_arms = view_fns.iter().map(|view_fn| {
		let view_function_struct_ident = view_fn.view_function_struct_ident();
		quote::quote! {
			<#view_function_struct_ident<#type_use_gen> as #topsoil_core::view_functions::ViewFunctionIdSuffix>::SUFFIX => {
				<#view_function_struct_ident<#type_use_gen> as #topsoil_core::view_functions::ViewFunction>::execute(input, output)
			}
		}
	});

	quote::quote! {
		impl<#type_impl_gen> #topsoil_core::view_functions::DispatchViewFunction
			for #pallet_ident<#type_use_gen> #where_clause
		{
			#[deny(unreachable_patterns)]
			fn dispatch_view_function<O: #topsoil_core::__private::codec::Output>(
				id: & #topsoil_core::view_functions::ViewFunctionId,
				input: &mut &[u8],
				output: &mut O
			) -> Result<(), #topsoil_core::view_functions::ViewFunctionDispatchError>
			{
				match id.suffix {
					#( #query_match_arms )*
					_ => Err(#topsoil_core::view_functions::ViewFunctionDispatchError::NotFound(id.clone())),
				}
			}
		}
	}
}

fn impl_view_function_metadata(
	def: &Def,
	span: Span,
	where_clause: Option<&syn::WhereClause>,
	view_fns: &[ViewFunctionDef],
) -> TokenStream {
	let topsoil_core = &def.topsoil_core;
	let pallet_ident = &def.pallet_struct.pallet;
	let type_impl_gen = &def.type_impl_generics(span);
	let type_use_gen = &def.type_use_generics(span);

	let view_functions = view_fns.iter().map(|view_fn| {
		let view_function_struct_ident = view_fn.view_function_struct_ident();
		let name = &view_fn.name;
		let inputs = view_fn.args.iter().filter_map(|fn_arg| {
			match fn_arg {
				syn::FnArg::Receiver(_) => None,
				syn::FnArg::Typed(typed) => {
					let pat = &typed.pat;
					let ty = &typed.ty;
					Some(quote::quote! {
						#topsoil_core::__private::metadata_ir::PalletViewFunctionParamMetadataIR {
							name: ::core::stringify!(#pat),
							ty: #topsoil_core::__private::scale_info::meta_type::<#ty>(),
						}
					})
				}
			}
		});

		let no_docs = vec![];
		let doc = if cfg!(feature = "no-metadata-docs") { &no_docs } else { &view_fn.docs };

		let deprecation = match crate::deprecation::get_deprecation(
			&quote::quote! { #topsoil_core },
			&def.item.attrs,
		) {
			Ok(deprecation) => deprecation,
			Err(e) => return e.into_compile_error(),
		};

		quote::quote! {
			#topsoil_core::__private::metadata_ir::PalletViewFunctionMetadataIR {
				name: ::core::stringify!(#name),
				id: <#view_function_struct_ident<#type_use_gen> as #topsoil_core::view_functions::ViewFunction>::id().into(),
				inputs: #topsoil_core::__private::subsoil::std::vec![ #( #inputs ),* ],
				output: #topsoil_core::__private::scale_info::meta_type::<
					<#view_function_struct_ident<#type_use_gen> as #topsoil_core::view_functions::ViewFunction>::ReturnType
				>(),
				docs: #topsoil_core::__private::subsoil::std::vec![ #( #doc ),* ],
				deprecation_info: #deprecation,
			}
		}
	});

	quote::quote! {
		impl<#type_impl_gen> #pallet_ident<#type_use_gen> #where_clause {
			#[doc(hidden)]
			pub fn pallet_view_functions_metadata()
				-> #topsoil_core::__private::Vec<#topsoil_core::__private::metadata_ir::PalletViewFunctionMetadataIR> {
				#topsoil_core::__private::vec![ #( #view_functions ),* ]
			}
		}
	}
}
