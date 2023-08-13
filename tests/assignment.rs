use std::collections::HashMap;

use evilang_lib::ast::expression::BoxExpression;
use evilang_lib::ast::expression::Expression::{AssignmentExpression, BinaryExpression, Identifier, IntegerLiteral};
use evilang_lib::ast::operator::Operator::{Assignment, DivisionAssignment, MinusAssignment, ModulusAssignment, Multiplication, MultiplicationAssignment, Plus, PlusAssignment};
use evilang_lib::interpreter::environment::Environment;
use evilang_lib::interpreter::runtime_values::PrimitiveValue;

use crate::common::{identifier_stmt, TestData, TestRes};

mod common;

#[test]
fn simple_assignment() -> TestRes {
	let mut env = Environment::new_from_primitives(HashMap::from([
		("x".into(), PrimitiveValue::Integer(-1))
	]));
	TestData::new("x;x = 1;x;".to_string()).expect_statements_and_results(vec![
		identifier_stmt("x"),
		AssignmentExpression {
			operator: Assignment,
			left: BoxExpression::from(Identifier("x".to_string())),
			right: BoxExpression::from(IntegerLiteral(1)),
		}.consume_as_statement(),
		identifier_stmt("x"),
	], vec![
		PrimitiveValue::Integer(-1),
		PrimitiveValue::Integer(1),
		PrimitiveValue::Integer(1),
	]).check_with_env(&mut env);
}

#[test]
fn complex_assignment() -> TestRes {
	let mut env = Environment::new_from_primitives(HashMap::from([
		("x".into(), PrimitiveValue::Integer(12))
	]));
	{
		TestData::new(r#"
	x += 1;
	x -= 1;
	x *= 10;
	x /= 12;
	x %= 3;
	"#.to_string()).expect_statements_and_results(vec![
			AssignmentExpression {
				operator: PlusAssignment,
				left: BoxExpression::from(Identifier("x".to_string())),
				right: BoxExpression::from(IntegerLiteral(1)),
			}.consume_as_statement(),
			AssignmentExpression {
				operator: MinusAssignment,
				left: BoxExpression::from(Identifier("x".to_string())),
				right: BoxExpression::from(IntegerLiteral(1)),
			}.consume_as_statement(),
			AssignmentExpression {
				operator: MultiplicationAssignment,
				left: BoxExpression::from(Identifier("x".to_string())),
				right: BoxExpression::from(IntegerLiteral(10)),
			}.consume_as_statement(),
			AssignmentExpression {
				operator: DivisionAssignment,
				left: BoxExpression::from(Identifier("x".to_string())),
				right: BoxExpression::from(IntegerLiteral(12)),
			}.consume_as_statement(),
			AssignmentExpression {
				operator: ModulusAssignment,
				left: BoxExpression::from(Identifier("x".to_string())),
				right: BoxExpression::from(IntegerLiteral(3)),
			}.consume_as_statement(),
		], vec![
			PrimitiveValue::Integer(13),
			PrimitiveValue::Integer(12),
			PrimitiveValue::Integer(120),
			PrimitiveValue::Integer(10),
			PrimitiveValue::Integer(1),
		])
	}.check_with_env(&mut env)
}

#[test]
fn chained_assignment() -> TestRes {
	let mut env = Environment::new_from_primitives(HashMap::from([
		("x".into(), PrimitiveValue::Integer(5)),
		("y".into(), PrimitiveValue::Integer(6)),
	]));
	let mut test = {
		TestData::new(r#"
	x;
	y;
	x = y = 1;
	x;
	y;
	"#.to_string()).expect_statements_and_results(vec![
			identifier_stmt("x"),
			identifier_stmt("y"),
			AssignmentExpression {
				operator: Assignment,
				left: BoxExpression::from(Identifier("x".to_string())),
				right: BoxExpression::from(AssignmentExpression {
					operator: Assignment,
					left: BoxExpression::from(Identifier("y".to_string())),
					right: BoxExpression::from(IntegerLiteral(1)),
				}),
			}.consume_as_statement(),
			identifier_stmt("x"),
			identifier_stmt("y"),
		], vec![
			PrimitiveValue::Integer(5),
			PrimitiveValue::Integer(6),
			PrimitiveValue::Integer(1),
			PrimitiveValue::Integer(1),
			PrimitiveValue::Integer(1),
		])
	};
	return test.check_with_env(&mut env);
}

