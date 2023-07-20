use evilang_lib::ast::expression::Expression::{AssignmentExpression, BinaryExpression, Identifier, IntegerLiteral};
use evilang_lib::ast::operator::Operator::{Assignment, DivisionAssignment, GreaterThan, GreaterThanOrEqualTo, LessThan, LessThanOrEqualTo, MinusAssignment, ModulusAssignment, MultiplicationAssignment};
use evilang_lib::ast::statement::Statement::{BlockStatement, ExpressionStatement, IfStatement};

use crate::common::{ensure_program, TestRes};

mod common;

#[test]
fn if_branch() -> TestRes {
	ensure_program(r#"
	if (x >= 32) {
		stuff -= 1;
	}
"#, vec![
		IfStatement {
			condition: BinaryExpression {
				operator: GreaterThanOrEqualTo,
				left: Identifier("x".parse().unwrap()).into(),
				right: IntegerLiteral(32).into(),
			},
			if_branch: BlockStatement([
				ExpressionStatement(
					AssignmentExpression {
						operator: MinusAssignment,
						left: Identifier("stuff".parse().unwrap()).into(),
						right: IntegerLiteral(1).into(),
					},
				)
			].into()).into(),
			else_branch: None,
		},
	]);
}

#[test]
fn if_else_branch() -> TestRes {
	ensure_program(r#"
	if (x < 12) {
		stuff = 1;
	} else {
		ot1_h *= 4;
	}
"#, vec![
		IfStatement {
			condition: BinaryExpression {
				operator: LessThan,
				left: Identifier("x".parse().unwrap()).into(),
				right: IntegerLiteral(12).into(),
			},
			if_branch: BlockStatement([
				ExpressionStatement(
					AssignmentExpression {
						operator: Assignment,
						left: Identifier("stuff".parse().unwrap()).into(),
						right: IntegerLiteral(1).into(),
					},
				)
			].into()).into(),
			else_branch: Some(BlockStatement([
				ExpressionStatement(
					AssignmentExpression {
						operator: MultiplicationAssignment,
						left: Identifier("ot1_h".parse().unwrap()).into(),
						right: IntegerLiteral(4).into(),
					},
				),
			].into()).into()),
		},
	]);
}

#[test]
fn if_if_else_branch() -> TestRes {
	ensure_program(r#"
	if (x < 12)
		if (y <= 13) stuff = 1;
		else ot1_h *= 4;
"#, vec![
		IfStatement {
			condition: BinaryExpression {
				operator: LessThan,
				left: Identifier("x".parse().unwrap()).into(),
				right: IntegerLiteral(12).into(),
			},
			if_branch: IfStatement {
				condition: BinaryExpression {
					operator: LessThanOrEqualTo,
					left: Identifier("y".parse().unwrap()).into(),
					right: IntegerLiteral(13).into(),
				},
				if_branch: ExpressionStatement(
					AssignmentExpression {
						operator: Assignment,
						left: Identifier("stuff".parse().unwrap()).into(),
						right: IntegerLiteral(1).into(),
					},
				).into(),
				else_branch: Some(ExpressionStatement(
					AssignmentExpression {
						operator: MultiplicationAssignment,
						left: Identifier("ot1_h".parse().unwrap()).into(),
						right: IntegerLiteral(4).into(),
					},
				).into()),
			}.into(),
			else_branch: None,
		},
	]);
}

#[test]
fn if_elseif_else_ladder() -> TestRes {
	ensure_program(r#"
	if (x < 12) {
		stuff = 1;
	} else if (Zyx > 12) {
		stuff /= 12;
	} else {
		val23 *= 4;
		$data %= 13;
	}
"#, vec![
		IfStatement {
			condition: BinaryExpression {
				operator: LessThan,
				left: Identifier("x".parse().unwrap()).into(),
				right: IntegerLiteral(12).into(),
			},
			if_branch: BlockStatement([
				ExpressionStatement(
					AssignmentExpression {
						operator: Assignment,
						left: Identifier("stuff".parse().unwrap()).into(),
						right: IntegerLiteral(1).into(),
					},
				)
			].into()).into(),
			else_branch: Some(IfStatement {
				condition: BinaryExpression {
					operator: GreaterThan,
					left: Identifier("Zyx".parse().unwrap()).into(),
					right: IntegerLiteral(12).into(),
				},
				if_branch: BlockStatement([
					ExpressionStatement(
						AssignmentExpression {
							operator: DivisionAssignment,
							left: Identifier("stuff".parse().unwrap()).into(),
							right: IntegerLiteral(12).into(),
						},
					)
				].into()).into(),
				else_branch: Some(BlockStatement([
					ExpressionStatement(
						AssignmentExpression {
							operator: MultiplicationAssignment,
							left: Identifier("val23".parse().unwrap()).into(),
							right: IntegerLiteral(4).into(),
						},
					),
					ExpressionStatement(
						AssignmentExpression {
							operator: ModulusAssignment,
							left: Identifier("$data".parse().unwrap()).into(),
							right: IntegerLiteral(13).into(),
						},
					),
				].into()).into()),
			}.into()),
		},
	]);
}
