use evilang_lib::ast::expression::BoxExpression;
use evilang_lib::ast::expression::Expression::{AssignmentExpression, BinaryExpression, Identifier, IntegerLiteral};
use evilang_lib::ast::operator::Operator::{Plus, PlusAssignment};
use evilang_lib::ast::statement::Statement::VariableDeclarations;
use evilang_lib::ast::structs::VariableDeclaration;

use crate::common::{ensure_program, TestRes};

mod common;

#[test]
fn basic_declaration() -> TestRes {
	ensure_program("let x = 1 + 2;", vec![VariableDeclarations(Vec::from([
		VariableDeclaration {
			identifier: "x".parse().unwrap(),
			initializer: Some(BinaryExpression {
				operator: Plus,
				left: BoxExpression::from(IntegerLiteral(1)),
				right: BoxExpression::from(IntegerLiteral(2)),
			}),
		},
	]))]);
}

#[test]
fn multiple_declarations() -> TestRes {
	ensure_program("let $foo = 1 + 2, bar1, baz = $foo += 4;", vec![VariableDeclarations(Vec::from([
		VariableDeclaration {
			identifier: "$foo".parse().unwrap(),
			initializer: Some(BinaryExpression {
				operator: Plus,
				left: BoxExpression::from(IntegerLiteral(1)),
				right: BoxExpression::from(IntegerLiteral(2)),
			}),
		},
		VariableDeclaration {
			identifier: "bar1".parse().unwrap(),
			initializer: None,
		},
		VariableDeclaration {
			identifier: "baz".parse().unwrap(),
			initializer: Some(AssignmentExpression {
				operator: PlusAssignment,
				left: BoxExpression::from(Identifier("$foo".parse().unwrap())),
				right: BoxExpression::from(IntegerLiteral(4)),
			}),
		},
	]))]);
}
