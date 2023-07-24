use evilang_lib::ast::expression::Expression;
use evilang_lib::ast::expression::Expression::{AssignmentExpression, BinaryExpression, BooleanLiteral, Identifier, IntegerLiteral, UnaryExpression};
use evilang_lib::ast::operator::Operator::{LessThanOrEqualTo, Multiplication, MultiplicationAssignment, Plus, PlusAssignment};
use evilang_lib::ast::statement::Statement;
use evilang_lib::ast::structs::VariableDeclaration;
use evilang_lib::ast::statement::Statement::{BlockStatement, DoWhileLoop, EmptyStatement, ExpressionStatement, ForLoop, VariableDeclarations, WhileLoop};

use crate::common::{ensure_program, ensure_program_fails, TestRes};

mod common;

#[test]
fn while_loop() -> TestRes {
	let (initialization, condition, increment) = get_parts();
	ensure_program(r#"
	let i = 1;
	while (i <= 10){
		i += 1;
	}
"#, vec![
		initialization.clone(),
		WhileLoop {
			condition: condition.clone(),
			body: BlockStatement([increment.clone()].into()).into(),
		},
	]);
	ensure_program(r#"
	let i = 1;
	while (i <= 10) i += 1;
"#, vec![
		initialization,
		WhileLoop {
			condition,
			body: increment.into(),
		},
	]);
}

#[test]
fn do_while_loop() -> TestRes {
	ensure_program_fails(r#"
	let i = 1;
	do i += 1;
	while (i <= 10);
"#, None);
	let (initialization, condition, increment) = get_parts();
	ensure_program(r#"
	let i = 1;
	do {
		i += 1;
	} while (i <= 10);
"#, vec![
		initialization,
		DoWhileLoop {
			condition,
			body: BlockStatement([increment].into()).into(),
		},
	]);
}

#[test]
fn for_loop() -> TestRes {
	let (initialization, condition, increment) = get_parts();
	let for_body = ExpressionStatement(
		UnaryExpression {
			operator: Plus,
			argument: Identifier("i".to_string()).into(),
		},
	);
	ensure_program(r#"
	for(let i = 1; i <= 10; i += 1){
		+i;
	}
"#, vec![
		ForLoop {
			initialization: initialization.clone().into(),
			condition: condition.clone(),
			increment: increment.clone().into(),
			body: BlockStatement(vec![
				for_body.clone(),
			]).into(),
		},
	]);
	ensure_program(r#"
	for(let i = 1; i <= 10; i += 1) +i;
"#, vec![
		ForLoop {
			initialization: initialization.into(),
			condition,
			increment: increment.into(),
			body: for_body.into(),
		},
	]);
}

#[test]
fn empty_for_loop() -> TestRes {
	ensure_program(r#"
	for(;;);
"#, vec![
		ForLoop {
			initialization: EmptyStatement.into(),
			condition: BooleanLiteral(true),
			increment: EmptyStatement.into(),
			body: EmptyStatement.into(),
		},
	]);
}

#[test]
fn exotic_for_loop() -> TestRes {
	let (initialization, condition, increment) = get_parts();
	let i = Identifier("i".to_string());
	let j = Identifier("j".to_string());
	let for_body = ExpressionStatement(
		UnaryExpression {
			operator: Plus,
			argument: i.clone().into(),
		},
	);
	ensure_program(r#"
	for({
			let i = 1;
			let j = 12;
			i += j;
		};
		i <= 10;
		{
			i += 1;
			j *= 2;
		}){
		+i;
		i * j;
	}
"#, vec![
		ForLoop {
			initialization: BlockStatement(vec![
				initialization,
				VariableDeclarations(vec![VariableDeclaration {
					identifier: "j".to_string(),
					initializer: Some(IntegerLiteral(12)),
				}]),
				AssignmentExpression {
					operator: PlusAssignment,
					left: i.clone().into(),
					right: j.clone().into(),
				}.consume_as_statement(),
			]).into(),
			condition,
			increment: BlockStatement(vec![
				increment.into(),
				AssignmentExpression {
					operator: MultiplicationAssignment,
					left: j.clone().into(),
					right: IntegerLiteral(2).into(),
				}.consume_as_statement(),
			]).into(),
			body: BlockStatement(vec![
				for_body,
				BinaryExpression {
					operator: Multiplication,
					left: i.clone().into(),
					right: j.clone().into(),
				}.consume_as_statement(),
			]).into(),
		},
	]);
}

fn get_parts() -> (Statement, Expression, Statement) {
	let initialization = VariableDeclarations(vec![VariableDeclaration {
		identifier: "i".to_string(),
		initializer: Some(IntegerLiteral(1)),
	}]);
	let condition = BinaryExpression {
		operator: LessThanOrEqualTo,
		left: Identifier("i".to_string()).into(),
		right: IntegerLiteral(10).into(),
	};
	let increment = ExpressionStatement(
		AssignmentExpression {
			operator: PlusAssignment,
			left: Identifier("i".to_string()).into(),
			right: IntegerLiteral(1).into(),
		},
	);
	(initialization, condition, increment)
}
