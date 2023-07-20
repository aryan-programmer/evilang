use evilang_lib::ast::expression::Expression;
use evilang_lib::ast::expression::Expression::{AssignmentExpression, BinaryExpression, Identifier, IntegerLiteral};
use evilang_lib::ast::operator::Operator::{Assignment, Division, GreaterThan, GreaterThanOrEqualTo, LessThan, LessThanOrEqualTo};
use evilang_lib::ast::statement::Statement::ExpressionStatement;

use crate::common::{ensure_parsed_statement, TestRes};

mod common;

fn relational_tests(input: &str, expr: Expression) -> TestRes {
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

#[test]
fn gte() -> TestRes {
	return relational_tests("x >= 12;", BinaryExpression {
		operator: GreaterThanOrEqualTo,
		left: Identifier("x".parse().unwrap()).into(),
		right: IntegerLiteral(12).into(),
	});
}

#[test]
fn lte() -> TestRes {
	return relational_tests("x <= 12;", BinaryExpression {
		operator: LessThanOrEqualTo,
		left: Identifier("x".parse().unwrap()).into(),
		right: IntegerLiteral(12).into(),
	});
}

#[test]
fn gt() -> TestRes {
	return relational_tests("x > 12;", BinaryExpression {
		operator: GreaterThan,
		left: Identifier("x".parse().unwrap()).into(),
		right: IntegerLiteral(12).into(),
	});
}

#[test]
fn lt() -> TestRes {
	return relational_tests("x < 12;", BinaryExpression {
		operator: LessThan,
		left: Identifier("x".parse().unwrap()).into(),
		right: IntegerLiteral(12).into(),
	});
}

#[test]
fn gte_with_addition() -> TestRes {
	return relational_tests("x / 2 >= 12;", BinaryExpression {
		operator: GreaterThanOrEqualTo,
		left: BinaryExpression {
			operator: Division,
			left: Identifier("x".parse().unwrap()).into(),
			right: IntegerLiteral(2).into(),
		}.into(),
		right: IntegerLiteral(12).into(),
	});
}
