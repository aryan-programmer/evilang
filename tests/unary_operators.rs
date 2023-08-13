use evilang_lib::ast::expression::Expression::{BinaryExpression, BooleanLiteral, Identifier, IntegerLiteral, UnaryExpression};
use evilang_lib::ast::operator::Operator::{Equals, GreaterThan, LogicalNot, Minus, Multiplication, Plus};
use evilang_lib::interpreter::runtime_values::PrimitiveValue;

use crate::common::{test_expression_and_assignment, TestData, TestRes};

mod common;

#[test]
fn negation() -> TestRes {
	return test_expression_and_assignment("-x;", UnaryExpression {
		operator: Minus,
		argument: Identifier("x".parse().unwrap()).into(),
	});
}

#[test]
fn logical_not() -> TestRes {
	return test_expression_and_assignment("!x;", UnaryExpression {
		operator: LogicalNot,
		argument: Identifier("x".parse().unwrap()).into(),
	});
}

#[test]
fn complex_unary() -> TestRes {
	return test_expression_and_assignment("!+-(!-x * +-y + -13 > +15) == !true;", BinaryExpression {
		operator: Equals,
		left: UnaryExpression {
			operator: LogicalNot,
			argument: UnaryExpression {
				operator: Plus,
				argument: UnaryExpression {
					operator: Minus,
					argument: BinaryExpression {
						operator: GreaterThan,
						left: BinaryExpression {
							operator: Plus,
							left: BinaryExpression {
								operator: Multiplication,
								left: UnaryExpression {
									operator: LogicalNot,
									argument: UnaryExpression {
										operator: Minus,
										argument: Identifier("x".parse().unwrap()).into(),
									}.into(),
								}.into(),
								right: UnaryExpression {
									operator: Plus,
									argument: UnaryExpression {
										operator: Minus,
										argument: Identifier("y".parse().unwrap()).into(),
									}.into(),
								}.into(),
							}.into(),
							right: UnaryExpression {
								operator: Minus,
								argument: IntegerLiteral(13).into(),
							}.into(),
						}.into(),
						right: UnaryExpression {
							operator: Plus,
							argument: IntegerLiteral(15).into(),
						}.into(),
					}.consume_as_parenthesized().into(),
				}.into(),
			}.into(),
		}.into(),
		right: UnaryExpression {
			operator: LogicalNot,
			argument: BooleanLiteral(true).into(),
		}.into(),
	}, );
}

#[test]
fn interpretation_basic() -> TestRes {
	TestData::new(r#"
	!false;
	!true;
	-1;
	+2;
	-(-3);
	+(-4);
	-(+5);
	+(+6);
	10 + -17;
"#.to_string())
		.expect_results(vec![
			// Or
			PrimitiveValue::Boolean(true),
			PrimitiveValue::Boolean(false),
			PrimitiveValue::Integer(-1),
			PrimitiveValue::Integer(2),
			PrimitiveValue::Integer(3),
			PrimitiveValue::Integer(-4),
			PrimitiveValue::Integer(-5),
			PrimitiveValue::Integer(6),
			PrimitiveValue::Integer(-7),
		])
		.check();
}
