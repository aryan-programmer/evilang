use evilang_lib::ast::expression::{BoxExpression, Expression};
use evilang_lib::ast::expression::Expression::{BinaryExpression};
use evilang_lib::ast::operator::Operator::{Division, Minus, Modulus, Multiplication, Plus};
use evilang_lib::interpreter::runtime_values::PrimitiveValue;

use crate::common::{ensure_program_statement_results, TestRes};

mod common;

#[test]
fn addition() -> TestRes {
	ensure_program_statement_results("0.1+0.2;", vec![BinaryExpression {
		operator: Plus,
		left: BoxExpression::from(Expression::float_literal(0.1)),
		right: BoxExpression::from(Expression::float_literal(0.2)),
	}.consume_as_statement()], vec![PrimitiveValue::float(0.1+0.2)]);
}

#[test]
fn addition_and_subtraction() -> TestRes {
	ensure_program_statement_results("0.1+0.2-0.3;", vec![BinaryExpression {
		operator: Minus,
		left: BoxExpression::from(BinaryExpression {
			operator: Plus,
			left: BoxExpression::from(Expression::float_literal(0.1)),
			right: BoxExpression::from(Expression::float_literal(0.2)),
		}),
		right: BoxExpression::from(Expression::float_literal(0.3)),
	}.consume_as_statement()], vec![PrimitiveValue::float(0.1+0.2-0.3)]);
}

#[test]
fn multiplication() -> TestRes {
	ensure_program_statement_results("0.2*0.3;", vec![BinaryExpression {
		operator: Multiplication,
		left: BoxExpression::from(Expression::float_literal(0.2)),
		right: BoxExpression::from(Expression::float_literal(0.3)),
	}.consume_as_statement()], vec![PrimitiveValue::float(0.2*0.3)]);
}

#[test]
fn multiplication_2() -> TestRes {
	ensure_program_statement_results("0.2*0.3*0.4;", vec![BinaryExpression {
		operator: Multiplication,
		left: BoxExpression::from(BinaryExpression {
			operator: Multiplication,
			left: BoxExpression::from(Expression::float_literal(0.2)),
			right: BoxExpression::from(Expression::float_literal(0.3)),
		}),
		right: BoxExpression::from(Expression::float_literal(0.4)),
	}.consume_as_statement()], vec![PrimitiveValue::float(0.2*0.3*0.4)]);
}

#[test]
fn addition_and_multiplication() -> TestRes {
	ensure_program_statement_results("0.2+0.3*0.4;", vec![BinaryExpression {
		operator: Plus,
		left: BoxExpression::from(Expression::float_literal(0.2)),
		right: BoxExpression::from(BinaryExpression {
			operator: Multiplication,
			left: BoxExpression::from(Expression::float_literal(0.3)),
			right: BoxExpression::from(Expression::float_literal(0.4)),
		}),
	}.consume_as_statement()], vec![PrimitiveValue::float(0.2+0.3*0.4)]);

	ensure_program_statement_results("0.2+0.3*0.4+0.5;", vec![BinaryExpression {
		operator: Plus,
		left: BoxExpression::from(BinaryExpression {
			operator: Plus,
			left: BoxExpression::from(Expression::float_literal(0.2)),
			right: BoxExpression::from(BinaryExpression {
				operator: Multiplication,
				left: BoxExpression::from(Expression::float_literal(0.3)),
				right: BoxExpression::from(Expression::float_literal(0.4)),
			}),
		}),
		right: BoxExpression::from(Expression::float_literal(0.5)),
	}.consume_as_statement()], vec![PrimitiveValue::float(0.2+0.3*0.4+0.5)]);
}

#[test]
fn subtraction_division_and_modulus() -> TestRes {
	ensure_program_statement_results("0.2-0.3/0.4;", vec![BinaryExpression {
		operator: Minus,
		left: BoxExpression::from(Expression::float_literal(0.2)),
		right: BoxExpression::from(BinaryExpression {
			operator: Division,
			left: BoxExpression::from(Expression::float_literal(0.3)),
			right: BoxExpression::from(Expression::float_literal(0.4)),
		}),
	}.consume_as_statement()], vec![PrimitiveValue::float(0.2-0.3/0.4)]);

	ensure_program_statement_results("0.2-0.3%0.4-0.5;", vec![BinaryExpression {
		operator: Minus,
		left: BoxExpression::from(BinaryExpression {
			operator: Minus,
			left: BoxExpression::from(Expression::float_literal(0.2)),
			right: BoxExpression::from(BinaryExpression {
				operator: Modulus,
				left: BoxExpression::from(Expression::float_literal(0.3)),
				right: BoxExpression::from(Expression::float_literal(0.4)),
			}),
		}),
		right: BoxExpression::from(Expression::float_literal(0.5)),
	}.consume_as_statement()], vec![PrimitiveValue::float(0.2-0.3%0.4-0.5)]);
}

#[test]
fn parenthesis() -> TestRes {
	ensure_program_statement_results("(0.2+0.3)*0.4;", vec![BinaryExpression {
		operator: Multiplication,
		left: BinaryExpression {
			operator: Plus,
			left: BoxExpression::from(Expression::float_literal(0.2)),
			right: BoxExpression::from(Expression::float_literal(0.3)),
		}.consume_as_parenthesized().into(),
		right: BoxExpression::from(Expression::float_literal(0.4)),
	}.consume_as_statement()], vec![PrimitiveValue::float((0.2+0.3)*0.4)]);
}
