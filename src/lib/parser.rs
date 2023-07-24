use std::iter::Peekable;

use crate::ast::{expression::{BoxExpression, Expression}, operator::Operator, statement::{Statement, StatementList}};
use crate::ast::expression::IdentifierT;
use crate::ast::statement::BoxStatement;
use crate::ast::structs::{FunctionParameterDeclaration, VariableDeclaration};
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

macro_rules! binary_expressions {
    ($base_vis: vis fn $base_fn_name: ident(&mut self) -> $res_type: ty {
	    wrapper_function: $wrapper: ident;
	    $sub_vis1: vis $fn_name1: ident: $token_type1: expr;
	    $($sub_vis: vis $fn_name: ident: $token_type: expr);*;
    }) => {
	    $base_vis fn $base_fn_name(&mut self) -> $res_type {
		    return self.$fn_name1();
	    }
	    binary_expressions!(@@sub_parse $res_type, $wrapper, $sub_vis1, $fn_name1, $token_type1 $(; $sub_vis, $fn_name, $token_type)*;);
    };
	(@@sub_parse
		$res_type: ty, $wrapper: ident,
	    $sub_vis1: vis, $fn_name1: ident, $token_type1: expr;
	    $sub_vis2: vis, $fn_name2: ident, $token_type2: expr;
	    $($sub_vis: vis, $fn_name: ident, $token_type: expr);*;) => {
		$sub_vis1 fn $fn_name1(&mut self) -> $res_type {
		    return self.$wrapper(Self::$fn_name2, $token_type1);
	    }
	    binary_expressions!(@@sub_parse $res_type, $wrapper, $sub_vis2, $fn_name2, $token_type2 $(; $sub_vis, $fn_name, $token_type)*;);
	};
	(@@sub_parse
		$res_type: ty, $wrapper: ident,
	    $sub_vis1: vis, $fn_name1: ident, $token_type1: expr;
	    $sub_vis2: vis, $fn_name2: ident, $token_type2: expr;) => {
		$sub_vis1 fn $fn_name1(&mut self) -> $res_type {
			static_assertions::const_assert!(const_str::equal!(stringify!($token_type2), "None"));
			static_assertions::const_assert!(const_str::equal!(stringify!($sub_vis2), ""));
		    return self.$wrapper(Self::$fn_name2, $token_type1);
	    }
	}
}

impl Parser {
	pub fn new(stream: TokenStream) -> Parser {
		return Parser { peekable_stream: stream.peekable() };
	}

	fn identifier(&mut self) -> ResultWithError<IdentifierT> {
		return Ok(self.eat(TokenType::Identifier)?.data);
	}

