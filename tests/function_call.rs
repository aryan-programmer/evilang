use evilang_lib::ast::expression::Expression::{AssignmentExpression, BinaryExpression, FunctionCall, Identifier, IntegerLiteral, MemberAccess, StringLiteral};
use evilang_lib::ast::expression::MemberIndexer::{PropertyName, SubscriptExpression};
use evilang_lib::ast::operator::Operator::{Assignment, ModulusAssignment, Plus};

use crate::common::{ensure_program, test_expression_and_assignment, TestRes};

mod common;

#[test]
fn function_call() -> TestRes {
	test_expression_and_assignment(r#"foo(x,y);"#, FunctionCall {
		function: Identifier("foo".to_string()).into(),
		arguments: vec![
			Identifier("x".to_string()),
			Identifier("y".to_string()),
		],
	});
}

#[test]
fn chained_function_call() -> TestRes {
	test_expression_and_assignment(r#"foo()(x,y);"#, FunctionCall {
		function: FunctionCall {
			function: Identifier("foo".to_string()).into(),
			arguments: vec![],
		}.into(),
		arguments: vec![
			Identifier("x".to_string()),
			Identifier("y".to_string()),
		],
	});
}

#[test]
fn console_log() -> TestRes {
	ensure_program(r#"console.log("values");"#, vec![
		FunctionCall {
			function: MemberAccess {
				object: Identifier("console".to_string()).into(),
				member: PropertyName("log".to_string()),
			}.into(),
			arguments: vec![
				StringLiteral("values".to_string()),
			],
		}.into(),
	]);
}

#[test]
fn member_complex_assignment() -> TestRes {
	test_expression_and_assignment(r#"a.b["c" + y](p1, $).d %= 1+$.left($.right=1)+4;"#, AssignmentExpression {
		operator: ModulusAssignment,
		left: MemberAccess {
			object: FunctionCall {
				function: MemberAccess {
					object: MemberAccess {
						object: Identifier("a".to_string()).into(),
						member: PropertyName("b".to_string()),
					}.into(),
					member: SubscriptExpression(
						BinaryExpression {
							operator: Plus,
							left: StringLiteral("c".to_string()).into(),
							right: Identifier("y".to_string()).into(),
						}.into()
					),
				}.into(),
				arguments: vec![
					Identifier("p1".to_string()),
					Identifier("$".to_string()),
				],
			}.into(),
			member: PropertyName("d".to_string()),
		}.into(),
		right: BinaryExpression {
			operator: Plus,
			left: BinaryExpression {
				operator: Plus,
				left: IntegerLiteral(1).into(),
				right: FunctionCall {
					function: MemberAccess {
						object: Identifier("$".to_string()).into(),
						member: PropertyName("left".to_string())
					}.into(),
					arguments: vec![
						AssignmentExpression {
							operator: Assignment,
							left: MemberAccess {
								object: Identifier("$".to_string()).into(),
								member: PropertyName("right".to_string())
							}.into(),
							right: IntegerLiteral(1).into(),
						}
					],
				}.into(),
			}.into(),
			right: IntegerLiteral(4).into(),
		}.into(),
	});
}

