use evilang_lib::ast::expression::Expression;
use evilang_lib::ast::expression::Expression::{AssignmentExpression, BinaryExpression, BooleanLiteral, Identifier, IntegerLiteral, StringLiteral};
use evilang_lib::ast::operator::Operator::{LessThanOrEqualTo, MultiplicationAssignment, PlusAssignment};
use evilang_lib::ast::statement::Statement;
use evilang_lib::ast::statement::Statement::{BlockStatement, DoWhileLoop, EmptyStatement, ExpressionStatement, ForLoop, VariableDeclarations, WhileLoop};
use evilang_lib::ast::structs::VariableDeclaration;
use evilang_lib::interpreter::runtime_value::PrimitiveValue;
use evilang_lib::interpreter::runtime_value::PrimitiveValue::Integer;

use crate::common::{ensure_program, ensure_program_fails, ensure_program_statement_results, identifier_stmt, push_res_stack_stmt, TestData, TestRes};

mod common;

#[test]
fn while_loop() -> TestRes {
	let (initialization, condition, increment) = get_parts();
	TestData::new(r#"
	let i = 1;
	while (i <= 10){
		push_res_stack(i);
		i += 1;
	}
"#.to_string())
		.expect_statements(vec![
			initialization.clone(),
			WhileLoop {
				condition: condition.clone(),
				body: BlockStatement([
					push_res_stack_stmt(Identifier("i".into())),
					increment.clone()
				].into()).into(),
			},
		])
		.expect_stack(primitive_values_integer_range())
		.check();
	ensure_program_statement_results(r#"
	let i = 1;
	while (i <= 10) i += 1;
	i;
"#, vec![
		initialization,
		WhileLoop {
			condition,
			body: increment.into(),
		},
		identifier_stmt("i"),
	], vec![
		PrimitiveValue::Null,
		PrimitiveValue::Null,
		PrimitiveValue::Integer(11),
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
	TestData::new(r#"
	let i = 1;
	do {
		push_res_stack(i);
		i += 1;
	} while (i <= 10);
"#.to_string())
		.expect_statements(vec![
			initialization,
			DoWhileLoop {
				condition,
				body: BlockStatement([
					push_res_stack_stmt(Identifier("i".into())),
					increment,
				].into()).into(),
			},
		])
		.expect_stack(primitive_values_integer_range())
		.check();
	TestData::new(r#"
	do {
		push_res_stack("atleast_once");
	} while (false);
"#.to_string())
		.expect_statements(vec![
			DoWhileLoop {
				condition: BooleanLiteral(false),
				body: BlockStatement([
					push_res_stack_stmt(StringLiteral("atleast_once".to_string())),
				].into()).into(),
			},
		])
		.expect_stack(vec![PrimitiveValue::String("atleast_once".to_string())])
		.check();
}

#[test]
fn for_loop() -> TestRes {
	let (initialization, condition, increment) = get_parts();
	let for_body = push_res_stack_stmt(Identifier("i".to_string()));
	TestData::new(r#"
	for(let i = 1; i <= 10; i += 1){
		push_res_stack(i);
	}
"#.to_string())
		.expect_statements(vec![
			ForLoop {
				initialization: initialization.clone().into(),
				condition: condition.clone(),
				increment: increment.clone().into(),
				body: BlockStatement(vec![for_body.clone()]).into(),
			},
		])
		.expect_stack(primitive_values_integer_range())
		.check();
	TestData::new(r#"
	for(let i = 1; i <= 10; i += 1)
		push_res_stack(i);
"#.to_string())
		.expect_statements(vec![
			ForLoop {
				initialization: initialization.into(),
				condition,
				increment: increment.into(),
				body: for_body.into(),
			},
		])
		.expect_stack(primitive_values_integer_range())
		.check();
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
	TestData::new(r#"
	for({
			let i = 1;
			let j = 2;
			i += j;
		};
		i <= 10;
		{
			i += 1;
			j *= 2;
		}){
		push_res_stack(i);
		push_res_stack(j);
	}
"#.to_string())
		.expect_statements(vec![
			ForLoop {
				initialization: BlockStatement(vec![
					initialization,
					VariableDeclarations(vec![VariableDeclaration {
						identifier: "j".to_string(),
						initializer: Some(IntegerLiteral(2)),
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
					push_res_stack_stmt(Identifier("i".to_string())),
					push_res_stack_stmt(Identifier("j".to_string())),
				]).into(),
			},
		])
		.expect_stack(vec![
			/*i=*/Integer(3), /*j=*/Integer(2),
			/*i=*/Integer(4), /*j=*/Integer(4),
			/*i=*/Integer(5), /*j=*/Integer(8),
			/*i=*/Integer(6), /*j=*/Integer(16),
			/*i=*/Integer(7), /*j=*/Integer(32),
			/*i=*/Integer(8), /*j=*/Integer(64),
			/*i=*/Integer(9), /*j=*/Integer(128),
			/*i=*/Integer(10), /*j=*/Integer(256),
		])
		.check();
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

fn primitive_values_integer_range() -> Vec<PrimitiveValue> {
	(1..11).map(|v| PrimitiveValue::Integer(v)).collect()
}
