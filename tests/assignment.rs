use evilang_lib::ast::BoxStatement;
use evilang_lib::ast::Operator::{Assignment, Multiplication, Plus, PlusAssignment};
use evilang_lib::ast::Statement::{AssignmentExpression, BinaryExpression, Identifier, IntegerLiteral};

use crate::common::{ensure_program, TestRes};

mod common;

#[test]
fn simple_assignment() -> TestRes {
	ensure_program("x = 1;", vec![AssignmentExpression {
		operator: Assignment,
		left: BoxStatement::from(Identifier("x".to_string())),
		right: BoxStatement::from(IntegerLiteral(1)),
	}]);
}

#[test]
fn chained_assignment() -> TestRes {
	ensure_program("x = y = 1;", vec![AssignmentExpression {
		operator: Assignment,
		left: BoxStatement::from(Identifier("x".to_string())),
		right: BoxStatement::from(AssignmentExpression {
			operator: Assignment,
			left: BoxStatement::from(Identifier("y".to_string())),
			right: BoxStatement::from(IntegerLiteral(1)),
		}),
	}]);
}

#[test]
fn chained_complex_assignment() -> TestRes {
	ensure_program("x = y += z = 1;", vec![AssignmentExpression {
		operator: Assignment,
		left: BoxStatement::from(Identifier("x".to_string())),
		right: BoxStatement::from(AssignmentExpression {
			operator: PlusAssignment,
			left: BoxStatement::from(Identifier("y".to_string())),
			right: BoxStatement::from(AssignmentExpression {
				operator: Assignment,
				left: BoxStatement::from(Identifier("z".to_string())),
				right: BoxStatement::from(IntegerLiteral(1)),
			}),
		}),
	}]);
}

#[test]
fn complex_assignments() -> TestRes {
	ensure_program("x += y = 1+2*(z=1)+4;", vec![
		AssignmentExpression {
			operator: PlusAssignment,
			left: BoxStatement::from(Identifier("x".to_string())),
			right: BoxStatement::from(AssignmentExpression {
				operator: Assignment,
				left: BoxStatement::from(Identifier("y".to_string())),
				right: BoxStatement::from(BinaryExpression {
					operator: Plus,
					left: BoxStatement::from(BinaryExpression {
						operator: Plus,
						left: BoxStatement::from(IntegerLiteral(1)),
						right: BoxStatement::from(BinaryExpression {
							operator: Multiplication,
							left: BoxStatement::from(IntegerLiteral(2)),
							right: BoxStatement::from(AssignmentExpression {
								operator: Assignment,
								left: BoxStatement::from(Identifier("z".to_string())),
								right: BoxStatement::from(IntegerLiteral(1)),
							}),
						}),
					}),
					right: BoxStatement::from(IntegerLiteral(4)),
				}),
			}),
		},
	]);
}
