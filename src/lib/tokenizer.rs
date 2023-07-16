use once_cell::sync::Lazy;
use regex::Regex;

use crate::errors::{ErrorT, ResultWithError};

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
const SEMICOLON_REGEX: &str = r#"^;"#;
//language=regexp
const OPEN_BLOCK_REGEX: &str = r#"^\{"#;
//language=regexp
const CLOSE_BLOCK_REGEX: &str = r#"^\}"#;
//language=regexp
const MULTIPLICATIVE_OPERATORS_REGEX: &str = r#"^[*\\%]"#;
//language=regexp
const ADDITIVE_OPERATORS_REGEX: &str = r#"^[+\-]"#;
//language=regexp
const OPEN_PAREN_REGEX: &str = r#"^\("#;
//language=regexp
const CLOSE_PAREN_REGEX: &str = r#"^\)"#;

static REGEX_TO_TOKEN_TYPE: Lazy<Vec<(Regex, Option<TokenType>)>> = Lazy::new(|| {
	let regex_str_with_type = [
		(WHITESPACE_REGEX, None),
		(SINGLE_LINE_COMMENT_REGEX, None),
		(MULTI_LINE_COMMENT_REGEX, None),
		(INTEGER_REGEX, Some(TokenType::Integer)),
		(STRING_REGEX, Some(TokenType::String)),
		(SEMICOLON_REGEX, Some(TokenType::Semicolon)),
		(OPEN_BLOCK_REGEX, Some(TokenType::OpenBlock)),
		(CLOSE_BLOCK_REGEX, Some(TokenType::CloseBlock)),
		(MULTIPLICATIVE_OPERATORS_REGEX, Some(TokenType::MultiplicativeOperator)),
		(ADDITIVE_OPERATORS_REGEX, Some(TokenType::AdditiveOperator)),
		(OPEN_PAREN_REGEX, Some(TokenType::OpenParen)),
		(CLOSE_PAREN_REGEX, Some(TokenType::CloseParen)),
	];
	return regex_str_with_type
		.iter()
		.map(|v| (Regex::new(v.0).unwrap(), v.1))
		.collect();
});

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TokenType {
	Integer,
	String,
	Semicolon,
	OpenBlock,
	CloseBlock,
	MultiplicativeOperator,
	AdditiveOperator,
	OpenParen,
	CloseParen,
}

#[derive(Eq, PartialEq, Hash)]
pub struct Token {
	pub typ: TokenType,
	pub data: String
}

pub struct TokenStream {
	str: String,
	position: usize,
	// chars: Chars<'_>,
	// words: UWordBounds<'_>,
}

impl Iterator for TokenStream {
	type Item = ResultWithError<Token>;

	fn next(&mut self) -> Option<Self::Item> {
		'outer: loop {
			// let word_opt = self.words.next();
			if self.position >= self.str.len() {
				return None;
			}
			// let word = word_opt.unwrap();
			let from = &self.str[self.position..];
			for (regex, token_t) in REGEX_TO_TOKEN_TYPE.iter() {
				let Some(matched_string) = regex.find(from) else { continue };
				let s = matched_string.as_str();
				self.position += s.len();
				if let None = token_t {
					continue 'outer;
				}
				return Some(Ok(Token { typ: token_t.unwrap().clone(), data: s.parse().unwrap() }));
			}
			return Some(Err(ErrorT::TokenCannotBeParsed.into()));
		}
	}
}

impl TokenStream {
	pub fn new(str: String) -> TokenStream {
		return TokenStream { str, position: 0, /*chars: str.chars(), words: str.split_word_bounds()*/ };
	}
}
