use std::iter::Peekable;

use crate::ast::{Statement, StatementList};
use crate::errors::{ErrorT, ResultWithError};
use crate::tokenizer::{Token, TokenStream, TokenType};

pub fn parse(program: String) -> ResultWithError<StatementList> {
	let mut p = Parser::new(TokenStream::new(program));
	return p.program()
}

pub struct Parser {
	// stream: TokenStream,
	peekable_stream: Peekable<TokenStream>,
}

impl Parser {
	fn new(stream: TokenStream) -> Parser {
		return Parser { peekable_stream: stream.peekable() };
	}

	fn eat(&mut self, typ: TokenType) -> ResultWithError<Token> {
		let token = self.peekable_stream.next().ok_or(ErrorT::EndOfTokenStream)??;
		if token.typ != typ {
			return Err(ErrorT::InvalidTokenType.into())
		}
		return Ok(token)
	}

	fn lookahead(&mut self) -> ResultWithError<&Token> {
		return match self.peekable_stream.peek().ok_or(ErrorT::EndOfTokenStream.into()) {
			Ok(Ok(v)) => Ok(v),
			Ok(&Err(ref e)) => Err(e.clone()),
			Err(e) => Err(e),
		};
	}

	/*
	program:
		| statement_list
	*/
	fn program(&mut self) -> ResultWithError<StatementList> {
		return self.statement_list(None);
	}

	/*
	statement_list:
		| statement[]
	*/
	fn statement_list(&mut self, stop_lookahead_type: Option<TokenType>) -> ResultWithError<StatementList> {
		let mut res = StatementList::new();
		if let Some(stop) = stop_lookahead_type {
			while let Ok(lookahead) = self.lookahead() {
				if lookahead.typ == stop {
					break
				}
				res.push(self.statement()?);
			}
		} else {
			while let Ok(_lookahead) = self.lookahead() {
				res.push(self.statement()?);
			}
		}
		return Ok(res);
	}

	/*
	statement:
		| block_statement
		| expression_statement
		|
	*/
	fn statement(&mut self) -> ResultWithError<Statement> {
		return match self.lookahead()?.typ {
			TokenType::OpenBlock => self.block_statement(),
			TokenType::Semicolon => self.empty_statement(),
			_ => self.expression_statement(),
		}
	}

	/*
	block_statement:
		| '{' statement_list '}'
	*/
	fn block_statement(&mut self) -> ResultWithError<Statement> {
		if self.lookahead()?.typ == TokenType::CloseBlock {
			return Ok(Statement::BlockStatement(vec![]));
		}
		self.eat(TokenType::OpenBlock)?;
		let res = self.statement_list(Some(TokenType::CloseBlock))?;
		self.eat(TokenType::CloseBlock)?;
		return Ok(Statement::BlockStatement(res));
	}

	/*
	expression_statement:
		| expression ';'
	*/
	fn expression_statement(&mut self) -> ResultWithError<Statement> {
		let res = self.expression()?;
		self.eat(TokenType::Semicolon)?;
		return Ok(res);
	}

	/*
	empty_statement:
		| ';'
	*/
	fn empty_statement(&mut self) -> ResultWithError<Statement> {
		self.eat(TokenType::Semicolon)?;
		return Ok(Statement::EmptyStatement);
	}

	/*
	expression:
		| literal
	*/
	fn expression(&mut self) -> ResultWithError<Statement> {
		return self.literal();
	}

	/*
	expression:
		| integer_literal
		| string_literal
	*/
	fn literal(&mut self) -> ResultWithError<Statement> {
		return match self.lookahead()?.typ {
			TokenType::Integer => self.integer_literal(),
			TokenType::String => self.string_literal(),
			_ => Err(ErrorT::InvalidTokenType.into()),
		}
	}

	fn string_literal(&mut self) -> ResultWithError<Statement> {
		let v = self.eat(TokenType::String)?;
		let rep_v = v.data[1..v.data.len() - 1].replace("\\\"", "\"");
		return Ok(Statement::StringLiteral(rep_v))
	}

	fn integer_literal(&mut self) -> ResultWithError<Statement> {
		let v = self.eat(TokenType::Integer)?;
		return Ok(Statement::IntegerLiteral(v.data.parse().unwrap()))
	}
}