	fn eat_any(&mut self) -> ResultWithError<Token> {
		return Ok(self.peekable_stream.next().ok_or(ErrorT::EndOfTokenStream)??)
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
	pub fn program(&mut self) -> ResultWithError<StatementList> {
		return self.statement_list(None);
	}

	/*
	statement_list:
		| statement[]
	*/
	fn statement_list(&mut self, stop_lookahead_type: Option<TokenType>) -> ResultWithError<StatementList> {
		let mut res = StatementList::new();
		let stop = stop_lookahead_type.unwrap_or(TokenType::_EOFDummy);
		while let Ok(lookahead) = self.lookahead() {
			if lookahead.typ == stop {
				break
			}
			res.push(self.statement()?);
		}
		return Ok(res);
	}

	/*
	statement:
		| block_statement
		| expression_statement
		| empty_statement
		| variable_declarations_statement
		| if_statement
		| while_loop
		| do_while_loop
		| for_loop
	*/
	fn statement(&mut self) -> ResultWithError<Statement> {
		return match self.lookahead()?.typ {
			TokenType::OpenBlock => self.block_statement(),
			TokenType::Semicolon => self.empty_statement(),
			TokenType::Keyword(Keyword::Let) => self.variable_declarations_statement(),
			TokenType::Keyword(Keyword::If) => self.if_statement(),
			TokenType::Keyword(Keyword::While) => self.while_loop(),
			TokenType::Keyword(Keyword::Do) => self.do_while_loop(),
			TokenType::Keyword(Keyword::For) => self.for_loop(),
			TokenType::Keyword(Keyword::Fn) => self.function_statement(),
			TokenType::Keyword(Keyword::Return) => self.return_statement(),
			_ => self.expression_statement(),
		};
	}

	/*
	return_statement:
		| 'return' ';'
		| 'return' expression ';'
	*/
	fn return_statement(&mut self) -> ResultWithError<Statement> {
		self.eat(TokenType::Keyword(Keyword::Return))?;
		let res = if self.lookahead()?.typ != TokenType::Semicolon {
			Some(self.expression()?)
		} else { None };
		self.eat(TokenType::Semicolon)?;
		return Ok(Statement::ReturnStatement(res));
	}

	/*
	function_statement:
		| 'fn' Identifier '(' function_parameter_declarations ')' block_statement
	*/
	fn function_statement(&mut self) -> ResultWithError<Statement> {
		self.eat(TokenType::Keyword(Keyword::Fn))?;
		let name: IdentifierT = self.identifier()?;
		self.eat(TokenType::OpenParen)?;
		let params = self.delimited_items(
			Self::function_parameter_declaration,
			TokenType::Comma,
			TokenType::CloseParen
		)?;
		self.eat(TokenType::CloseParen)?;
		let body = self.block_statement()?;
		return Ok(Statement::function_declaration(name, params, body.into()));
	}

	/*
	function_parameter_declaration:
		| Identifier
	*/
	fn function_parameter_declaration(&mut self) -> ResultWithError<FunctionParameterDeclaration> {
		return Ok(FunctionParameterDeclaration::new(self.identifier()?));
	}

	/*
	if_statement:
		| 'if' '(' expression ')' statement
		| 'if' '(' expression ')' statement 'else' statement
	*/
	fn if_statement(&mut self) -> ResultWithError<Statement> {
		self.eat(TokenType::Keyword(Keyword::If))?;
		self.eat(TokenType::OpenParen)?;
		let condition = self.expression()?;
		self.eat(TokenType::CloseParen)?;
		let if_branch = BoxStatement::from(self.statement()?);
		let else_branch = match self.lookahead()?.typ {
			TokenType::Keyword(Keyword::Else) => {
				self.eat(TokenType::Keyword(Keyword::Else))?;
				Some(BoxStatement::from(self.statement()?))
			},
			_ => None
		};
		return Ok(Statement::if_statement(condition, if_branch, else_branch));
	}

	/*
	while_loop:
		| 'while' '(' expression ')' statement
	*/
	fn while_loop(&mut self) -> ResultWithError<Statement> {
		self.eat(TokenType::Keyword(Keyword::While))?;
		self.eat(TokenType::OpenParen)?;
		let condition = self.expression()?;
		self.eat(TokenType::CloseParen)?;
		let body = BoxStatement::from(self.statement()?);
		return Ok(Statement::while_loop(condition, body));
	}

	/*
	do_while_loop:
		| 'do' block_statement 'while' '(' expression ')' ';'
	*/
	fn do_while_loop(&mut self) -> ResultWithError<Statement> {
		self.eat(TokenType::Keyword(Keyword::Do))?;
		let body = BoxStatement::from(self.block_statement()?);
		self.eat(TokenType::Keyword(Keyword::While))?;
		self.eat(TokenType::OpenParen)?;
		let condition = self.expression()?;
		self.eat(TokenType::CloseParen)?;
		self.eat(TokenType::Semicolon)?;
		return Ok(Statement::do_while_loop(condition, body));
	}

	/*
	for_loop:
		| 'for' '(' expression ')' statement
	*/
	fn for_loop(&mut self) -> ResultWithError<Statement> {
		self.eat(TokenType::Keyword(Keyword::For))?;
		self.eat(TokenType::OpenParen)?;
		let init = self.for_loop_initialization_statement()?;
		let condition = self.for_loop_condition_expression()?;
		let increment = self.for_loop_increment_statement()?;
		self.eat(TokenType::CloseParen)?;
		let body = self.statement()?;
		return Ok(Statement::for_loop(init.into(), condition, increment.into(), body.into()));
	}

	/*
	for_loop_initialization_statement:
		| block_statement ';'
		| empty_statement
		| variable_declarations_statement
		| expression_statement
	*/
	fn for_loop_initialization_statement(&mut self) -> ResultWithError<Statement> {
		return match self.lookahead()?.typ {
			TokenType::OpenBlock => {
				let res = self.block_statement()?;
				self.eat(TokenType::Semicolon)?;
				Ok(res)
			},
			TokenType::Semicolon => self.empty_statement(),
			TokenType::Keyword(Keyword::Let) => self.variable_declarations_statement(),
			_ => self.expression_statement(),
		};
	}

	/*
	for_loop_condition_expression:
		| empty_statement
		| expression_statement
	*/
	fn for_loop_condition_expression(&mut self) -> ResultWithError<Expression> {
		let result = match self.lookahead()?.typ {
			TokenType::Semicolon => Expression::BooleanLiteral(true),
			_ => self.expression()?,
		};
		self.eat(TokenType::Semicolon)?;
		return Ok(result);
	}

	/*
	for_loop_increment_statement:
		| block_statement
		| empty_statement
		| expression
	*/
	fn for_loop_increment_statement(&mut self) -> ResultWithError<Statement> {
		return match self.lookahead()?.typ {
			TokenType::OpenBlock => self.block_statement(),
			TokenType::CloseParen => Ok(Statement::EmptyStatement),
			_ => Ok(Statement::ExpressionStatement(self.expression()?)),
		};
	}

	/*
	variable_declarations_statement:
		| 'let' variable_declarations ';'
	*/
	fn variable_declarations_statement(&mut self) -> ResultWithError<Statement> {
		self.eat(TokenType::Keyword(Keyword::Let))?;
		let res = self.delimited_items(
			Self::variable_declaration,
			TokenType::Comma,
			TokenType::Semicolon
		)?;
		if res.len() == 0 {
			return Err(ErrorT::ExpectedVariableDeclaration.into());
		}
		self.eat(TokenType::Semicolon)?;
		return Ok(Statement::VariableDeclarations(res));
	}

	/*
	variable_declaration:
		| Identifier
		| Identifier variable_initializer
	*/
	fn variable_declaration(&mut self) -> ResultWithError<VariableDeclaration> {
		let identifier = self.identifier()?;
		let initializer = match self.lookahead()?.typ {
			TokenType::Semicolon | TokenType::Comma => None,
			_ => Some(self.variable_initializer()?)
		};
		return Ok(VariableDeclaration::new(identifier, initializer));
	}

	/*
	variable_declaration:
		| '=' assignment_expression
	*/
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
		| relational_expression
		| lhs AssignmentOperator assignment_expression
	*/
	fn assignment_expression(&mut self) -> ResultWithError<Expression> {
		let left = self.base_binary_expression()?;
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

	binary_expressions!(
		fn base_binary_expression(&mut self) -> ResultWithError<Expression>{
			wrapper_function: left_to_right_binary_expression;
			logical_or_expresion: TokenType::LogicalOrOperator;
			logical_and_expresion: TokenType::LogicalAndOperator;
			equality_expression: TokenType::EqualityOperator;
			relational_expression: TokenType::RelationalOperator;
			additive_expression: TokenType::AdditiveOperator;
			multiplicative_expression: TokenType::MultiplicativeOperator;
			base_unary_expression: None;
		}
	);

	/*
	unary_expression:
		| primary_expression
		| AdditiveOperator unary_expression
		| LogicalNotOperator unary_expression
	*/
	fn base_unary_expression(&mut self) -> ResultWithError<Expression> {
		if !(self.lookahead()?.typ.is_unary_operator()) {
			return self.base_expression();
		}
		let operator = Operator::try_from(&self.eat_any()?.data)?;
		return Ok(Expression::unary_expression(operator, self.base_unary_expression()?.into()));
	}

	/*
	base_expression:
		| primary_expression
	 */
	fn base_expression(&mut self) -> ResultWithError<Expression> {
		return self.call_or_member_expression();
	}

	/*
	call_or_member_expression:
		| primary_expression
		| call_or_member_expression . Identifier
		| call_or_member_expression '[' expression ']'
		| call_or_member_expression '(' call_arguments ')'
	*/
	fn call_or_member_expression(&mut self) -> ResultWithError<Expression> {
		let mut res = self.primary_expression()?;
		while match self.lookahead()?.typ {
			TokenType::Dot | TokenType::OpenSquareBracket | TokenType::OpenParen => true,
			_ => false
		} {
			if self.lookahead()?.typ == TokenType::Dot {
				self.eat(TokenType::Dot)?;
				let property_name = self.identifier()?;
				res = Expression::member_property_access(res.into(), property_name);
			} else if self.lookahead()?.typ == TokenType::OpenSquareBracket {
				self.eat(TokenType::OpenSquareBracket)?;
				let expr = self.expression()?.into();
				self.eat(TokenType::CloseSquareBracket)?;

				res = Expression::member_subscript(res.into(), expr);
			} else if self.lookahead()?.typ == TokenType::OpenParen {
				self.eat(TokenType::OpenParen)?;
				let exprs = self.delimited_items(
					Self::expression,
					TokenType::Comma,
					TokenType::CloseParen
				)?;
				self.eat(TokenType::CloseParen)?;

				res = Expression::function_call(res.into(), exprs);
			} else {
				return Err(ErrorT::InvalidTokenType.into());
			}
		}
		return Ok(res);
	}

	/*
	primary_expression:
		| literal
		| parenthesized_expression
		| identifier
	*/
	fn primary_expression(&mut self) -> ResultWithError<Expression> {
		let token_type = self.lookahead()?.typ;
		if token_type.is_literal() {
			return self.literal();
		}
		return match token_type {
			TokenType::OpenParen => self.parenthesized_expression(),
			_ => self.identifier_expression(),
		};
	}

	fn identifier_expression(&mut self) -> ResultWithError<Expression> {
		return Ok(Expression::Identifier(self.identifier()?));
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
	literal:
		| integer_literal
		| string_literal
		| 'true'
		| 'false'
	*/
	fn literal(&mut self) -> ResultWithError<Expression> {
		return match self.lookahead()?.typ {
			TokenType::Integer => self.integer_literal(),
			TokenType::String => self.string_literal(),
			_ => self.singular_literal(),
		}
	}

	fn singular_literal(&mut self) -> ResultWithError<Expression> {
		let res = match self.lookahead()?.typ {
			TokenType::Keyword(Keyword::True) => Ok(Expression::BooleanLiteral(true)),
			TokenType::Keyword(Keyword::False) => Ok(Expression::BooleanLiteral(false)),
			TokenType::Keyword(Keyword::Null) => Ok(Expression::NullLiteral),
			_ => Err(ErrorT::InvalidTokenType.into()),
		};
		if res.is_ok() {
			self.eat_any()?;
		}
		return res;
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

	// region ...Utilities
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
	delimited_items:
		| item
		| item delimiter delimited_items
	*/
	fn delimited_items<T>(
		&mut self,
		item: fn(&mut Self) -> ResultWithError<T>,
		delimiter: TokenType,
		end: TokenType
	) -> ResultWithError<Vec<T>> {
		let mut res = Vec::<T>::new();
		if self.lookahead()?.typ != end {
			loop {
				res.push(item(self)?);
				if self.lookahead()?.typ == delimiter {
					self.eat(TokenType::Comma)?;
				} else {
					break;
				}
			}
		}
		return Ok(res);
	}
	// endregion
}
