use evilang_lib::ast::expression::{Expression, Expression::{AssignmentExpression, BinaryExpression, Identifier, MemberAccess, StringLiteral}};
use evilang_lib::ast::expression::Expression::DottedIdentifiers;
use evilang_lib::ast::expression::MemberIndexer::{PropertyName, SubscriptExpression};
use evilang_lib::ast::operator::Operator::{Assignment, ModulusAssignment, Plus};

use crate::common::{ensure_program, test_expression_and_assignment, TestRes};

mod common;

#[test]
fn function_call() -> TestRes {
	test_expression_and_assignment(r#"foo(x,y);"#, Expression::function_call(
		Identifier("foo".to_string()).into(),
		vec![
			Identifier("x".to_string()),
			Identifier("y".to_string()),
		],
	));
}

#[test]
fn chained_function_call() -> TestRes {
	test_expression_and_assignment(r#"foo()(x,y);"#, Expression::function_call(
		Expression::function_call(
			Identifier("foo".to_string()).into(),
			vec![],
		).into(),
		vec![
			Identifier("x".to_string()),
			Identifier("y".to_string()),
		],
	));
}

#[test]
fn console_log() -> TestRes {
	ensure_program(r#"console.log("values");"#, vec![
		Expression::function_call(
			DottedIdentifiers([
				"console".to_string(),
				"log".to_string(),
			].into()).into(),
			vec![
				StringLiteral("values".to_string()),
			],
		).into(),
	]);
}

#[test]
fn member_complex_assignment() -> TestRes {
	test_expression_and_assignment(r#"a.b["c" + y](p1, $).d %= 1+$.left($.right=1)+4;"#, AssignmentExpression {
		operator: ModulusAssignment,
		left: MemberAccess {
			object: Expression::function_call(
				MemberAccess {
					object: DottedIdentifiers([
						"a".to_string(),
						"b".to_string(),
					].into()).into(),
					member: SubscriptExpression(
						BinaryExpression {
							operator: Plus,
							left: StringLiteral("c".to_string()).into(),
							right: Identifier("y".to_string()).into(),
						}.into()
					),
				}.into(),
				vec![
					Identifier("p1".to_string()),
					Identifier("$".to_string()),
				],
			).into(),
			member: PropertyName("d".to_string()),
		}.into(),
		right: BinaryExpression {
			operator: Plus,
			left: BinaryExpression {
				operator: Plus,
				left: Expression::integer_literal(1).into(),
				right: Expression::function_call(
					DottedIdentifiers([
						"$".to_string(),
						"left".to_string(),
					].into()).into(),
					vec![
						AssignmentExpression {
							operator: Assignment,
							left: DottedIdentifiers([
								"$".to_string(),
								"right".to_string(),
							].into()).into(),
							right: Expression::integer_literal(1).into(),
						}
					],
				).into(),
			}.into(),
			right: Expression::integer_literal(4).into(),
		}.into(),
	});
}
