use evilang_lib::ast::expression::Expression;
use evilang_lib::ast::expression::Expression::{AssignmentExpression, BinaryExpression, DottedIdentifiers, Identifier, MemberAccess, StringLiteral};
use evilang_lib::ast::expression::MemberIndexer::{PropertyName, SubscriptExpression};
use evilang_lib::ast::operator::Operator::{Assignment, ModulusAssignment, Multiplication, Plus};

use crate::common::{test_expression_and_assignment, TestRes};

mod common;

#[test]
fn member_access() -> TestRes {
	test_expression_and_assignment(r#"a.b["c"].d;"#, MemberAccess {
		object: MemberAccess {
			object: DottedIdentifiers([
				"a".to_string(),
				"b".to_string(),
			].into()).into(),
			member: SubscriptExpression(StringLiteral("c".to_string()).into()),
		}.into(),
		member: PropertyName("d".to_string()),
	});
}

#[test]
fn member_complex_assignment() -> TestRes {
	test_expression_and_assignment(r#"a.b["c" + y].d %= 1+$.left*($.right=1)+4;"#, AssignmentExpression {
		operator: ModulusAssignment,
		left: MemberAccess {
			object: MemberAccess {
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
			member: PropertyName("d".to_string()),
		}.into(),
		right: BinaryExpression {
			operator: Plus,
			left: BinaryExpression {
				operator: Plus,
				left: Expression::integer_literal(1).into(),
				right: BinaryExpression {
					operator: Multiplication,
					left: DottedIdentifiers([
						"$".to_string(),
						"left".to_string(),
					].into()).into(),
					right: AssignmentExpression {
						operator: Assignment,
						left: DottedIdentifiers([
							"$".to_string(),
							"right".to_string(),
						].into()).into(),
						right: Expression::integer_literal(1).into(),
					}.consume_as_parenthesized().into(),
				}.into(),
			}.into(),
			right: Expression::integer_literal(4).into(),
		}.into(),
	});
}
