use regex::Regex;

use crate::tokenizer::{Keyword, TokenType};

//language=regexp
const WHITESPACE_REGEX: &str = r"^[\s\r\n]+";
//language=regexp
const INTEGER_REGEX: &str = r"^\d+";
//language=regexp
const STRING_REGEX: &str = r#"^("[^"\\]*(?:\\.[^"\\]*)*")"#;
//language=regexp
const SINGLE_LINE_COMMENT_REGEX: &str = r#"^//.*"#;
//language=regexp
const MULTI_LINE_COMMENT_REGEX: &str = r#"^/\*[^*]*\*+(?:[^/*][^*]*\*+)*/"#;
//language=regexp
const MULTIPLICATIVE_OPERATORS_REGEX: &str = r#"^[*\\%]"#;
//language=regexp
const ADDITIVE_OPERATORS_REGEX: &str = r#"^[+\-]"#;
//language=regexp
const IDENTIFIER_REGEX: &str = r#"^[a-zA-Z_$][a-zA-Z0-9_$]*"#;
//language=regexp
const ASSIGNMENT_REGEX: &str = r#"^[+\-*\\%]?="#;

pub(super) type Matcher = Box<dyn Fn(&str) -> Option<&str>>;

fn regex_matcher(regex_str: &str) -> Matcher {
	let reg = Regex::new(regex_str).unwrap();
	return Box::new(move |s: &str| Some(reg.find(s)?.as_str()));
}

fn starts_with_matcher(start: &'static str) -> Matcher {
	return Box::new(move |s: &str| if s.starts_with(start) {
		Some(start)
	} else {
		None
	});
}

pub(super) fn get_token_matchers() -> Vec<(Matcher, Option<TokenType>)> {
	let regex_str_with_type = vec![
		(regex_matcher(WHITESPACE_REGEX), None),
		(regex_matcher(SINGLE_LINE_COMMENT_REGEX), None),
		(regex_matcher(MULTI_LINE_COMMENT_REGEX), None),
		//
		(starts_with_matcher(";"), Some(TokenType::Semicolon)),
		(starts_with_matcher("{"), Some(TokenType::OpenBlock)),
		(starts_with_matcher("}"), Some(TokenType::CloseBlock)),
		(starts_with_matcher("("), Some(TokenType::OpenParen)),
		(starts_with_matcher(")"), Some(TokenType::CloseParen)),
		(starts_with_matcher(","), Some(TokenType::Comma)),
		//
		(regex_matcher(ASSIGNMENT_REGEX), Some(TokenType::AssignmentOperator)),
		(regex_matcher(MULTIPLICATIVE_OPERATORS_REGEX), Some(TokenType::MultiplicativeOperator)),
		(regex_matcher(ADDITIVE_OPERATORS_REGEX), Some(TokenType::AdditiveOperator)),
		//
		(regex_matcher(INTEGER_REGEX), Some(TokenType::Integer)),
		(regex_matcher(STRING_REGEX), Some(TokenType::String)),
		//
		(starts_with_matcher("let"), Some(TokenType::Keyword(Keyword::Let))),
		//
		(regex_matcher(IDENTIFIER_REGEX), Some(TokenType::Identifier)),
	];
	return regex_str_with_type;
}
