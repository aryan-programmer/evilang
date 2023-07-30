use evilang_lib::ast::expression::BoxExpression;
use evilang_lib::ast::expression::Expression::{BinaryExpression, IntegerLiteral};
use evilang_lib::ast::operator::Operator::{Division, Minus, Modulus, Multiplication, Plus};
use evilang_lib::interpreter::runtime_value::PrimitiveValue;

use crate::common::{ensure_program_statement_results, TestRes};

mod common;

#[test]
fn addition() -> TestRes {
	ensure_program_statement_results("1+2;", vec![BinaryExpression {
		operator: Plus,
		left: BoxExpression::from(IntegerLiteral(1)),
		right: BoxExpression::from(IntegerLiteral(2)),
	}.consume_as_statement()], vec![PrimitiveValue::Integer(3)]);
}

#[test]
fn addition_and_subtraction() -> TestRes {
	ensure_program_statement_results("1+2-3;", vec![BinaryExpression {
		operator: Minus,
		left: BoxExpression::from(BinaryExpression {
			operator: Plus,
			left: BoxExpression::from(IntegerLiteral(1)),
			right: BoxExpression::from(IntegerLiteral(2)),
		}),
		right: BoxExpression::from(IntegerLiteral(3)),
	}.consume_as_statement()], vec![PrimitiveValue::Integer(0)]);
}

#[test]
fn multiplication() -> TestRes {
	ensure_program_statement_results("2*3;", vec![BinaryExpression {
		operator: Multiplication,
		left: BoxExpression::from(IntegerLiteral(2)),
		right: BoxExpression::from(IntegerLiteral(3)),
	}.consume_as_statement()], vec![PrimitiveValue::Integer(6)]);
}

#[test]
fn multiplication_2() -> TestRes {
	ensure_program_statement_results("2*3*4;", vec![BinaryExpression {
		operator: Multiplication,
		left: BoxExpression::from(BinaryExpression {
			operator: Multiplication,
			left: BoxExpression::from(IntegerLiteral(2)),
			right: BoxExpression::from(IntegerLiteral(3)),
		}),
		right: BoxExpression::from(IntegerLiteral(4)),
	}.consume_as_statement()], vec![PrimitiveValue::Integer(24)]);
}

#[test]
fn addition_and_multiplication() -> TestRes {
	ensure_program_statement_results("2+3*4;", vec![BinaryExpression {
		operator: Plus,
		left: BoxExpression::from(IntegerLiteral(2)),
		right: BoxExpression::from(BinaryExpression {
			operator: Multiplication,
			left: BoxExpression::from(IntegerLiteral(3)),
			right: BoxExpression::from(IntegerLiteral(4)),
		}),
	}.consume_as_statement()], vec![PrimitiveValue::Integer(14)]);

	ensure_program_statement_results("2+3*4+5;", vec![BinaryExpression {
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
	}.consume_as_statement()], vec![PrimitiveValue::Integer(19)]);
}

#[test]
fn subtraction_division_and_modulus() -> TestRes {
	ensure_program_statement_results("2-3/4;", vec![BinaryExpression {
		operator: Minus,
		left: BoxExpression::from(IntegerLiteral(2)),
		right: BoxExpression::from(BinaryExpression {
			operator: Division,
			left: BoxExpression::from(IntegerLiteral(3)),
			right: BoxExpression::from(IntegerLiteral(4)),
		}),
	}.consume_as_statement()], vec![PrimitiveValue::Integer(2)]);

	ensure_program_statement_results("2-3%4-5;", vec![BinaryExpression {
		operator: Minus,
		left: BoxExpression::from(BinaryExpression {
			operator: Minus,
			left: BoxExpression::from(IntegerLiteral(2)),
			right: BoxExpression::from(BinaryExpression {
				operator: Modulus,
				left: BoxExpression::from(IntegerLiteral(3)),
				right: BoxExpression::from(IntegerLiteral(4)),
			}),
		}),
		right: BoxExpression::from(IntegerLiteral(5)),
	}.consume_as_statement()], vec![PrimitiveValue::Integer(-6)]);
}

#[test]
fn parenthesis() -> TestRes {
	ensure_program_statement_results("(2+3)*4;", vec![BinaryExpression {
		operator: Multiplication,
		left: BoxExpression::from(BinaryExpression {
			operator: Plus,
			left: BoxExpression::from(IntegerLiteral(2)),
			right: BoxExpression::from(IntegerLiteral(3)),
		}),
		right: BoxExpression::from(IntegerLiteral(4)),
	}.consume_as_statement()], vec![PrimitiveValue::Integer(20)]);
}
