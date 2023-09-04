use proc_macro2::TokenStream;
use quote::quote;

#[allow(non_snake_case)]
pub struct CrateImports {
	pub ResultWithError: TokenStream,
	pub Descriptor: TokenStream,
	pub ErrorT: TokenStream,
	pub EvilangError: TokenStream,
	pub RuntimeError: TokenStream,
	pub VariablesMap: TokenStream,
	pub Environment: TokenStream,
	pub RuntimeObject: TokenStream,
	pub GcPtrToObject: TokenStream,
	pub gc_ptr_cell_from: TokenStream,
	pub PrimitiveValue: TokenStream,
	pub expect_object_fn: TokenStream,
	pub FunctionReturnValue: TokenStream,
	pub FunctionParameters: TokenStream,
	pub concat_str: TokenStream,
	pub INativeClass: TokenStream,
	pub INativeClass_BuildClass: TokenStream,
	pub from_option_of_primitive_value: TokenStream,
	pub NativeClassMemberFunctionContext: TokenStream,
	pub NativeClassStaticFunctionContext: TokenStream,
	pub native_wrap: TokenStream,
	pub native_unwrap_exec_fn: TokenStream,
	pub Some_: TokenStream,
	pub None_: TokenStream,
	pub Err_: TokenStream,
	pub Ok_: TokenStream,
	pub HashMap: TokenStream,
	pub module: TokenStream,
	pub INativeClass_IsStructWrapper: TokenStream,
}

impl CrateImports {
	pub fn new(module: TokenStream) -> CrateImports {
		CrateImports {
			ResultWithError: quote! {#module::errors::ResultWithError},
			Descriptor: quote! {#module::errors::Descriptor},
			ErrorT: quote! {#module::errors::ErrorT},
			EvilangError: quote! {#module::errors::EvilangError},
			RuntimeError: quote! {#module::errors::RuntimeError},
			VariablesMap: quote! {#module::interpreter::variables_containers::VariablesMap},
			Environment: quote! {#module::interpreter::environment::Environment},
			RuntimeObject: quote! {#module::interpreter::runtime_values::objects::runtime_object::RuntimeObject},
			GcPtrToObject: quote! {#module::interpreter::runtime_values::objects::runtime_object::GcPtrToObject},
			gc_ptr_cell_from: quote! {#module::types::cell_ref::gc_ptr_cell_from},
			PrimitiveValue: quote! {#module::interpreter::runtime_values::PrimitiveValue},
			expect_object_fn: quote! {#module::interpreter::utils::expect_object_fn},
			FunctionReturnValue: quote! {#module::interpreter::runtime_values::functions::types::FunctionReturnValue},
			FunctionParameters: quote! {#module::interpreter::runtime_values::functions::types::FunctionParameters},
			concat_str: quote! {#module::types::consts::concat_str},
			INativeClass: quote! {#module::interpreter::runtime_values::i_native_struct::INativeClass},
			INativeClass_BuildClass: quote! {#module::interpreter::runtime_values::i_native_struct::INativeClass_BuildClass},
			INativeClass_IsStructWrapper: quote! {#module::interpreter::runtime_values::i_native_struct::INativeClass_IsStructWrapper},
			from_option_of_primitive_value: quote! {#module::interpreter::runtime_values::i_native_struct::from_option_of_primitive_value},
			NativeClassMemberFunctionContext: quote! {#module::interpreter::runtime_values::i_native_struct::NativeClassMemberFunctionContext},
			NativeClassStaticFunctionContext: quote! {#module::interpreter::runtime_values::i_native_struct::NativeClassStaticFunctionContext},
			native_wrap: quote! {#module::interpreter::runtime_values::i_native_struct::native_wrap},
			native_unwrap_exec_fn: quote! {#module::interpreter::runtime_values::i_native_struct::native_unwrap_exec_fn},
			Some_: quote! {::std::option::Option::Some},
			None_: quote! {::std::option::Option::None},
			Err_: quote! {::std::result::Result::Err},
			Ok_: quote! {::std::result::Result::Ok},
			HashMap: quote! {std::collections::HashMap},
			module,
		}
	}
}
