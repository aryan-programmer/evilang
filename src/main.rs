#![allow(dead_code)]

use std::mem::size_of;
use std::ops::Deref;

use evilang_lib::ast::expression::{Expression, MemberIndexer};
use evilang_lib::ast::operator::Operator;
use evilang_lib::ast::statement::{Statement, StatementList};
use evilang_lib::ast::structs::VariableDeclaration;
use evilang_lib::parser::parse;
use evilang_lib::tokenizer::{Token, TokenType};

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
	dbg!(size_of::<Statement>());
	dbg!(size_of::<Expression>());
	dbg!(size_of::<MemberIndexer>());
	dbg!(size_of::<Operator>());
	dbg!(size_of::<TokenType>());
	dbg!(size_of::<Token>());
	dbg!(size_of::<VariableDeclaration>());
	dbg!(size_of::<String>());
	dbg!(size_of::<StatementList>());
	print_program(r#"
	console.log("values");
"#);
}
