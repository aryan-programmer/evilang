use evilang_lib::ast::expression::Expression::{BinaryExpression, Identifier, IntegerLiteral};
use evilang_lib::ast::operator::Operator::{Division, GreaterThan, GreaterThanOrEqualTo, LessThan, LessThanOrEqualTo};

use crate::common::{test_expression_and_assignment, TestRes};

mod common;

#[test]
fn gte() -> TestRes {
	return test_expression_and_assignment("x >= 12;", BinaryExpression {
		operator: GreaterThanOrEqualTo,
		left: Identifier("x".parse().unwrap()).into(),
		right: IntegerLiteral(12).into(),
	});
}

#[test]
fn lte() -> TestRes {
	return test_expression_and_assignment("x <= 12;", BinaryExpression {
		operator: LessThanOrEqualTo,
		left: Identifier("x".parse().unwrap()).into(),
		right: IntegerLiteral(12).into(),
	});
}

#[test]
fn gt() -> TestRes {
	return test_expression_and_assignment("x > 12;", BinaryExpression {
		operator: GreaterThan,
		left: Identifier("x".parse().unwrap()).into(),
		right: IntegerLiteral(12).into(),
	});
}

#[test]
fn lt() -> TestRes {
	return test_expression_and_assignment("x < 12;", BinaryExpression {
		operator: LessThan,
		left: Identifier("x".parse().unwrap()).into(),
		right: IntegerLiteral(12).into(),
	});
}

#[test]
fn gte_with_addition() -> TestRes {
	return test_expression_and_assignment("x / 2 >= 12;", BinaryExpression {
		operator: GreaterThanOrEqualTo,
		left: BinaryExpression {
			operator: Division,
			left: Identifier("x".parse().unwrap()).into(),
			right: IntegerLiteral(2).into(),
		}.into(),
		right: IntegerLiteral(12).into(),
	});
}
