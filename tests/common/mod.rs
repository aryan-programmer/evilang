// Not really dead code
#![allow(dead_code)]

use std::ops::Deref;

use evilang_lib::ast::expression::Expression;
use evilang_lib::ast::expression::Expression::{AssignmentExpression, Identifier};
use evilang_lib::ast::operator::Operator::Assignment;
use evilang_lib::ast::statement::{Statement, StatementList};
use evilang_lib::ast::statement::Statement::ExpressionStatement;
use evilang_lib::errors::ErrorT;
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

pub fn ensure_program_fails(input: &str, typ: Option<ErrorT>) -> TestRes {
	match parse(input.to_string()) {
		Ok(parsed) => {
			// println!("{:?}", parsed);
			panic!("Program {} expected to fail parsed as {:#?}", input, parsed);
		}
		Err(error_type) => {
			if let Some(t) = typ {
				assert_eq!(t, error_type.typ, "Expected error types to match");
			}
		}
	}
	return;
}

pub fn ensure_parsed_statement(input: &str, expected: Statement) -> TestRes {
	return ensure_program(input, vec![expected]);
}

pub fn test_expression_and_assignment(input: &str, expr: Expression) -> TestRes {
	ensure_parsed_statement(input, expr.clone().consume_as_statement());
	let new_input = "y = ".to_string() + input;
	ensure_parsed_statement(new_input.as_str(), ExpressionStatement(
		AssignmentExpression {
			operator: Assignment,
			left: Identifier("y".to_string()).into(),
			right: expr.into(),
		},
	));
}
