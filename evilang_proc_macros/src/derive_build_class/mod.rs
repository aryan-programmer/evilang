#![allow(non_snake_case)]

use std::borrow::Cow;
use std::ops::Deref;

use darling::FromMeta;
use itertools::Either;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use split_iter::Splittable;
use syn::{Attribute, Expr, FnArg, ImplItem, ImplItemFn, ItemImpl, Path, Signature, Type};
use syn::spanned::Spanned;

use crate::utils::{expr_as_string, MODULE_NAME, str_concat_token_stream};
use crate::utils::attributes::{ParseArgs, TryParseAttribute};
use crate::utils::crate_imports::CrateImports;

fn define_alias_exports<'a, TIter: Iterator<Item=&'a FnExportData>>(
	imports: &CrateImports,
	iter: TIter,
	export_name: &Ident,
) -> Vec<TokenStream> {
	let CrateImports { ResultWithError, Environment, FunctionParameters, FunctionReturnValue, .. } = imports;
	iter.map(|export_attr| {
		let new_export_name = &export_attr.export_ident;
		return quote_spanned! { export_attr.attribute.get_span() =>
			#[inline(always)]
			pub fn #new_export_name(env: &mut #Environment, params: #FunctionParameters) -> #ResultWithError<#FunctionReturnValue> {
				return #export_name(env, params);
			}
		};
	}).collect::<Vec<_>>()
}

fn for_params<TIterElemType, TIter, TDrainRes, TDrainNextFn>(
	iter: TIter,
	drain_next_fn: TDrainNextFn,
) -> (Vec<TDrainRes>, Vec<Ident>)
	where TIter: Iterator<Item=TIterElemType>,
	      TDrainNextFn: Fn(usize, TIterElemType, &Ident) -> TDrainRes {
	let (params_decl_list, param_names_list): (Vec<_>, Vec<_>) = iter
		.enumerate()
		.map(|(i, v)| {
			let name = Ident::new(
				("_param_val_".to_string() + i.to_string().as_str()).as_str(),
				Span::call_site(),
			);
			(drain_next_fn(i, v, &name), name)
		}).unzip();
	(params_decl_list, param_names_list)
}

fn define_export_for_constructor(
	imports: &CrateImports,
	ExpT: &TokenStream,
	NATIVE_BOX_WRAP_NAME: &TokenStream,
	fn_dat: &FunctionData,
	// other_exports: TIter,
	ctor_export: &FnExportData,
) -> TokenStream {
	let CrateImports {
		ResultWithError,
		Environment,
		FunctionParameters,
		FunctionReturnValue,
		from_option_of_primitive_value,
		concat_str,
		expect_object_fn,
		Descriptor,
		NativeClassMemberFunctionContext,
		native_wrap,
		PrimitiveValue,
		Ok_,
		INativeClass,
		..
	} = imports;
	let export_name = &ctor_export.export_ident;
	let orig_name = &fn_dat.signature.ident;
	let (params_decl_list, param_names_list) = for_params(
		fn_dat.signature.inputs.iter().skip(1/*skip ctx*/),
		|_i, input, name| quote_spanned! {input.span() =>
			let #name = #from_option_of_primitive_value(drain.next())?;
		},
	);
	// let rest_exports = define_alias_exports(
	// 	&imports,
	// 	other_exports,
	// 	orig_name.to_string().into(),
	// 	&export_name,
	// );
	let res = quote_spanned! { ctor_export.attribute.get_span() =>
		pub fn #export_name(env: &mut #Environment, mut params: #FunctionParameters) ->
			#ResultWithError<#FunctionReturnValue> {
			const FUNC_NAME: &'static str = #concat_str!(
				<#ExpT as #INativeClass>::NAME,
				"::",
				::std::stringify!(#export_name)
			);
			const THIS_PARAM_NAME: &'static str = #concat_str!("this parameter of ", FUNC_NAME);
			let mut drain = params.drain(..);
			let this_val = drain.next().unwrap();
			#(#params_decl_list)*
			let this_obj = #expect_object_fn(
				&this_val,
				|| #Descriptor::Name(THIS_PARAM_NAME.into())
			)?;
			let new_obj = #ExpT::#orig_name(
				#NativeClassMemberFunctionContext::new(env, &this_val),
				#(#param_names_list),*
			)?;
			#native_wrap(this_obj, #NATIVE_BOX_WRAP_NAME.into(), new_obj);
			return #Ok_(#PrimitiveValue::Null);
		}
		// #(#rest_exports)*
	};
	res
}

