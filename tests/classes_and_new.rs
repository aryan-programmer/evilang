use evilang_lib::ast::expression::{Expression::{AssignmentExpression, BinaryExpression, Identifier, IntegerLiteral, MemberAccess}, Expression};
use evilang_lib::ast::expression::Expression::SuperExpression;
use evilang_lib::ast::expression::MemberIndexer::PropertyName;
use evilang_lib::ast::operator::Operator::{Assignment, Plus};
use evilang_lib::ast::statement::Statement::{BlockStatement, ClassDeclarationStatement, ReturnStatement, VariableDeclarations};
use evilang_lib::ast::structs::{FunctionDeclaration, FunctionParameterDeclaration, VariableDeclaration};

use crate::common::{ensure_program, TestRes};

mod common;

#[test]
fn classes_and_new() -> TestRes {
	ensure_program(r#"
class Point {
  fn constructor(this, x, y) {
    this.x = x;
    this.y = y;
  }

  fn calc(this) {
    return this.x + this.y;
  }
}

class Point3D extends Point {
  fn constructor(this, x, y, z) {
    super(x, y);
    this.z = z;
  }

  fn calc(this) {
    return super.calc() + this.z;
  }
}

let p = new Point(10, 12);
let p3 = new Point3D(10, 20, 30);
"#, [
		ClassDeclarationStatement {
			name: "Point".into(),
			super_class: None,
			methods: [
				FunctionDeclaration::new(
					"constructor".into(),
					[
						FunctionParameterDeclaration {
							identifier: "this".into(),
						},
						FunctionParameterDeclaration {
							identifier: "x".into(),
						},
						FunctionParameterDeclaration {
							identifier: "y".into(),
						},
					].into(),
					BlockStatement([
						AssignmentExpression {
							operator: Assignment,
							left: MemberAccess {
								object: Identifier("this".into()).into(),
								member: PropertyName("x".into()),
							}.into(),
							right: Identifier("x".into()).into(),
						}.consume_as_statement(),
						AssignmentExpression {
							operator: Assignment,
							left: MemberAccess {
								object: Identifier("this".into()).into(),
								member: PropertyName("y".into()),
							}.into(),
							right: Identifier("y".into()).into(),
						}.consume_as_statement(),
					].into()).into(),
				),
				FunctionDeclaration::new(
					"calc".into(),
					[
						FunctionParameterDeclaration {
							identifier: "this".into(),
						},
					].into(),
					BlockStatement([
						ReturnStatement(Some(
							BinaryExpression {
								operator: Plus,
								left: MemberAccess {
									object: Identifier("this".into()).into(),
									member: PropertyName("x".into()),
								}.into(),
								right: MemberAccess {
									object: Identifier("this".into()).into(),
									member: PropertyName("y".into()),
								}.into(),
							},
						)),
					].into()).into(),
				),
			].into(),
		},
		ClassDeclarationStatement {
			name: "Point3D".into(),
			super_class: Some(Identifier("Point".into())),
			methods: [
				FunctionDeclaration::new(
					"constructor".into(),
					[
						FunctionParameterDeclaration {
							identifier: "this".into(),
						},
						FunctionParameterDeclaration {
							identifier: "x".into(),
						},
						FunctionParameterDeclaration {
							identifier: "y".into(),
						},
						FunctionParameterDeclaration {
							identifier: "z".into(),
						},
					].into(),
					BlockStatement([
						Expression::function_call(
							SuperExpression.into(),
							[
								Identifier("x".into()),
								Identifier("y".into()),
							].into(),
						).consume_as_statement(),
						AssignmentExpression {
							operator: Assignment,
							left: MemberAccess {
								object: Identifier("this".into()).into(),
								member: PropertyName("z".into()),
							}.into(),
							right: Identifier("z".into()).into(),
						}.consume_as_statement(),
					].into()).into(),
				),
				FunctionDeclaration::new(
					"calc".into(),
					[
						FunctionParameterDeclaration {
							identifier: "this".into(),
						},
					].into(),
					BlockStatement([
						ReturnStatement(Some(
							BinaryExpression {
								operator: Plus,
								left: Expression::function_call(
									MemberAccess {
										object: SuperExpression.into(),
										member: PropertyName("calc".into()),
									}.into(),
									[].into(),
								).into(),
								right: MemberAccess {
									object: Identifier("this".into()).into(),
									member: PropertyName("z".into()),
								}.into(),
							},
						)),
					].into()).into(),
				),
			].into(),
		},
		VariableDeclarations([
			VariableDeclaration {
				identifier: "p".into(),
				initializer: Some(Expression::new_object_expression(
					Identifier("Point".into()).into(),
					[
						IntegerLiteral(10),
						IntegerLiteral(12),
					].into(),
				)),
			},
		].into()),
		VariableDeclarations([
			VariableDeclaration {
				identifier: "p3".into(),
				initializer: Some(Expression::new_object_expression(
					Identifier("Point3D".into()).into(),
					[
						IntegerLiteral(10),
						IntegerLiteral(20),
						IntegerLiteral(30),
					].into(),
				)),
			},
		].into()),
	].into());
}
