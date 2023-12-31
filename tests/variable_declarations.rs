use evilang_lib::ast::expression::{BoxExpression, Expression};
use evilang_lib::ast::expression::Expression::{AssignmentExpression, BinaryExpression, Identifier};
use evilang_lib::ast::operator::Operator::{Plus, PlusAssignment};
use evilang_lib::ast::statement::Statement::VariableDeclarations;
use evilang_lib::ast::structs::VariableDeclaration;
use evilang_lib::interpreter::runtime_values::PrimitiveValue;

use crate::common::{ensure_program_statement_results, identifier_stmt, TestRes};

mod common;

#[test]
fn basic_declaration() -> TestRes {
	ensure_program_statement_results("let x = 1 + 2;x;", vec![VariableDeclarations(Vec::from([
		VariableDeclaration {
			identifier: "x".parse().unwrap(),
			initializer: Some(BinaryExpression {
				operator: Plus,
				left: BoxExpression::from(Expression::integer_literal(1)),
				right: BoxExpression::from(Expression::integer_literal(2)),
			}),
		},
	])), identifier_stmt("x")], vec![
		PrimitiveValue::Null,
		PrimitiveValue::integer(3),
	]);
}

#[test]
fn multiple_declarations() -> TestRes {
	ensure_program_statement_results(r#"
let $foo = 1 + 2, bar1, baz = $foo += 4;
$foo;
bar1;
baz;
"#, vec![
		VariableDeclarations(Vec::from([
			VariableDeclaration {
				identifier: "$foo".parse().unwrap(),
				initializer: Some(BinaryExpression {
					operator: Plus,
					left: BoxExpression::from(Expression::integer_literal(1)),
					right: BoxExpression::from(Expression::integer_literal(2)),
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
					right: BoxExpression::from(Expression::integer_literal(4)),
				}),
			},
		])),
		identifier_stmt("$foo"),
		identifier_stmt("bar1"),
		identifier_stmt("baz"),
	], vec![
		PrimitiveValue::Null,
		PrimitiveValue::integer(7),
		PrimitiveValue::Null,
		PrimitiveValue::integer(7),
	]);
}
