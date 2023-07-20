use evilang_lib::ast::expression::BoxExpression;
use evilang_lib::ast::expression::Expression::{BinaryExpression, IntegerLiteral};
use evilang_lib::ast::operator::Operator::{Minus, Multiplication, Plus};

use crate::common::{ensure_program, TestRes};

mod common;

#[test]
fn addition() -> TestRes {
	ensure_program("1+2;", vec![BinaryExpression {
		operator: Plus,
		left: BoxExpression::from(IntegerLiteral(1)),
		right: BoxExpression::from(IntegerLiteral(2)),
	}.consume_as_statement()]);
}

#[test]
fn addition_and_subtraction() -> TestRes {
	ensure_program("1+2-3;", vec![BinaryExpression {
		operator: Minus,
		left: BoxExpression::from(BinaryExpression {
			operator: Plus,
			left: BoxExpression::from(IntegerLiteral(1)),
			right: BoxExpression::from(IntegerLiteral(2)),
		}),
		right: BoxExpression::from(IntegerLiteral(3)),
	}.consume_as_statement()]);
}

#[test]
fn multiplication() -> TestRes {
	ensure_program("2*3;", vec![BinaryExpression {
		operator: Multiplication,
		left: BoxExpression::from(IntegerLiteral(2)),
		right: BoxExpression::from(IntegerLiteral(3)),
	}.consume_as_statement()]);
}

#[test]
fn multiplication_2() -> TestRes {
	ensure_program("2*3*4;", vec![BinaryExpression {
		operator: Multiplication,
		left: BoxExpression::from(BinaryExpression {
			operator: Multiplication,
			left: BoxExpression::from(IntegerLiteral(2)),
			right: BoxExpression::from(IntegerLiteral(3)),
		}),
		right: BoxExpression::from(IntegerLiteral(4)),
	}.consume_as_statement()]);
}

#[test]
fn addition_and_multiplication() -> TestRes {
	ensure_program("2+3*4;", vec![BinaryExpression {
		operator: Plus,
		left: BoxExpression::from(IntegerLiteral(2)),
		right: BoxExpression::from(BinaryExpression {
			operator: Multiplication,
			left: BoxExpression::from(IntegerLiteral(3)),
			right: BoxExpression::from(IntegerLiteral(4)),
		}),
	}.consume_as_statement()]);

	ensure_program("2+3*4+5;", vec![BinaryExpression {
		operator: Plus,
		left: BoxExpression::from(BinaryExpression {
			operator: Plus,
			left: BoxExpression::from(IntegerLiteral(2)),
			right: BoxExpression::from(BinaryExpression {
				operator: Multiplication,
				left: BoxExpression::from(IntegerLiteral(3)),
				right: BoxExpression::from(IntegerLiteral(4)),
			}),
		}),
		right: BoxExpression::from(IntegerLiteral(5)),
	}.consume_as_statement()]);
}

#[test]
fn parenthesis() -> TestRes {
	ensure_program("(2+3)*4;", vec![BinaryExpression {
		operator: Multiplication,
		left: BoxExpression::from(BinaryExpression {
			operator: Plus,
			left: BoxExpression::from(IntegerLiteral(2)),
			right: BoxExpression::from(IntegerLiteral(3)),
		}),
		right: BoxExpression::from(IntegerLiteral(4)),
	}.consume_as_statement()]);
}
