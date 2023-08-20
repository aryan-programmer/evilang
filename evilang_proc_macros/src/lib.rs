use std::borrow::Cow;

use proc_macro2;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{Data, Fields, Index, parse_macro_input};
use syn::DeriveInput;
use syn::spanned::Spanned;

#[proc_macro_derive(Clone__SilentlyFail)]
pub fn clone_silently_fail_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = input.ident;

	let generics = input.generics;
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let sum = gen_try_clone_impl(&input.data, |name| quote! {::evilang_traits::Clone__SilentlyFail::clone__silently_fail(#name)});

	let expanded = quote! {
        impl #impl_generics ::evilang_traits::Clone__SilentlyFail for #name #ty_generics #where_clause {
			#[inline(always)]
            fn clone__silently_fail(&self) -> Self {
                #sum
            }
        }
    };

	// Hand the output tokens back to the compiler.
	proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(TryClone)]
pub fn try_clone_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = input.ident;

	let generics = input.generics;
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let sum = gen_try_clone_impl(&input.data, |name| quote! {::evilang_traits::TryClone::try_clone(#name)?});

	let expanded = quote! {
        impl #impl_generics ::evilang_traits::TryClone for #name #ty_generics #where_clause {
			#[inline(always)]
            fn try_clone(&self) -> Result<Self, ::evilang_traits::CloningFailureErrorT> {
                Ok(#sum)
            }
        }
    };

	// Hand the output tokens back to the compiler.
	proc_macro::TokenStream::from(expanded)
}

// Generate an expression to sum up the heap size of each field.
fn gen_try_clone_impl(data: &Data, clone_wrapper: fn(TokenStream) -> TokenStream) -> TokenStream {
	match *data {
		Data::Struct(ref data) => {
			let fields = &data.fields;
			let self_prefix = quote!(&self.);
			let clone = generate_clone_for_struct_fields(&self_prefix, fields, clone_wrapper);
			quote!(Self #clone)
		}
		Data::Enum(ref enum_data) => {
			let self_ident = quote! {Self};
			let match_arms = enum_data.variants.iter().map(|v| {
				let var_ident = &v.ident;
				let prefix = quote!(_self_);
				let destructuring = generate_destructuring_for_enum_variant_fields(&prefix, &v.fields);
				let clone = generate_clone_for_struct_fields(&prefix, &v.fields, clone_wrapper);
				quote_spanned! {v.span() => #self_ident::#var_ident #destructuring => {
					Self::#var_ident #clone
				}}
			});
			quote! {
				match self {
					#(#match_arms)*
				}
			}
		}
		Data::Union(_) => unimplemented!(),
	}
}

fn generate_destructuring_for_enum_variant_fields(
	prefix: &TokenStream,
	fields: &Fields,
) -> TokenStream {
	match fields {
		Fields::Named(ref fields) => {
			let recurse = fields.named.iter().map(|f| {
				let name_ident = &f.ident;
				let prefixxed_name = str_concat_token_stream(
					Cow::Borrowed(prefix),
					Cow::Owned(name_ident.as_ref().unwrap().to_token_stream()),
				);
				quote_spanned! {f.span()=>#name_ident: #prefixxed_name}
			});
			quote! { {#(#recurse),*} }
		}
		Fields::Unnamed(ref fields) => {
			let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
				let index = Index::from(i);
				let prefixxed_name = str_concat_token_stream(
					Cow::Borrowed(prefix),
					Cow::Owned(index.to_token_stream().into()),
				);
				quote_spanned! {f.span()=>#prefixxed_name}
			});
			quote! {(#(#recurse),*)}
		}
		Fields::Unit => {
			quote!()
		}
	}
}

fn generate_clone_for_struct_fields(
	prefix: &TokenStream,
	fields: &Fields,
	clone_wrapper: fn(TokenStream) -> TokenStream,
) -> TokenStream {
	match fields {
		Fields::Named(ref fields) => {
			let recurse = fields.named.iter().map(|f| {
				let name_ident = &f.ident;
				let prefixxed_name = str_concat_token_stream(
					Cow::Borrowed(prefix),
					Cow::Owned(name_ident.as_ref().unwrap().to_token_stream()),
				);
				let clone = clone_wrapper(prefixxed_name);
				quote_spanned! {f.span()=>#name_ident: #clone}
			});
			quote! { { #(#recurse),* } }
		}
		Fields::Unnamed(ref fields) => {
			let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
				let index = Index::from(i);
				let prefixxed_name = str_concat_token_stream(
					Cow::Borrowed(prefix),
					Cow::Owned(index.to_token_stream().into()),
				);
				let clone = clone_wrapper(prefixxed_name);
				quote_spanned! {f.span()=>#clone}
			});
			quote! {(#(#recurse),*)}
		}
		Fields::Unit => {
			quote!()
		}
	}
}

fn str_concat_token_stream(a: Cow<TokenStream>, b: Cow<TokenStream>) -> TokenStream {
	return (a.to_string() + b.to_string().as_str()).parse().unwrap();
}
