use std::mem::size_of;
use std::ops::Deref;

use evilang_lib::ast::{Statement, StatementList};
use evilang_lib::parser::parse;

type TestRes = ();

fn ensure_program(input: &str, expected: StatementList) -> TestRes {
	match parse(input.to_string()) {
		Ok(parsed) => {
			// println!("{:#?}", parsed);
			assert_eq!(parsed.deref(), &expected, "Mismatched parsed AST and expected AST");
		}
		Err(error_type) => {
			panic!("{}", error_type)
		}
	}
	return;
}

fn main() {
	dbg!(size_of::<Statement>());
	dbg!(size_of::<String>());
	dbg!(size_of::<StatementList>());
}
