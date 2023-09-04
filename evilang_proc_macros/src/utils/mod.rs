use maybe_owned::MaybeOwned;
use proc_macro2::TokenStream;
use syn::{Expr, Lit};

pub(crate) mod attributes;
pub(crate) mod crate_imports;

pub(crate) const MODULE_NAME: &'static str = "evilang_lib";

pub(crate) fn expr_as_string(expr: &Expr) -> String {
	match expr {
		Expr::Lit(lit) => {
			match &lit.lit {
				Lit::Str(v) => {
					return v.value();
				}
				v => panic!("Invalid literal (expected a string literal): {0:#?}", v)
			}
		}
		v => panic!("Invalid expression (expected a string literal): {0:#?}", v)
	}
}

pub fn str_concat_token_stream(a: MaybeOwned<TokenStream>, b: MaybeOwned<TokenStream>) -> TokenStream {
	return (a.to_string() + b.to_string().as_str()).parse().unwrap();
}
