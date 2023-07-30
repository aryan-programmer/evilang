use evilang_lib::ast::expression::BoxExpression;
use evilang_lib::ast::expression::Expression::{AssignmentExpression, BinaryExpression, Identifier, IntegerLiteral};
use evilang_lib::ast::operator::Operator::{Plus, PlusAssignment};
use evilang_lib::ast::statement::Statement::VariableDeclarations;
use evilang_lib::ast::structs::VariableDeclaration;
use evilang_lib::interpreter::runtime_value::PrimitiveValue;

use crate::common::{ensure_program_statement_results, TestRes};

mod common;

#[test]
fn basic_declaration() -> TestRes {
	ensure_program_statement_results("let x = 1 + 2;x;", vec![VariableDeclarations(Vec::from([
		VariableDeclaration {
			identifier: "x".parse().unwrap(),
			initializer: Some(BinaryExpression {
				operator: Plus,
				left: BoxExpression::from(IntegerLiteral(1)),
				right: BoxExpression::from(IntegerLiteral(2)),
			}),
		},
	])), Identifier("x".into()).consume_as_statement()], vec![
		PrimitiveValue::Null,
		PrimitiveValue::Integer(3),
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
		])),
		Identifier("$foo".into()).consume_as_statement(),
		Identifier("bar1".into()).consume_as_statement(),
		Identifier("baz".into()).consume_as_statement(),
	], vec![
		PrimitiveValue::Null,
		PrimitiveValue::Integer(7),
		PrimitiveValue::Null,
		PrimitiveValue::Integer(7),
	]);
}
