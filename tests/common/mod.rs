// Not really dead code
#![allow(dead_code)]

use std::ops::Deref;

use evilang_lib::ast::expression::Expression;
use evilang_lib::ast::expression::Expression::{AssignmentExpression, Identifier};
use evilang_lib::ast::operator::Operator::Assignment;
use evilang_lib::ast::statement::Statement::ExpressionStatement;
use evilang_lib::ast::statement::StatementList;
use evilang_lib::errors::ErrorT;
use evilang_lib::interpreter::environment::Environment;
use evilang_lib::interpreter::runtime_value::PrimitiveValue;
use evilang_lib::parser::parse;

pub type TestRes = ();

pub fn ensure_program_ref(input: &str, expected: &StatementList) -> TestRes {
	match parse(input.to_string()) {
		Ok(parsed) => {
			// println!("{:?}", parsed);
			assert_eq!(parsed.deref(), expected, "Mismatched parsed AST and expected AST");
		}
		Err(error_type) => {
			panic!("{}", error_type)
		}
	}
	return;
}

pub fn ensure_program(input: &str, expected: StatementList) -> TestRes {
	return ensure_program_ref(input, &expected);
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

pub fn ensure_program_statement_results(
	input: &str,
	expected: StatementList,
	results: Vec<PrimitiveValue>,
) -> TestRes {
	ensure_program_ref(input, &expected);
	let mut env = Environment::new_with_parent(None);
	assert_eq!(expected.len(), results.len(), "Expected lengths of expected Statements list and expected results list to match");
	for (stmt, expected_val) in expected.iter().zip(results.iter()) {
		if let ExpressionStatement(expr) = stmt {
			let value = env.eval(expr).unwrap();
			let borrow = value.borrow();
			let got_val = borrow.deref();
			assert_eq!(got_val, expected_val, "Expected values to match");
		} else {
			env.run_statement(stmt).unwrap();
			assert_eq!(&PrimitiveValue::Null, expected_val, "Expected expected value to be null for not expression statements");
		}
	}

	return;
}

pub fn test_expression_and_assignment(input: &str, expr: Expression) -> TestRes {
	ensure_program(input, vec![expr.clone().consume_as_statement()]);
	let new_input = "y = ".to_string() + input;
	ensure_program(new_input.as_str(), vec![ExpressionStatement(
		AssignmentExpression {
			operator: Assignment,
			left: Identifier("y".to_string()).into(),
			right: expr.into(),
		},
	)]);
}
