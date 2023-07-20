use std::iter::Peekable;

use crate::ast::{expression::{BoxExpression, Expression}, operator::Operator, statement::{Statement, StatementList}};
use crate::ast::statement::VariableDeclaration;
use crate::errors::{ensure, ErrorT, ResultWithError};
use crate::tokenizer::{Keyword, Token, TokenStream, TokenType};

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
		| empty_statement
		| variable_declarations_statement
	*/
	fn statement(&mut self) -> ResultWithError<Statement> {
		return match self.lookahead()?.typ {
			TokenType::OpenBlock => self.block_statement(),
			TokenType::Semicolon => self.empty_statement(),
			TokenType::Keyword(Keyword::Let) => self.variable_declarations_statement(),
			_ => self.expression_statement(),
		}
	}

	fn variable_declarations_statement(&mut self) -> ResultWithError<Statement> {
		self.eat(TokenType::Keyword(Keyword::Let))?;
		let res = self.variable_declarations()?;
		self.eat(TokenType::Semicolon)?;
		return Ok(Statement::VariableDeclarations(res));
	}

	fn variable_declarations(&mut self) -> ResultWithError<Vec<VariableDeclaration>> {
		let mut res = Vec::<VariableDeclaration>::new();
		loop {
			res.push(self.variable_declaration()?);
			if self.lookahead()?.typ != TokenType::Comma {
				break;
			} else {
				self.eat(TokenType::Comma)?;
			}
		}
		return Ok(res);
	}

	fn variable_declaration(&mut self) -> ResultWithError<VariableDeclaration> {
		let identifier = self.eat(TokenType::Identifier)?.data;
		let initializer = match self.lookahead()?.typ {
			TokenType::Semicolon | TokenType::Comma => None,
			_ => Some(self.variable_initializer()?)
		};
		return Ok(VariableDeclaration {
			identifier,
			initializer,
		});
	}

	fn variable_initializer(&mut self) -> ResultWithError<Expression> {
		let oper = Operator::try_from(&self.eat(TokenType::AssignmentOperator)?.data)?;
		if oper != Operator::Assignment {
			return Err(ErrorT::ExpectedSimpleAssignmentOperator.into())
		}
		return self.assignment_expression();
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
		return Ok(Statement::ExpressionStatement(res));
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
	fn expression(&mut self) -> ResultWithError<Expression> {
		return self.assignment_expression();
	}

	/*
	assignment_expression:
		| additive_expression
		| lhs AssignmentOperator assignment_expression
	*/
	fn assignment_expression(&mut self) -> ResultWithError<Expression> {
		let left = self.additive_expression()?;
		if self.lookahead()?.typ != TokenType::AssignmentOperator {
			return Ok(left);
		}
		let op = self.eat(TokenType::AssignmentOperator)?;
		ensure(left.is_lhs(), ErrorT::ExpectedLhsExpression)?;
		let right = self.assignment_expression()?;
		return Ok(Expression::assignment_expression(
			Operator::try_from(&op.data)?,
			BoxExpression::from(left),
			BoxExpression::from(right),
		));
	}

	/*
	additive_expression:
		| multiplicative_expression
		| additive_expression AdditiveOperator multiplicative_expression
	*/
	fn additive_expression(&mut self) -> ResultWithError<Expression> {
		return self.left_to_right_binary_expression(
			Self::multiplicative_expression,
			TokenType::AdditiveOperator
		);
	}

	/*
	multiplicative_expression:
		| primary_expression
		| multiplicative_expression MultiplicativeOperator primary_expression
	*/
	fn multiplicative_expression(&mut self) -> ResultWithError<Expression> {
		return self.left_to_right_binary_expression(
			Self::primary_expression,
			TokenType::MultiplicativeOperator
		);
	}

	/*
	lhs_expression:
		| identifier
	*/
	fn lhs_expression(&mut self) -> ResultWithError<Expression> {
		return Ok(Expression::Identifier(self.eat(TokenType::Identifier)?.data));
	}

	/*
	primary_expression:
		| literal
		| parenthesized_expression
		| lhs_expression
	*/
	fn primary_expression(&mut self) -> ResultWithError<Expression> {
		let token_type = self.lookahead()?.typ;
		if token_type.is_literal() {
			return self.literal();
		}
		return match token_type {
			TokenType::OpenParen => self.parenthesized_expression(),
			_ => self.lhs_expression(),
		};
	}

	/*
	parenthesized_expression:
		| '(' expression ')'
	*/
	fn parenthesized_expression(&mut self) -> ResultWithError<Expression> {
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
		sub_expression: fn(&mut Self) -> ResultWithError<Expression>,
		expression_operator_token_type: TokenType,
	) -> ResultWithError<Expression> {
		let mut left = sub_expression(self)?;
		while self.lookahead()?.typ == expression_operator_token_type {
			let op = self.eat(expression_operator_token_type)?;
			let right = sub_expression(self)?;
			left = Expression::binary_expression(
				Operator::try_from(&op.data)?,
				BoxExpression::from(left),
				BoxExpression::from(right),
			);
		}
		return Ok(left);
	}

	/*
	literal:
		| integer_literal
		| string_literal
	*/
	fn literal(&mut self) -> ResultWithError<Expression> {
		return match self.lookahead()?.typ {
			TokenType::Integer => self.integer_literal(),
			TokenType::String => self.string_literal(),
			_ => Err(ErrorT::InvalidTokenType.into()),
		}
	}

	fn string_literal(&mut self) -> ResultWithError<Expression> {
		let v = self.eat(TokenType::String)?;
		let rep_v = v.data[1..v.data.len() - 1].replace("\\\"", "\"");
		return Ok(Expression::StringLiteral(rep_v))
	}

	fn integer_literal(&mut self) -> ResultWithError<Expression> {
		let v = self.eat(TokenType::Integer)?;
		return Ok(Expression::IntegerLiteral(v.data.parse().unwrap()))
	}
}
