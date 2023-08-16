use evilang_lib::ast::{expression::Expression, statement::Statement};

use crate::common::{ensure_program, TestRes};

mod common;

#[test]
fn multiple_expressions() -> TestRes {
	ensure_program(r#""This is a string and this is a double quote: \"";
42;
"More stuff";"#, vec![
		Expression::StringLiteral("This is a string and this is a double quote: \"".parse().unwrap()).consume_as_statement(),
		Expression::integer_literal(42).consume_as_statement(),
		Expression::StringLiteral("More stuff".parse().unwrap()).consume_as_statement(),
	]);
}

#[test]
fn block_expression() -> TestRes {
	ensure_program(r#"{
	"This is a string and this is a double quote: \"";
	42;
}
"More stuff";"#, vec![
		Statement::BlockStatement(vec![
			Expression::StringLiteral("This is a string and this is a double quote: \"".parse().unwrap()).consume_as_statement(),
			Expression::integer_literal(42).consume_as_statement(),
		]),
		Expression::StringLiteral("More stuff".parse().unwrap()).consume_as_statement(),
	]);
}

#[test]
fn empty_expression() -> TestRes {
	ensure_program(r#"42;;"data";"#, vec![
		Expression::integer_literal(42).consume_as_statement(),
		Statement::EmptyStatement,
		Expression::StringLiteral("data".to_string()).consume_as_statement(),
	]);
}
