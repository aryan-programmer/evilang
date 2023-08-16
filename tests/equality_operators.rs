use evilang_lib::ast::expression::Expression;
use evilang_lib::ast::expression::Expression::{BinaryExpression, BooleanLiteral, Identifier, NullLiteral};
use evilang_lib::ast::operator::Operator::{Equals, GreaterThan, Multiplication, NotEquals, Plus};

use crate::common::{test_expression_and_assignment, TestRes};

mod common;

#[test]
fn equality() -> TestRes {
	return test_expression_and_assignment("x == null;", BinaryExpression {
		operator: Equals,
		left: Identifier("x".parse().unwrap()).into(),
		right: NullLiteral.into(),
	});
}

#[test]
fn inequality() -> TestRes {
	return test_expression_and_assignment("x != false;", BinaryExpression {
		operator: NotEquals,
		left: Identifier("x".parse().unwrap()).into(),
		right: BooleanLiteral(false).into(),
	});
}

#[test]
fn complex_equality() -> TestRes {
	return test_expression_and_assignment("x * y + 13 > 15 == true;", BinaryExpression {
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
				right: Expression::integer_literal(13).into(),
			}.into(),
			right: Expression::integer_literal(15).into(),
		}.into(),
		right: BooleanLiteral(true).into(),
	}, );
}
