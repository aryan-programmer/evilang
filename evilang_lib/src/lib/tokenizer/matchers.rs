use crate::tokenizer::{Keyword, TokenType};
use regex_automata::dfa::regex::Regex;

//language=regexp
const NUMBER_REGEX: &str = r"^[-+]?[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)?";
//language=regexp
const WHITESPACE_REGEX: &str = r"^[\s\r\n]+";
//language=regexp
// const INTEGER_REGEX: &str = r"^\d+";
//language=regexp
const STRING_REGEX: &str = r#"^("[^"\\]*(?:\\.[^"\\]*)*")"#;
//language=regexp
const SINGLE_LINE_COMMENT_REGEX: &str = r#"^//.*"#;
//language=regexp
const MULTI_LINE_COMMENT_REGEX: &str = r#"^/\*[^*]*\*+(?:[^/*][^*]*\*+)*/"#;
//language=regexp
const IDENTIFIER_REGEX: &str = r#"^[a-zA-Z_$][a-zA-Z0-9_$]*"#;

//pub(super) type Matcher = Box<dyn Fn(&str) -> Option<&str>>;
pub(super) enum Matcher {
	Regex(Box<Regex>),
	StartsWith(&'static str),
	Keyword {
		start: &'static str,
		start_code_points_len: usize,
	},
	OneOfMany(Vec<&'static str>),
}

impl Matcher {
	pub fn check_match<'a>(&self, s: &'a str) -> Option<&'a str> {
		match self {
			Matcher::Regex(reg) => Some(&s[reg.find(s)?.range()]),
			Matcher::StartsWith(start) if s.starts_with(start) => Some(start),
			Matcher::Keyword {
				start,
				start_code_points_len,
			} if s.starts_with(start)
				&& s
					.chars()
					.nth(*start_code_points_len)
					.is_some_and(|v| !is_identifier_char(v)) =>
			{
				Some(start)
			}
			Matcher::OneOfMany(items) => items
				.iter()
				.find(|start| s.starts_with(**start))
				.map(|v| &**v),
			_ => None,
		}
	}
}

fn is_identifier_char(ch: char) -> bool {
	if ch.is_alphanumeric() {
		return true;
	}
	return matches!(ch, '_' | '$');
}

fn regex_matcher(regex_str: &str) -> Matcher {
	return Matcher::Regex(Regex::new(regex_str).unwrap().into());
}

fn starts_with_matcher(start: &'static str) -> Matcher {
	return Matcher::StartsWith(start);
}

fn keyword_matcher(start: &'static str) -> Matcher {
	let start_code_points_len = start.chars().count();
	return Matcher::Keyword {
		start,
		start_code_points_len,
	};
}

fn one_of_many<const COUNT: usize>(starters: [&'static str; COUNT]) -> Matcher {
	return Matcher::OneOfMany(starters.to_vec());
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
		(starts_with_matcher("["), Some(TokenType::OpenSquareBracket)),
		(
			starts_with_matcher("]"),
			Some(TokenType::CloseSquareBracket),
		),
		(starts_with_matcher(","), Some(TokenType::Comma)),
		(starts_with_matcher("."), Some(TokenType::Dot)),
		(starts_with_matcher("::"), Some(TokenType::DoubleColon)),
		// (starts_with_matcher("->"), Some(TokenType::Arrow)),
		//
		(one_of_many(["==", "!="]), Some(TokenType::EqualityOperator)),
		(
			starts_with_matcher("&&"),
			Some(TokenType::LogicalAndOperator),
		),
		(
			starts_with_matcher("||"),
			Some(TokenType::LogicalOrOperator),
		),
		(
			starts_with_matcher("!"),
			Some(TokenType::LogicalNotOperator),
		),
		(
			one_of_many(["=", "+=", "-=", "*=", "/=", "%="]),
			Some(TokenType::AssignmentOperator),
		),
		(
			one_of_many(["*", "/", "%"]),
			Some(TokenType::MultiplicativeOperator),
		),
		(one_of_many(["+", "-"]), Some(TokenType::AdditiveOperator)),
		(
			one_of_many(["<=", ">="]),
			Some(TokenType::RelationalOperator),
		),
		(one_of_many(["<", ">"]), Some(TokenType::RelationalOperator)),
		//
		(regex_matcher(NUMBER_REGEX), Some(TokenType::Number)),
		// (regex_matcher(INTEGER_REGEX), Some(TokenType::Integer)),
		(regex_matcher(STRING_REGEX), Some(TokenType::String)),
		//
		(
			keyword_matcher("let"),
			Some(TokenType::Keyword(Keyword::Let)),
		),
		(keyword_matcher("if"), Some(TokenType::Keyword(Keyword::If))),
		(
			keyword_matcher("else"),
			Some(TokenType::Keyword(Keyword::Else)),
		),
		(
			keyword_matcher("true"),
			Some(TokenType::Keyword(Keyword::True)),
		),
		(
			keyword_matcher("false"),
			Some(TokenType::Keyword(Keyword::False)),
		),
		(
			keyword_matcher("null"),
			Some(TokenType::Keyword(Keyword::Null)),
		),
		(
			keyword_matcher("while"),
			Some(TokenType::Keyword(Keyword::While)),
		),
		(keyword_matcher("do"), Some(TokenType::Keyword(Keyword::Do))),
		(
			keyword_matcher("for"),
			Some(TokenType::Keyword(Keyword::For)),
		),
		(keyword_matcher("fn"), Some(TokenType::Keyword(Keyword::Fn))),
		(
			keyword_matcher("captures"),
			Some(TokenType::Keyword(Keyword::Captures)),
		),
		(
			keyword_matcher("return"),
			Some(TokenType::Keyword(Keyword::Return)),
		),
		(
			keyword_matcher("class"),
			Some(TokenType::Keyword(Keyword::Class)),
		),
		(
			keyword_matcher("extends"),
			Some(TokenType::Keyword(Keyword::Extends)),
		),
		(
			keyword_matcher("new"),
			Some(TokenType::Keyword(Keyword::New)),
		),
		(
			keyword_matcher("break"),
			Some(TokenType::Keyword(Keyword::Break)),
		),
		(
			keyword_matcher("continue"),
			Some(TokenType::Keyword(Keyword::Continue)),
		),
		(
			keyword_matcher("namespace"),
			Some(TokenType::Keyword(Keyword::Namespace)),
		),
		(
			keyword_matcher("import"),
			Some(TokenType::Keyword(Keyword::Import)),
		),
		(keyword_matcher("as"), Some(TokenType::Keyword(Keyword::As))),
		//
		(regex_matcher(IDENTIFIER_REGEX), Some(TokenType::Identifier)),
	];
	return regex_str_with_type;
}
