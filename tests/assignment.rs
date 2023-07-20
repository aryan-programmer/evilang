use evilang_lib::ast::expression::BoxExpression;
use evilang_lib::ast::expression::Expression::{AssignmentExpression, BinaryExpression, Identifier, IntegerLiteral};
use evilang_lib::ast::operator::Operator::{Assignment, Multiplication, Plus, PlusAssignment};

use crate::common::{ensure_program, TestRes};

mod common;

#[test]
fn simple_assignment() -> TestRes {
	ensure_program("x = 1;", vec![AssignmentExpression {
		operator: Assignment,
		left: BoxExpression::from(Identifier("x".to_string())),
		right: BoxExpression::from(IntegerLiteral(1)),
	}.consume_as_statement()]);
}

#[test]
fn chained_assignment() -> TestRes {
	ensure_program("x = y = 1;", vec![AssignmentExpression {
		operator: Assignment,
		left: BoxExpression::from(Identifier("x".to_string())),
		right: BoxExpression::from(AssignmentExpression {
			operator: Assignment,
			left: BoxExpression::from(Identifier("y".to_string())),
			right: BoxExpression::from(IntegerLiteral(1)),
		}),
	}.consume_as_statement()]);
}

#[test]
fn chained_complex_assignment() -> TestRes {
	ensure_program("x = y += z = 1;", vec![AssignmentExpression {
		operator: Assignment,
		left: BoxExpression::from(Identifier("x".to_string())),
		right: BoxExpression::from(AssignmentExpression {
			operator: PlusAssignment,
			left: BoxExpression::from(Identifier("y".to_string())),
			right: BoxExpression::from(AssignmentExpression {
				operator: Assignment,
				left: BoxExpression::from(Identifier("z".to_string())),
				right: BoxExpression::from(IntegerLiteral(1)),
			}),
		}),
	}.consume_as_statement()]);
}

#[test]
fn complex_assignments() -> TestRes {
	ensure_program("x += y = 1+2*(z=1)+4;", vec![AssignmentExpression {
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
	}.consume_as_statement()]);
}