fn define_export_for_member_function(
	imports: &CrateImports,
	ExpT: &TokenStream,
	NATIVE_BOX_WRAP_NAME: &TokenStream,
	fn_dat: &FunctionData,
) -> TokenStream {
	if fn_dat.exports.len() == 0 {
		return quote! {};
	}
	let CrateImports {
		ResultWithError,
		Environment,
		FunctionParameters,
		FunctionReturnValue,
		from_option_of_primitive_value,
		concat_str,
		NativeClassMemberFunctionContext,
		Ok_,
		native_unwrap_exec_fn,
		INativeClass,
		..
	} = imports;
	let mut iter = fn_dat.exports.iter();
	let main_export = iter.next().expect("Expected an export");
	let orig_name = &fn_dat.signature.ident;
	let export_name = &main_export.export_ident;
	let first_arg = fn_dat.signature.inputs.first().expect("Expected at-least one argument");
	let FnArg::Receiver(self_param) = first_arg else {
		return quote_spanned!(first_arg.span() =>
			compile_error!("Expected a self parameter for a member function")
		);
	};
	let is_self_mut = self_param.mutability.is_some();
	let borrow_type = if is_self_mut { quote!(borrow_mut) } else { quote!(borrow) };
	let (params_decl_list, param_names_list) = for_params(
		// The signature contains a self param which is transformed from a this parameter,
		fn_dat.signature.inputs.iter().skip(2/*skip self & ctx*/),
		|_i, input, name| quote_spanned! {input.span() =>
			let #name = #from_option_of_primitive_value(drain.next())?;
		},
	);
	let rest_exports = define_alias_exports(
		&imports,
		iter,
		&export_name,
	);
	let res = quote_spanned! { main_export.attribute.get_span() =>
		pub fn #export_name(env: &mut #Environment, mut params: #FunctionParameters) ->
			#ResultWithError<#FunctionReturnValue> {
			const FUNC_NAME: &'static str = #concat_str!(
				<#ExpT as #INativeClass>::NAME,
				"::",
				::std::stringify!(#export_name)
			);
			const THIS_PARAM_NAME: &'static str = #concat_str!("this parameter of ", FUNC_NAME);
			// if params.len() != #num_params_incl_this {
			// 	return #Err_(#EvilangError::new(#ErrorT::UnexpectedRuntimeError(#RuntimeError::InvalidNumberArgumentsToFunction {
			// 		got: params.len(),
			// 		expected: #Some_(::std::stringify!(#num_params_incl_this).to_string()),
			// 		func: #Descriptor::Name(FUNC_NAME.into()),
			// 	})));
			// }
			let mut drain = params.drain(..);
			let this_val = drain.next().unwrap();
			#(#params_decl_list)*
			let this_val_ref = &this_val;
			let result = #native_unwrap_exec_fn::<#ExpT, _, _, _>(
				&this_val,
				#NATIVE_BOX_WRAP_NAME.into(),
				move |v| v.#borrow_type().#orig_name(
					#NativeClassMemberFunctionContext::new(env, this_val_ref),
					#(#param_names_list),*
				),
				|| THIS_PARAM_NAME.into(),
			)?;
			return #Ok_(result.into());
		}
		#(#rest_exports)*
	};
	res
}

fn define_export_for_static_function(
	imports: &CrateImports,
	ExpT: &TokenStream,
	fn_dat: &FunctionData,
) -> TokenStream {
	if fn_dat.exports.len() == 0 {
		return quote! {};
	}
	let CrateImports {
		ResultWithError,
		Environment,
		FunctionParameters,
		FunctionReturnValue,
		from_option_of_primitive_value,
		NativeClassStaticFunctionContext,
		Ok_,
		..
	} = imports;
	let mut iter = fn_dat.exports.iter();
	let main_export = iter.next().expect("Expected an export");
	let orig_name = &fn_dat.signature.ident;
	let export_name = &main_export.export_ident;
	let (params_decl_list, param_names_list) = for_params(
		fn_dat.signature.inputs.iter().skip(1/*skip ctx*/),
		|_i, input, name| quote_spanned! {input.span() =>
			let #name = #from_option_of_primitive_value(drain.next())?;
		},
	);
	let rest_exports = define_alias_exports(
		&imports,
		iter,
		&export_name,
	);
	let res = quote_spanned! { main_export.attribute.get_span() =>
		pub fn #export_name(env: &mut #Environment, mut params: #FunctionParameters) ->
			#ResultWithError<#FunctionReturnValue> {
			let mut drain = params.drain(..);
			#(#params_decl_list)*
			let result = #ExpT::#orig_name(
				#NativeClassStaticFunctionContext::new(env),
				#(#param_names_list),*
			)?;
			return #Ok_(result.into());
		}
		#(#rest_exports)*
	};
	res
}

