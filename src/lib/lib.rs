pub mod parser;
pub mod ast;
pub mod tokenizer;
pub mod errors;

#[cfg(test)]
mod tests {
	use std::ops::Deref;

	use crate::ast::{Statement, StatementList};
	use crate::parser::parse;

	type TestRes = ();

	fn ensure_program(input: &str, expected: StatementList) -> TestRes {
		match parse(input.to_string()) {
			Ok(parsed) => {
				// println!("{:?}", parsed);
				assert_eq!(parsed.deref(), &expected, "Mismatched parsed AST and expected AST");
			}
			Err(error_type) => {
				panic!("{}", error_type)
			}
		}
		return;
	}

	fn ensure_parsed_statement(input: &str, expected: Statement) -> TestRes {
		return ensure_program(input, vec![expected]);
	}

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

	#[test]
	fn multiple_statements() -> TestRes {
		return ensure_program(r#""This is a string and this is a double quote: \"";
42;
"More stuff";"#, vec![
			Statement::StringLiteral("This is a string and this is a double quote: \"".parse().unwrap()),
			Statement::IntegerLiteral(42),
			Statement::StringLiteral("More stuff".parse().unwrap()),
		]);
	}

	#[test]
	fn block_statement() -> TestRes {
		return ensure_program(r#"{
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
		return ensure_program(r#"42;;"data";"#, vec![
			Statement::IntegerLiteral(42),
			Statement::EmptyStatement,
			Statement::StringLiteral("data".to_string()),
		]);
	}
}

