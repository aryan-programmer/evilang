use evilang_lib::ast::expression::Expression;
use evilang_lib::interpreter::runtime_value::PrimitiveValue;

use crate::common::{ensure_program_statement_results, TestRes};

mod common;

#[test]
fn integer_literal() -> TestRes {
	return ensure_program_statement_results(
		"42;",
		vec![Expression::IntegerLiteral(42).consume_as_statement()],
		vec![PrimitiveValue::Integer(42)],
	);
}

#[test]
fn string_literal() -> TestRes {
	return ensure_program_statement_results(
		r#""This is a string and this is a double quote: \"";"#,
		vec![Expression::StringLiteral("This is a string and this is a double quote: \"".parse().unwrap()).consume_as_statement()],
		vec![PrimitiveValue::String("This is a string and this is a double quote: \"".parse().unwrap())],
	);
}

#[test]
fn comments_and_white_space() -> TestRes {
	ensure_program_statement_results(
		r#"
// This is a comment
//*
"someCode();"
/*/
"someOtherCode();"
//*/
;
"#,
		vec![Expression::StringLiteral("someCode();".parse().unwrap()).consume_as_statement()],
		vec![PrimitiveValue::String("someCode();".parse().unwrap())],
	);
	ensure_program_statement_results(
		r#"
// This is a comment
/*
"someCode();"
/*/
"someOtherCode();"
//*/
;
"#,
		vec![Expression::StringLiteral("someOtherCode();".parse().unwrap()).consume_as_statement()],
		vec![PrimitiveValue::String("someOtherCode();".parse().unwrap())],
	);
}
