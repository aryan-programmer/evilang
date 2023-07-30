use std::collections::HashMap;

use evilang_lib::ast::expression::BoxExpression;
use evilang_lib::ast::expression::Expression::{AssignmentExpression, BinaryExpression, Identifier, IntegerLiteral};
use evilang_lib::ast::operator::Operator::{Assignment, DivisionAssignment, MinusAssignment, ModulusAssignment, Multiplication, MultiplicationAssignment, Plus, PlusAssignment};
use evilang_lib::interpreter::environment::Environment;
use evilang_lib::interpreter::runtime_value::PrimitiveValue;

use crate::common::{ensure_execution_results, identifier_stmt, TestRes};

mod common;

#[test]
fn simple_assignment() -> TestRes {
	let mut env = Environment::new(HashMap::from([
		("x".into(), PrimitiveValue::Integer(-1))
	]), None);
	ensure_execution_results("x;x = 1;x;", vec![
		identifier_stmt("x"),
		AssignmentExpression {
			operator: Assignment,
			left: BoxExpression::from(Identifier("x".to_string())),
			right: BoxExpression::from(IntegerLiteral(1)),
		}.consume_as_statement(),
		identifier_stmt("x"),
	], &mut env, vec![
		PrimitiveValue::Integer(-1),
		PrimitiveValue::Integer(1),
		PrimitiveValue::Integer(1),
	]);
}

#[test]
fn complex_assignment() -> TestRes {
	let mut env = Environment::new(HashMap::from([
		("x".into(), PrimitiveValue::Integer(12))
	]), None);
	ensure_execution_results(r#"
x += 1;
x -= 1;
x *= 10;
x /= 12;
x %= 3;
"#, vec![
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
	], &mut env, vec![
		PrimitiveValue::Integer(13),
		PrimitiveValue::Integer(12),
		PrimitiveValue::Integer(120),
		PrimitiveValue::Integer(10),
		PrimitiveValue::Integer(1),
	]);
}

#[test]
fn chained_assignment() -> TestRes {
	let mut env = Environment::new(HashMap::from([
		("x".into(), PrimitiveValue::Integer(5)),
		("y".into(), PrimitiveValue::Integer(6)),
	]), None);
	ensure_execution_results(r#"
x;
y;
x = y = 1;
x;
y;
"#, vec![
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
	], &mut env, vec![
		PrimitiveValue::Integer(5),
		PrimitiveValue::Integer(6),
		PrimitiveValue::Integer(1),
		PrimitiveValue::Integer(1),
		PrimitiveValue::Integer(1),
	]);
}

#[test]
fn chained_complex_assignment() -> TestRes {
	let mut env = Environment::new(HashMap::from([
		("x".into(), PrimitiveValue::Integer(5)),
		("y".into(), PrimitiveValue::Integer(6)),
		("z".into(), PrimitiveValue::Integer(7)),
	]), None);
	ensure_execution_results(r#"
x;
y;
z;
x = y += z = 2;
x;
y;
z;
"#, vec![
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
	], &mut env, vec![
		PrimitiveValue::Integer(5),
		PrimitiveValue::Integer(6),
		PrimitiveValue::Integer(7),
		PrimitiveValue::Integer(8),
		PrimitiveValue::Integer(8),
		PrimitiveValue::Integer(8),
		PrimitiveValue::Integer(2),
	]);
}

#[test]
fn complex_assignments() -> TestRes {
	let mut env = Environment::new(HashMap::from([
		("x".into(), PrimitiveValue::Integer(5)),
		("y".into(), PrimitiveValue::Integer(6)),
		("z".into(), PrimitiveValue::Integer(7)),
	]), None);
	ensure_execution_results(r#"
x;
y;
z;
x += y = 1+2*(z=1)+4;
x;
y;
z;
"#, vec![
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
							right: BoxExpression::from(AssignmentExpression {
								operator: Assignment,
								left: BoxExpression::from(Identifier("z".to_string())),
								right: BoxExpression::from(IntegerLiteral(1)),
							}),
						}),
					}),
					right: BoxExpression::from(IntegerLiteral(4)),
				}),
			}),
		}.consume_as_statement(),
		identifier_stmt("x"),
		identifier_stmt("y"),
		identifier_stmt("z"),
	], &mut env, vec![
		PrimitiveValue::Integer(5),
		PrimitiveValue::Integer(6),
		PrimitiveValue::Integer(7),
		PrimitiveValue::Integer(12),
		PrimitiveValue::Integer(12),
		PrimitiveValue::Integer(7),
		PrimitiveValue::Integer(1),
	]);
}
