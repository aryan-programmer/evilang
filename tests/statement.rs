use evilang_lib::ast::Statement;

use crate::common::{ensure_program, TestRes};

mod common;

#[test]
fn multiple_statements() -> TestRes {
	ensure_program(r#""This is a string and this is a double quote: \"";
42;
"More stuff";"#, vec![
		Statement::StringLiteral("This is a string and this is a double quote: \"".parse().unwrap()),
		Statement::IntegerLiteral(42),
		Statement::StringLiteral("More stuff".parse().unwrap()),
	]);
}

#[test]
fn block_statement() -> TestRes {
	ensure_program(r#"{
	"This is a string and this is a double quote: \"";
	42;
}
"More stuff";"#, vec![
		Statement::BlockStatement(vec![
			Statement::StringLiteral("This is a string and this is a double quote: \"".parse().unwrap()),
			Statement::IntegerLiteral(42),
		]),
		Statement::StringLiteral("More stuff".parse().unwrap()),
	]);
}

#[test]
fn empty_statement() -> TestRes {
	ensure_program(r#"42;;"data";"#, vec![
		Statement::IntegerLiteral(42),
		Statement::EmptyStatement,
		Statement::StringLiteral("data".to_string()),
	]);
}
