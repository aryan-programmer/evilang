use evilang_lib::ast::expression::{BoxExpression, Expression};
use evilang_lib::ast::expression::Expression::BinaryExpression;
use evilang_lib::ast::operator::Operator::{Division, Minus, Modulus, Multiplication, Plus};
use evilang_lib::interpreter::runtime_values::PrimitiveValue;

use crate::common::{ensure_program_statement_results, TestRes};

mod common;

#[test]
fn addition() -> TestRes {
	ensure_program_statement_results("1+2;", vec![BinaryExpression {
		operator: Plus,
		left: BoxExpression::from(Expression::integer_literal(1)),
		right: BoxExpression::from(Expression::integer_literal(2)),
	}.consume_as_statement()], vec![PrimitiveValue::integer(3)]);
}

#[test]
fn addition_and_subtraction() -> TestRes {
	ensure_program_statement_results("1+2-3;", vec![BinaryExpression {
		operator: Minus,
		left: BoxExpression::from(BinaryExpression {
			operator: Plus,
			left: BoxExpression::from(Expression::integer_literal(1)),
			right: BoxExpression::from(Expression::integer_literal(2)),
		}),
		right: BoxExpression::from(Expression::integer_literal(3)),
	}.consume_as_statement()], vec![PrimitiveValue::integer(0)]);
}

#[test]
fn multiplication() -> TestRes {
	ensure_program_statement_results("2*3;", vec![BinaryExpression {
		operator: Multiplication,
		left: BoxExpression::from(Expression::integer_literal(2)),
		right: BoxExpression::from(Expression::integer_literal(3)),
	}.consume_as_statement()], vec![PrimitiveValue::integer(6)]);
}

#[test]
fn multiplication_2() -> TestRes {
	ensure_program_statement_results("2*3*4;", vec![BinaryExpression {
		operator: Multiplication,
		left: BoxExpression::from(BinaryExpression {
			operator: Multiplication,
			left: BoxExpression::from(Expression::integer_literal(2)),
			right: BoxExpression::from(Expression::integer_literal(3)),
		}),
		right: BoxExpression::from(Expression::integer_literal(4)),
	}.consume_as_statement()], vec![PrimitiveValue::integer(24)]);
}

#[test]
fn addition_and_multiplication() -> TestRes {
	ensure_program_statement_results("2+3*4;", vec![BinaryExpression {
		operator: Plus,
		left: BoxExpression::from(Expression::integer_literal(2)),
		right: BoxExpression::from(BinaryExpression {
			operator: Multiplication,
			left: BoxExpression::from(Expression::integer_literal(3)),
			right: BoxExpression::from(Expression::integer_literal(4)),
		}),
	}.consume_as_statement()], vec![PrimitiveValue::integer(14)]);

	ensure_program_statement_results("2+3*4+5;", vec![BinaryExpression {
		operator: Plus,
		left: BoxExpression::from(BinaryExpression {
			operator: Plus,
			left: BoxExpression::from(Expression::integer_literal(2)),
			right: BoxExpression::from(BinaryExpression {
				operator: Multiplication,
				left: BoxExpression::from(Expression::integer_literal(3)),
				right: BoxExpression::from(Expression::integer_literal(4)),
			}),
		}),
		right: BoxExpression::from(Expression::integer_literal(5)),
	}.consume_as_statement()], vec![PrimitiveValue::integer(19)]);
}

#[test]
fn subtraction_division_and_modulus() -> TestRes {
	ensure_program_statement_results("2-3/4;", vec![BinaryExpression {
		operator: Minus,
		left: BoxExpression::from(Expression::integer_literal(2)),
		right: BoxExpression::from(BinaryExpression {
			operator: Division,
			left: BoxExpression::from(Expression::integer_literal(3)),
			right: BoxExpression::from(Expression::integer_literal(4)),
		}),
	}.consume_as_statement()], vec![PrimitiveValue::float(1.25)]);

	ensure_program_statement_results("2-3%4-5;", vec![BinaryExpression {
		operator: Minus,
		left: BoxExpression::from(BinaryExpression {
			operator: Minus,
			left: BoxExpression::from(Expression::integer_literal(2)),
			right: BoxExpression::from(BinaryExpression {
				operator: Modulus,
				left: BoxExpression::from(Expression::integer_literal(3)),
				right: BoxExpression::from(Expression::integer_literal(4)),
			}),
		}),
		right: BoxExpression::from(Expression::integer_literal(5)),
	}.consume_as_statement()], vec![PrimitiveValue::integer(-6)]);
}

#[test]
fn parenthesis() -> TestRes {
	ensure_program_statement_results("(2+3)*4;", vec![BinaryExpression {
		operator: Multiplication,
		left: BinaryExpression {
			operator: Plus,
			left: BoxExpression::from(Expression::integer_literal(2)),
			right: BoxExpression::from(Expression::integer_literal(3)),
		}.consume_as_parenthesized().into(),
		right: BoxExpression::from(Expression::integer_literal(4)),
	}.consume_as_statement()], vec![PrimitiveValue::integer(20)]);
}
