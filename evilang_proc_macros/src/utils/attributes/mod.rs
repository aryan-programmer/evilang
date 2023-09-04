use std::ops::Index;

use darling::ast::NestedMeta;
use darling::FromMeta;
use itertools::Either;
use itertools::Either::{Left, Right};
use syn::{Attribute, Expr, Meta, Path};

use crate::utils::MODULE_NAME;

pub(crate) trait ParseArgs {
	fn parse_args(args: proc_macro2::TokenStream) -> Self;
}

impl<T: FromMeta> ParseArgs for T {
	fn parse_args(args: proc_macro2::TokenStream) -> Self {
		let attr_args = NestedMeta::parse_meta_list(args).unwrap();
		let res = Self::from_list(&attr_args).unwrap();
		res
	}
}

fn is_path_match(p: &Path, self_name: &str) -> bool {
	if p.segments.len() == 1 {
		return p.segments.index(0).ident == self_name;
	} else if p.segments.len() == 2 {
		let name = &p.segments.index(1).ident;
		let mod_name = &p.segments.index(0).ident;
		return name == self_name && mod_name == MODULE_NAME;
	}
	return false;
}

pub(crate) trait TryParseAttribute: Default + ParseArgs {
	const NAME: &'static str;

	fn new_from_value_expr(expr: &Expr) -> Self;

	fn set_attribute(&mut self, attr: Attribute);
	fn get_attribute(&self) -> Option<&Attribute>;

	fn try_parse_attribute(attr: Attribute) -> Either<Self, Attribute> {
		let mut rv = match &attr.meta {
			Meta::Path(v) if is_path_match(v, <Self as TryParseAttribute>::NAME) =>
				Self::default(),
			Meta::List(list) if is_path_match(&list.path, <Self as TryParseAttribute>::NAME) =>
				Self::parse_args(list.tokens.clone()),
			Meta::NameValue(nv) if is_path_match(&nv.path, <Self as TryParseAttribute>::NAME) =>
				Self::new_from_value_expr(&nv.value),
			_ => return Right(attr)
		};
		rv.set_attribute(attr);
		Left(rv)
	}
}
