use evilang_lib::ast::expression::Expression::{AssignmentExpression, BinaryExpression, Identifier, IntegerLiteral};
use evilang_lib::ast::operator::Operator::{MultiplicationAssignment, NotEquals, PlusAssignment};
use evilang_lib::ast::statement::Statement;
use evilang_lib::ast::statement::Statement::{BlockStatement, ExpressionStatement, IfStatement, ReturnStatement};
use evilang_lib::ast::structs::FunctionParameterDeclaration;

use crate::common::{ensure_parsing_fails, ensure_program, TestRes};

mod common;

#[test]
fn invalids() -> TestRes {
	ensure_parsing_fails("fn func_name(, ){}", None);
	ensure_parsing_fails("fn func_name();", None);
	ensure_parsing_fails("fn func_name(param1, val2) param1 += val2;", None);
}

#[test]
fn function_with_no_params_and_no_body() -> TestRes {
	ensure_program(r#"
	fn func_name(){}
"#, vec![Statement::function_declaration(
		"func_name".to_string(),
		vec![],
		BlockStatement(vec![]).into(),
	)]);
}

#[test]
fn function_with_params_and_no_body() -> TestRes {
	ensure_program(r#"
	fn func_name(param1, val2){}
"#, vec![Statement::function_declaration(
		"func_name".to_string(),
		vec![
			FunctionParameterDeclaration {
				identifier: "param1".to_string(),
			},
			FunctionParameterDeclaration {
				identifier: "val2".to_string(),
			},
		],
		BlockStatement(vec![]).into(),
	)]);
}

#[test]
fn function_with_no_params() -> TestRes {
	ensure_program(r#"
	fn func_name(){
		return;
	}
"#, vec![Statement::function_declaration(
		"func_name".to_string(),
		vec![],
		BlockStatement(vec![ReturnStatement(None)]).into(),
	)]);
}

#[test]
fn function_with_params_and_return() -> TestRes {
	ensure_program(r#"
	fn func_name(param1, val2){
		param1 += val2;
		if(param1 != 0){
			return val2 *= param1;
		} else {
			return;
		}
	}
"#, vec![Statement::function_declaration(
		"func_name".to_string(),
		vec![
			FunctionParameterDeclaration {
				identifier: "param1".to_string(),
			},
			FunctionParameterDeclaration {
				identifier: "val2".to_string(),
			},
		],
		BlockStatement(vec![
			ExpressionStatement(
				AssignmentExpression {
					operator: PlusAssignment,
					left: Identifier("param1".to_string()).into(),
					right: Identifier("val2".to_string()).into(),
				},
			),
			IfStatement {
				condition: BinaryExpression {
					operator: NotEquals,
					left: Identifier("param1".to_string()).into(),
					right: IntegerLiteral(0).into(),
				},
				if_branch: BlockStatement(vec![
					ReturnStatement(
						Some(AssignmentExpression {
							operator: MultiplicationAssignment,
							left: Identifier("val2".to_string()).into(),
							right: Identifier("param1".to_string()).into(),
						}),
					),
				]).into(),
				else_branch: Some(BlockStatement(vec![ReturnStatement(None)]).into()),
			}.into(),
		]).into(),
	)]);
}