#[test]
fn chained_complex_assignment() -> TestRes {
	let mut env = Environment::new_from_primitives(HashMap::from([
		("x".into(), PrimitiveValue::Integer(5)),
		("y".into(), PrimitiveValue::Integer(6)),
		("z".into(), PrimitiveValue::Integer(7)),
	]));
	let mut test = {
		TestData::new(r#"
	x;
	y;
	z;
	x = y += z = 2;
	x;
	y;
	z;
	"#.to_string()).expect_statements_and_results(vec![
			identifier_stmt("x"),
			identifier_stmt("y"),
			identifier_stmt("z"),
			AssignmentExpression {
				operator: Assignment,
				left: BoxExpression::from(Identifier("x".to_string())),
				right: BoxExpression::from(AssignmentExpression {
					operator: PlusAssignment,
					left: BoxExpression::from(Identifier("y".to_string())),
					right: BoxExpression::from(AssignmentExpression {
						operator: Assignment,
						left: BoxExpression::from(Identifier("z".to_string())),
						right: BoxExpression::from(IntegerLiteral(2)),
					}),
				}),
			}.consume_as_statement(),
			identifier_stmt("x"),
			identifier_stmt("y"),
			identifier_stmt("z"),
		], vec![
			PrimitiveValue::Integer(5),
			PrimitiveValue::Integer(6),
			PrimitiveValue::Integer(7),
			PrimitiveValue::Integer(8),
			PrimitiveValue::Integer(8),
			PrimitiveValue::Integer(8),
			PrimitiveValue::Integer(2),
		])
	};
	return test.check_with_env(&mut env);
}

#[test]
fn complex_assignments() -> TestRes {
	let mut env = Environment::new_from_primitives(HashMap::from([
		("x".into(), PrimitiveValue::Integer(5)),
		("y".into(), PrimitiveValue::Integer(6)),
		("z".into(), PrimitiveValue::Integer(7)),
	]));
	let mut test = {
		TestData::new(r#"
	x;
	y;
	z;
	x += y = 1+2*(z=1)+4;
	x;
	y;
	z;
	"#.to_string()).expect_statements_and_results(vec![
			identifier_stmt("x"),
			identifier_stmt("y"),
			identifier_stmt("z"),
			AssignmentExpression {
				operator: PlusAssignment,
				left: BoxExpression::from(Identifier("x".to_string())),
				right: BoxExpression::from(AssignmentExpression {
					operator: Assignment,
					left: BoxExpression::from(Identifier("y".to_string())),
					right: BoxExpression::from(BinaryExpression {
						operator: Plus,
						left: BoxExpression::from(BinaryExpression {
							operator: Plus,
							left: BoxExpression::from(IntegerLiteral(1)),
							right: BoxExpression::from(BinaryExpression {
								operator: Multiplication,
								left: BoxExpression::from(IntegerLiteral(2)),
								right: AssignmentExpression {
									operator: Assignment,
									left: BoxExpression::from(Identifier("z".to_string())),
									right: BoxExpression::from(IntegerLiteral(1)),
								}.consume_as_parenthesized().into(),
							}),
						}),
						right: BoxExpression::from(IntegerLiteral(4)),
					}),
				}),
			}.consume_as_statement(),
			identifier_stmt("x"),
			identifier_stmt("y"),
			identifier_stmt("z"),
		], vec![
			PrimitiveValue::Integer(5),
			PrimitiveValue::Integer(6),
			PrimitiveValue::Integer(7),
			PrimitiveValue::Integer(12),
			PrimitiveValue::Integer(12),
			PrimitiveValue::Integer(7),
			PrimitiveValue::Integer(1),
		])
	};
	return test.check_with_env(&mut env);
}
