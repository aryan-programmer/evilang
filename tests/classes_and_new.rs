use evilang_lib::ast::expression::{Expression::{AssignmentExpression, BinaryExpression, Identifier, MemberAccess}, Expression};
use evilang_lib::ast::expression::MemberIndexer::PropertyName;
use evilang_lib::ast::operator::Operator::{Assignment, Plus};
use evilang_lib::ast::statement::Statement;
use evilang_lib::ast::statement::Statement::{BlockStatement, ReturnStatement, VariableDeclarations};
use evilang_lib::ast::structs::{FunctionDeclaration, FunctionParameterDeclaration, VariableDeclaration};
use evilang_lib::interpreter::runtime_values::PrimitiveValue;

use crate::common::{push_res_stack_stmt, TestData, TestRes};

mod common;

#[test]
fn classes_and_new() -> TestRes {
	TestData::new(r#"
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
		super.constructor(this, x, y);
		this.z = z;
	}

	fn calc(this) {
		return super.calc(this) + this.z;
	}
}

let p = new Point(10, 12);
let p3 = new Point3D(10, 20, 30);
push_res_stack(p->calc());
push_res_stack(p3->calc());
"#.to_string())
		// region ...expect_statements
		.expect_statements([
			// region ...Point
			Statement::class_declaration(
				"Point".into(),
				None,
				[
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
			),
			// endregion Point
			// region ...Point3D
			Statement::class_declaration(
				"Point3D".into(),
				Some(Identifier("Point".into())),
				[
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
								Expression::member_property_access(
									Identifier("super".into()).into(),
									"constructor".into(),
								).into(),
								[
									Identifier("this".into()),
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
											object: Identifier("super".into()).into(),
											member: PropertyName("calc".into()),
										}.into(),
										[
											Identifier("this".into())
										].into(),
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
			),
			// endregion Point3D
			VariableDeclarations([
				VariableDeclaration {
					identifier: "p".into(),
					initializer: Some(Expression::new_object_expression(
						Identifier("Point".into()).into(),
						[
							Expression::integer_literal(10),
							Expression::integer_literal(12),
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
							Expression::integer_literal(10),
							Expression::integer_literal(20),
							Expression::integer_literal(30),
						].into(),
					)),
				},
			].into()),
			push_res_stack_stmt(Expression::function_call(
				Expression::member_method_access(
					Identifier("p".into()).into(),
					"calc".into(),
				).into(),
				[].into(),
			)),
			push_res_stack_stmt(Expression::function_call(
				Expression::member_method_access(
					Identifier("p3".into()).into(),
					"calc".into(),
				).into(),
				[].into(),
			)),
		].into())
		// endregion expect_statements
		.expect_stack([
			PrimitiveValue::integer(22),
			PrimitiveValue::integer(60),
		].into());
}

#[test]
fn object_property_updating() -> TestRes {
	TestData::new(r#"
class SuperClass {
}
SuperClass.x = -2;

class Point extends SuperClass {
	fn constructor(this, x, y) {
		this.x = x;
		this.y = y;
	}

	fn calc(this) {
		return this.x + this.y;
	}

	fn setX(this, x){
		this.x = x;
	}

	fn setY(this, y){
		this.y = y;
	}
}
Point.x = -1;

let p = new Point(10, 12);
push_res_stack(p->calc());
p->setX(31);
push_res_stack(p->calc());
p->setY(11);
push_res_stack(p->calc());

push_res_stack(SuperClass.x);
push_res_stack(Point.x);
push_res_stack(p.x);
"#.to_string())
		.expect_stack([
			PrimitiveValue::integer(22),
			PrimitiveValue::integer(43),
			PrimitiveValue::integer(42),
			PrimitiveValue::integer(-2),
			PrimitiveValue::integer(-1),
			PrimitiveValue::integer(31),
		].into());
}

#[test]
fn monkey_patching() -> TestRes {
	TestData::new(r#"
class Point {
	fn constructor(this, x, y) {
		this.x = x;
		this.y = y;
	}

	fn calc(this) {
		return this.x + this.y;
	}
}

fn pusher(pnt){
	push_res_stack(pnt.x, pnt.y, pnt->calc());
}

let p = new Point(10, 12);
pusher(p);

Point.push = pusher;

p->push();

push_res_stack(p.push == pusher);
"#.to_string())
		.expect_stack([
			PrimitiveValue::integer(10),
			PrimitiveValue::integer(12),
			PrimitiveValue::integer(22),
			PrimitiveValue::integer(10),
			PrimitiveValue::integer(12),
			PrimitiveValue::integer(22),
			PrimitiveValue::Boolean(true),
		].into());
}
