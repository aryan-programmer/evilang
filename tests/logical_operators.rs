use evilang_lib::ast::expression::Expression::{BinaryExpression, BooleanLiteral, Identifier, IntegerLiteral, NullLiteral};
use evilang_lib::ast::operator::Operator::{Equals, GreaterThan, LessThanOrEqualTo, LogicalAnd, LogicalOr, Multiplication, Plus};
use evilang_lib::interpreter::runtime_value::PrimitiveValue;

use crate::common::{test_expression_and_assignment, TestData, TestRes};

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

#[test]
fn interpretation_basic() -> TestRes {
	TestData::new(r#"
	false || false;
	true || false;
	false || true;
	true || true;
	false && false;
	true && false;
	false && true;
	true && true;
"#.to_string())
		.expect_results(vec![
			// Or
			PrimitiveValue::Boolean(false),
			PrimitiveValue::Boolean(true),
			PrimitiveValue::Boolean(true),
			PrimitiveValue::Boolean(true),
			// And
			PrimitiveValue::Boolean(false),
			PrimitiveValue::Boolean(false),
			PrimitiveValue::Boolean(false),
			PrimitiveValue::Boolean(true),
		])
		.check();
}

#[test]
fn short_circuiting() -> TestRes {
	TestData::new(r#"
	false || push_res_stack(1);
	true || push_res_stack(2);
	false && push_res_stack(3);
	true && push_res_stack(4);
	false || push_res_stack(5) != null || push_res_stack(6) || true || push_res_stack(7);
	false || push_res_stack(8) == null || push_res_stack(9);
"#.to_string())
		.expect_results(vec![
			PrimitiveValue::Null, // push_res_stack
			PrimitiveValue::Boolean(true),
			PrimitiveValue::Boolean(false),
			PrimitiveValue::Null, // push_res_stack
			PrimitiveValue::Boolean(true),
			PrimitiveValue::Boolean(true),
		])
		.expect_stack(vec![
			PrimitiveValue::Integer(1),
			PrimitiveValue::Integer(4),
			PrimitiveValue::Integer(5),
			PrimitiveValue::Integer(6),
			PrimitiveValue::Integer(8),
		])
		.check()
}
