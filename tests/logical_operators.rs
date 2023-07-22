use evilang_lib::ast::expression::Expression::{BinaryExpression, BooleanLiteral, Identifier, IntegerLiteral, NullLiteral};
use evilang_lib::ast::operator::Operator::{Equals, GreaterThan, LessThanOrEqualTo, LogicalAnd, LogicalOr, Multiplication, Plus};

use crate::common::{test_expression_and_assignment, TestRes};

mod common;

#[test]
fn logical_and() -> TestRes {
	return test_expression_and_assignment("x == null && value > 15;", BinaryExpression {
		operator: LogicalAnd,
		left: BinaryExpression {
			operator: Equals,
			left: Identifier("x".to_string()).into(),
			right: NullLiteral.into(),
		}.into(),
		right: BinaryExpression {
			operator: GreaterThan,
			left: Identifier("value".to_string()).into(),
			right: IntegerLiteral(15).into(),
		}.into(),
	});
}

#[test]
fn logical_or() -> TestRes {
	return test_expression_and_assignment("x == $ || _ <= 1;", BinaryExpression {
		operator: LogicalOr,
		left: BinaryExpression {
			operator: Equals,
			left: Identifier("x".to_string()).into(),
			right: Identifier("$".to_string()).into(),
		}.into(),
		right: BinaryExpression {
			operator: LessThanOrEqualTo,
			left: Identifier("_".to_string()).into(),
			right: IntegerLiteral(1).into(),
		}.into(),
	});
}

#[test]
fn complex_equality() -> TestRes {
	return test_expression_and_assignment("x * y + 13 > 15 == true || bool_val && false == null + 10;", BinaryExpression {
		operator: LogicalOr,
		left: BinaryExpression {
			operator: Equals,
			left: BinaryExpression {
				operator: GreaterThan,
				left: BinaryExpression {
					operator: Plus,
					left: BinaryExpression {
						operator: Multiplication,
						left: Identifier("x".to_string()).into(),
						right: Identifier("y".to_string()).into(),
					}.into(),
					right: IntegerLiteral(13).into(),
				}.into(),
				right: IntegerLiteral(15).into(),
			}.into(),
			right: BooleanLiteral(true).into(),
		}.into(),
		right: BinaryExpression {
			operator: LogicalAnd,
			left: Identifier("bool_val".to_string()).into(),
			right: BinaryExpression {
				operator: Equals,
				left: BooleanLiteral(false).into(),
				right: BinaryExpression {
					operator: Plus,
					left: NullLiteral.into(),
					right: IntegerLiteral(10).into(),
				}.into(),
			}.into(),
		}.into(),
	});
}
