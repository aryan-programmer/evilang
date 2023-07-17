#![allow(dead_code)]
use std::mem::size_of;
use std::ops::Deref;

use evilang_lib::ast::{Statement, StatementList};
use evilang_lib::parser::parse;

type TestRes = ();

fn print_program(input: &str) -> TestRes {
	match parse(input.to_string()) {
		Ok(parsed) => {
			println!("Input: {}\nParsed:\n{:#?}", input, parsed);
		}
		Err(error_type) => {
			panic!("{}", error_type)
		}
	}
	return;
}

fn ensure_program(input: &str, expected: StatementList) -> TestRes {
	match parse(input.to_string()) {
		Ok(parsed) => {
			println!("Input: {}\nParsed:\n{:#?}", input, parsed);
			assert_eq!(parsed.deref(), &expected, "Mismatched parsed AST and expected AST");
		}
		Err(error_type) => {
			panic!("{}", error_type)
		}
	}
	return;
}

fn main() {
	print_program("x += y = 1+2*(z=1)+4;");
	dbg!(size_of::<Statement>());
	dbg!(size_of::<String>());
	dbg!(size_of::<StatementList>());
}