fn define_export_for_raw_native_function(
	imports: &CrateImports,
	ExpT: &TokenStream,
	fn_dat: &FunctionData,
) -> TokenStream {
	if fn_dat.exports.len() == 0 {
		return quote! {};
	}
	let CrateImports {
		ResultWithError,
		Environment,
		FunctionParameters,
		FunctionReturnValue,
		..
	} = imports;
	let mut iter = fn_dat.exports.iter();
	let main_export = iter.next().expect("Expected an export");
	let orig_name = &fn_dat.signature.ident;
	let export_name = &main_export.export_ident;
	let rest_exports = define_alias_exports(
		&imports,
		iter,
		&export_name,
	);
	let res = quote_spanned! { main_export.attribute.get_span() =>
		#[inline(always)]
		pub fn #export_name(env: &mut #Environment, params: #FunctionParameters) ->
			#ResultWithError<#FunctionReturnValue> {
			return #ExpT::#orig_name(env, params);
		}
		#(#rest_exports)*
	};
	res
}

#[derive(FromMeta, Debug, Default)]
pub(crate) struct RootAttributes {
	// #[darling(default)]
	// name: Option<String>,
	#[darling(default)]
	evilang_lib_crate: Option<Path>,
}

#[derive(FromMeta, Default, Debug, Clone)]
pub(crate) struct ExportAttribute {
	#[darling(default, rename = "_as")]
	export_as: Option<String>,
	raw: bool,
	#[darling(skip)]
	attribute: Option<Attribute>,
}

impl ExportAttribute {
	fn get_span(&self) -> Span {
		self.attribute.as_ref().map(Attribute::span).unwrap_or_else(Span::call_site)
	}
}

impl TryParseAttribute for ExportAttribute {
	const NAME: &'static str = "export";

	fn new_from_value_expr(expr: &Expr) -> Self {
		Self {
			export_as: Some(expr_as_string(expr)),
			attribute: None,
			raw: false,
		}
	}

	fn set_attribute(&mut self, attr: Attribute) {
		self.attribute = Some(attr)
	}

	fn get_attribute(&self) -> Option<&Attribute> {
		self.attribute.as_ref()
	}
}

#[derive(Debug)]
pub(crate) struct FnExportData {
	attribute: ExportAttribute,
	export_ident: Ident,
}

impl FnExportData {
	pub fn new(
		attribute: ExportAttribute,
		orig_name_str: Cow<str>,
	) -> Self {
		let export_ident = Ident::new(
			attribute.export_as
				.as_ref()
				.map(String::as_str)
				.unwrap_or_else(|| orig_name_str.as_ref()),
			Span::call_site(),
		);
		Self { attribute, export_ident }
	}
}

#[derive(Debug)]
pub(crate) struct FunctionData {
	signature: Signature,
	exports: Vec<FnExportData>,
}

impl FunctionData {
	pub fn new(signature: Signature, exports: Vec<ExportAttribute>) -> Self {
		let orig_name = signature.ident.to_string();
		let orig_name_str = orig_name.as_str();
		Self {
			signature,
			exports: exports
				.into_iter()
				.map(|v|
					FnExportData::new(v, orig_name_str.into())
				)
				.collect(),
		}
	}
}

#[derive(Debug)]
pub(crate) struct RootData {
	self_ty: Type,
	functions: Vec<FunctionData>,
	attributes: RootAttributes,
}

impl RootData {
	pub fn parse_and_strip_extra_attributes(attr: proc_macro::TokenStream, mut implementation: ItemImpl) -> (RootData, ItemImpl) {
		let mut functions = Vec::<FunctionData>::new();
		implementation = ItemImpl {
			items: implementation.items.into_iter().map(|impl_item| match impl_item {
				ImplItem::Fn(f) => {
					// dbg!(&f.attrs);
					let (exports_iter, attrs_iter) =
						f.attrs
							.into_iter()
							.map(ExportAttribute::try_parse_attribute)
							.split(Either::is_right);
					let exports: Vec<_> = exports_iter.map(Either::unwrap_left).collect();
					let attrs = attrs_iter.map(Either::unwrap_right).collect();
					// dbg!((&exports, &attrs));
					functions.push(FunctionData::new(
						f.sig.clone(),
						exports,
					));
					ImplItem::Fn(ImplItemFn {
						attrs,
						..f
					})
				}
				v => v
			}).collect(),
			..implementation
		};
		(Self {
			self_ty: implementation.self_ty.deref().clone(),
			functions,
			attributes: RootAttributes::parse_args(attr.into()),
		}, implementation)
	}

