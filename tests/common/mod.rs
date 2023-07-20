use std::ops::Deref;

use evilang_lib::ast::statement::{Statement, StatementList};
use evilang_lib::parser::parse;

pub type TestRes = ();

pub fn ensure_program(input: &str, expected: StatementList) -> TestRes {
	match parse(input.to_string()) {
		Ok(parsed) => {
			// println!("{:?}", parsed);
			assert_eq!(parsed.deref(), &expected, "Mismatched parsed AST and expected AST");
		}
		Err(error_type) => {
			panic!("{}", error_type)
		}
	}
	return;
}

#[allow(dead_code)]
pub fn ensure_parsed_statement(input: &str, expected: Statement) -> TestRes {
	return ensure_program(input, vec![expected]);
}
