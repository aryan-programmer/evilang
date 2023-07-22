use crate::errors::{ErrorT, ResultWithError};
use crate::tokenizer::matchers::{get_token_matchers, Matcher};
pub use crate::tokenizer::token::{Keyword, TokenType};

mod matchers;
mod token;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Token {
	pub typ: TokenType,
	pub data: String
}

pub struct TokenStream {
	str: String,
	position: usize,
	token_matchers: Vec<(Matcher, Option<TokenType>)>,
	sent_eof_dummy: bool,
}

impl Iterator for TokenStream {
	type Item = ResultWithError<Token>;

	fn next(&mut self) -> Option<Self::Item> {
		'outer: loop {
			if self.position >= self.str.len() {
				return if self.sent_eof_dummy {
					None
				} else {
					Some(Ok(Token { typ: TokenType::_EOFDummy, data: String::new() }))
				}
			}
			let from = &self.str[self.position..];
			for (matcher, token_t) in self.token_matchers.iter() {
				let Some(s) = matcher(from) else { continue };
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
		return TokenStream {
			str,
			position: 0,
			token_matchers: get_token_matchers(),
			sent_eof_dummy: false,
		};
	}
}
