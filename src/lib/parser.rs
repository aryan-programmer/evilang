use std::iter::Peekable;

use crate::ast::{BoxStatement, Operator, Statement, StatementList};
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
		| multiplicative_expression
	*/
	fn expression(&mut self) -> ResultWithError<Statement> {
		return self.additive_expression();
	}

	/*
	multiplicative_expression:
		| primary_expression
		| multiplicative_expression MultiplicativeOperator primary_expression
	*/
	fn multiplicative_expression(&mut self) -> ResultWithError<Statement> {
		return self.left_to_right_binary_expression(
			Self::primary_expression,
			TokenType::MultiplicativeOperator
		);
	}

	/*
	additive_expression:
		| multiplicative_expression
		| additive_expression AdditiveOperator multiplicative_expression
	*/
	fn additive_expression(&mut self) -> ResultWithError<Statement> {
		return self.left_to_right_binary_expression(
			Self::multiplicative_expression,
			TokenType::AdditiveOperator
		);
	}

	/*
	primary_expression:
		| literal
		| parenthesized_expression
	*/
	fn primary_expression(&mut self) -> ResultWithError<Statement> {
		return match self.lookahead()?.typ {
			TokenType::OpenParen=>self.parenthesized_expression(),
			_=>self.literal(),
		};
	}

	/*
	parenthesized_expression:
		| '(' expression ')'
	*/
	fn parenthesized_expression(&mut self) -> ResultWithError<Statement> {
		self.eat(TokenType::OpenParen)?;
		let res = self.expression();
		self.eat(TokenType::CloseParen)?;
		return res;
	}

	/*
	left_to_right_binary_expression(sub_expression, ExprOperator):
		| sub_expression
		| left_to_right_binary_expression(sub_expression, ExprOperator) ExprOperator sub_expression
	*/
	fn left_to_right_binary_expression(
		&mut self,
		sub_expression: fn(&mut Self)->ResultWithError<Statement>,
		expression_operator_token_type: TokenType,
	) -> ResultWithError<Statement> {
		let mut left = sub_expression(self)?;
		while self.lookahead()?.typ == expression_operator_token_type {
			let op = self.eat(expression_operator_token_type)?;
			let right = sub_expression(self)?;
			left = Statement::BinaryExpression {
				left: BoxStatement::from(left),
				right: BoxStatement::from(right),
				operator: Operator::try_from(&op.data)?
			};
		}
		return Ok(left);
	}

	/*
	literal:
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
