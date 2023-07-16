use evilang_lib::ast::Statement;

use crate::common::{ensure_parsed_statement, TestRes};

mod common;

#[test]
fn integer_literal() -> TestRes {
	return ensure_parsed_statement("42;", Statement::IntegerLiteral(42));
}

#[test]
fn string_literal() -> TestRes {
	return ensure_parsed_statement(
		r#""This is a string and this is a double quote: \"";"#,
		Statement::StringLiteral("This is a string and this is a double quote: \"".parse().unwrap())
	);
}

#[test]
fn comments_and_white_space() -> TestRes {
	ensure_parsed_statement(
		r#"
// This is a comment
//*
"someCode();"
/*/
"someOtherCode();"
//*/
;
"#,
		Statement::StringLiteral("someCode();".parse().unwrap())
	);
	ensure_parsed_statement(
		r#"
// This is a comment
/*
"someCode();"
/*/
"someOtherCode();"
//*/
;
"#,
		Statement::StringLiteral("someOtherCode();".parse().unwrap())
	);
}