	pub fn generate_implementation(self) -> TokenStream {
		let module = self.attributes.evilang_lib_crate
			.as_ref()
			.map(Path::to_token_stream)
			.unwrap_or_else(|| quote! {::#MODULE_NAME});
		let imports = CrateImports::new(module);
		let CrateImports { ResultWithError, Environment, GcPtrToObject, PrimitiveValue, concat_str, INativeClass, INativeClass_BuildClass, Ok_, gc_ptr_cell_from, RuntimeObject, VariablesMap, HashMap, INativeClass_IsStructWrapper, .. } = &imports;
		let SelfT = &self.self_ty;
		let ExpT = quote! {super::#SelfT};
		let Self_exports = str_concat_token_stream(
			SelfT.to_token_stream().into(),
			quote! {_exports}.into(),
		);
		let NATIVE_BOX_WRAP_NAME = quote! {(<#ExpT as #INativeClass_IsStructWrapper>::NATIVE_BOX_WRAP_NAME)};

		// let get_export_tuples_for_function = |func: &FunctionData| {
		// 	func.exports.iter().map(|export| {
		// 		let name = &export.export_ident;
		// 		quote_spanned!(export.attribute.get_span()=>
		// 			(::std::stringify!(#name).into(), #gc_ptr_cell_from(#PrimitiveValue::new_native_function(#Self_exports::#name)))
		// 		)
		// 	})
		// };

		let export_tuples_for_functions = self.functions.iter().flat_map(|func| {
			func.exports.iter().map(|export| {
				let name = &export.export_ident;
				quote_spanned!(export.attribute.get_span()=>
					(::std::stringify!(#name).into(), #gc_ptr_cell_from(#PrimitiveValue::new_native_function(#Self_exports::#name)))
				)
			})
		});

		let handle_exports_for_function = |func: &FunctionData| {
			if func.exports.len() == 0 {
				return quote!();
			}
			if let Some(FnArg::Receiver(_)) = func.signature.inputs.first() {
				return define_export_for_member_function(
					&imports,
					&ExpT,
					&NATIVE_BOX_WRAP_NAME,
					func,
				);
			};
			{
				let (others_iter, ctors_iter) =
					func.exports
						.iter()
						.split(|v| v.export_ident == "constructor");
				let ctors: Vec<_> = ctors_iter.collect();
				if ctors.len() > 0 {
					if ctors.len() != 1 || others_iter.count() > 0 {
						return quote_spanned! {func.signature.span() =>
							compile_error!("A function exported as a constructor can not have multiple #[export] attributes");
						};
					}
					return define_export_for_constructor(
						&imports,
						&ExpT,
						&NATIVE_BOX_WRAP_NAME,
						func,
						// others,
						&ctors[0],
					);
				}
			}
			if func.exports.iter().any(|v| v.attribute.raw) {
				return define_export_for_raw_native_function(
					&imports,
					&ExpT,
					func,
				);
			}
			return define_export_for_static_function(
				&imports,
				&ExpT,
				func,
			);
		};

		let member_exports = self.functions.iter().map(|func| {
			let res = handle_exports_for_function(func);
			return quote_spanned!(func.signature.span()=>#res);
		});

		quote! {
			impl #INativeClass_IsStructWrapper for #SelfT {
				const NATIVE_BOX_WRAP_NAME: &'static str = #concat_str!("!native:", <#SelfT as #INativeClass>::NAME);
			}
			impl #INativeClass_BuildClass for #SelfT {
				fn build_class(env: &mut #Environment) -> #ResultWithError<#GcPtrToObject> {
					return #Ok_(#RuntimeObject::new_gc(#VariablesMap::new_direct(#HashMap::from([
						#(#export_tuples_for_functions),*
					])), <#SelfT as #INativeClass>::get_parent_class(env)?, <#SelfT as #INativeClass>::NAME.into()));
				}
			}
			#[allow(non_snake_case)]
			mod #Self_exports {
				#(#member_exports)*
			}
		}
	}
}
