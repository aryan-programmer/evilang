use std::collections::HashMap;

use evilang_lib::ast::expression::{BoxExpression, Expression};
use evilang_lib::ast::expression::Expression::{AssignmentExpression, BinaryExpression, Identifier};
use evilang_lib::ast::operator::Operator::{Assignment, DivisionAssignment, MinusAssignment, ModulusAssignment, Multiplication, MultiplicationAssignment, Plus, PlusAssignment};
use evilang_lib::interpreter::environment::Environment;
use evilang_lib::interpreter::environment::resolver::DefaultResolver;
use evilang_lib::interpreter::runtime_values::PrimitiveValue;

use crate::common::{identifier_stmt, TestData, TestRes};

mod common;

#[test]
fn simple_assignment() -> TestRes {
	let mut env = Environment::new_from_primitives(
		HashMap::from([
			("x".into(), PrimitiveValue::integer(-1))
		]),
		DefaultResolver::new_box());
	TestData::new("x;x = 1;x;".to_string()).expect_statements_and_results(vec![
		identifier_stmt("x"),
		AssignmentExpression {
			operator: Assignment,
			left: BoxExpression::from(Identifier("x".to_string())),
			right: BoxExpression::from(Expression::integer_literal(1)),
		}.consume_as_statement(),
		identifier_stmt("x"),
	], vec![
		PrimitiveValue::integer(-1),
		PrimitiveValue::integer(1),
		PrimitiveValue::integer(1),
	]).check_with_env(&mut env);
}

#[test]
fn complex_assignment() -> TestRes {
	let mut env = Environment::new_from_primitives(
		HashMap::from([
			("x".into(), PrimitiveValue::integer(12))
		]),
		DefaultResolver::new_box());
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
				right: BoxExpression::from(Expression::integer_literal(1)),
			}.consume_as_statement(),
			AssignmentExpression {
				operator: MinusAssignment,
				left: BoxExpression::from(Identifier("x".to_string())),
				right: BoxExpression::from(Expression::integer_literal(1)),
			}.consume_as_statement(),
			AssignmentExpression {
				operator: MultiplicationAssignment,
				left: BoxExpression::from(Identifier("x".to_string())),
				right: BoxExpression::from(Expression::integer_literal(10)),
			}.consume_as_statement(),
			AssignmentExpression {
				operator: DivisionAssignment,
				left: BoxExpression::from(Identifier("x".to_string())),
				right: BoxExpression::from(Expression::integer_literal(12)),
			}.consume_as_statement(),
			AssignmentExpression {
				operator: ModulusAssignment,
				left: BoxExpression::from(Identifier("x".to_string())),
				right: BoxExpression::from(Expression::integer_literal(3)),
			}.consume_as_statement(),
		], vec![
			PrimitiveValue::integer(13),
			PrimitiveValue::integer(12),
			PrimitiveValue::integer(120),
			PrimitiveValue::integer(10),
			PrimitiveValue::integer(1),
		])
	}.check_with_env(&mut env)
}

#[test]
fn chained_assignment() -> TestRes {
	let mut env = Environment::new_from_primitives(
		HashMap::from([
			("x".into(), PrimitiveValue::integer(5)),
			("y".into(), PrimitiveValue::integer(6)),
		]),
		DefaultResolver::new_box());
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
					right: BoxExpression::from(Expression::integer_literal(1)),
				}),
			}.consume_as_statement(),
			identifier_stmt("x"),
			identifier_stmt("y"),
		], vec![
			PrimitiveValue::integer(5),
			PrimitiveValue::integer(6),
			PrimitiveValue::integer(1),
			PrimitiveValue::integer(1),
			PrimitiveValue::integer(1),
		])
	};
	return test.check_with_env(&mut env);
}

#[test]
fn chained_complex_assignment() -> TestRes {
	let mut env = Environment::new_from_primitives(
		HashMap::from([
			("x".into(), PrimitiveValue::integer(5)),
			("y".into(), PrimitiveValue::integer(6)),
			("z".into(), PrimitiveValue::integer(7)),
		]),
		DefaultResolver::new_box());
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
						right: BoxExpression::from(Expression::integer_literal(2)),
					}),
				}),
			}.consume_as_statement(),
			identifier_stmt("x"),
			identifier_stmt("y"),
			identifier_stmt("z"),
		], vec![
			PrimitiveValue::integer(5),
			PrimitiveValue::integer(6),
			PrimitiveValue::integer(7),
			PrimitiveValue::integer(8),
			PrimitiveValue::integer(8),
			PrimitiveValue::integer(8),
			PrimitiveValue::integer(2),
		])
	};
	return test.check_with_env(&mut env);
}

#[test]
fn complex_assignments() -> TestRes {
	let mut env = Environment::new_from_primitives(
		HashMap::from([
			("x".into(), PrimitiveValue::integer(5)),
			("y".into(), PrimitiveValue::integer(6)),
			("z".into(), PrimitiveValue::integer(7)),
		]),
		DefaultResolver::new_box());
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
							left: BoxExpression::from(Expression::integer_literal(1)),
							right: BoxExpression::from(BinaryExpression {
								operator: Multiplication,
								left: BoxExpression::from(Expression::integer_literal(2)),
								right: AssignmentExpression {
									operator: Assignment,
									left: BoxExpression::from(Identifier("z".to_string())),
									right: BoxExpression::from(Expression::integer_literal(1)),
								}.consume_as_parenthesized().into(),
							}),
						}),
						right: BoxExpression::from(Expression::integer_literal(4)),
					}),
				}),
			}.consume_as_statement(),
			identifier_stmt("x"),
			identifier_stmt("y"),
			identifier_stmt("z"),
		], vec![
			PrimitiveValue::integer(5),
			PrimitiveValue::integer(6),
			PrimitiveValue::integer(7),
			PrimitiveValue::integer(12),
			PrimitiveValue::integer(12),
			PrimitiveValue::integer(7),
			PrimitiveValue::integer(1),
		])
	};
	return test.check_with_env(&mut env);
}
