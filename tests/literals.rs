use evilang_lib::ast::expression::Expression;

use crate::common::{ensure_parsed_statement, TestRes};

mod common;

#[test]
fn integer_literal() -> TestRes {
	return ensure_parsed_statement("42;", Expression::IntegerLiteral(42).consume_as_statement());
}

#[test]
fn string_literal() -> TestRes {
	return ensure_parsed_statement(
		r#""This is a string and this is a double quote: \"";"#,
		Expression::StringLiteral("This is a string and this is a double quote: \"".parse().unwrap()).consume_as_statement()
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
		Expression::StringLiteral("someCode();".parse().unwrap()).consume_as_statement()
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
		Expression::StringLiteral("someOtherCode();".parse().unwrap()).consume_as_statement()
	);
}
