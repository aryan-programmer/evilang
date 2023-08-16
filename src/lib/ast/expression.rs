use num_traits::{Num};

use crate::ast::operator::Operator;
use crate::ast::statement::Statement;
use crate::ast::structs::{CallExpression, ClassDeclaration, FunctionDeclaration};
use crate::errors::ResultWithError;
use crate::math::number::NumberT;

pub type BoxExpression = Box<Expression>;

pub type IdentifierT = String;
pub type DottedIdentifiers = Vec<IdentifierT>;

#[derive(Debug, Clone, PartialEq)]
pub enum MemberIndexer {
	PropertyName(IdentifierT),
	SubscriptExpression(BoxExpression),
	MethodNameArrow(IdentifierT),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
	NullLiteral,
	BooleanLiteral(bool),
	NumericLiteral(NumberT),
	StringLiteral(String),
	ParenthesizedExpression(BoxExpression),
	UnaryExpression {
		operator: Operator,
		argument: BoxExpression,
	},
	BinaryExpression {
		operator: Operator,
		left: BoxExpression,
		right: BoxExpression,
	},
	AssignmentExpression {
		operator: Operator,
		left: BoxExpression,
		right: BoxExpression,
	},
	DottedIdentifiers(DottedIdentifiers),
	Identifier(IdentifierT),
	MemberAccess {
		object: BoxExpression,
		member: MemberIndexer,
	},
	FunctionCall(CallExpression),
	NewObjectExpression(CallExpression),
	FunctionExpression(FunctionDeclaration),
	ClassDeclarationExpression(Box<ClassDeclaration>),
}

impl Expression {
	pub fn numeric_literal(v: &str) -> ResultWithError<Expression> {
		return Ok(Expression::NumericLiteral(NumberT::from_str_radix(v, 10)?));
	}

	pub fn integer_literal(v: i64) -> Expression {
		return Expression::NumericLiteral(NumberT::Integer(v as i128));
	}

	pub fn float_literal(v: f64) -> Expression {
		return Expression::NumericLiteral(NumberT::Float(v));
	}

	#[inline(always)]
	pub fn binary_expression(operator: Operator, left: BoxExpression, right: BoxExpression) -> Expression {
		return Expression::BinaryExpression { operator, left, right };
	}

	#[inline(always)]
	pub fn unary_expression(operator: Operator, argument: BoxExpression) -> Expression {
		return Expression::UnaryExpression { operator, argument };
	}

	#[inline(always)]
	pub fn assignment_expression(operator: Operator, left: BoxExpression, right: BoxExpression) -> Expression {
		return Expression::AssignmentExpression { operator, left, right };
	}

	#[inline(always)]
	pub fn member_method_access(object: BoxExpression, property_name: IdentifierT) -> Expression {
		return Expression::MemberAccess { object, member: MemberIndexer::MethodNameArrow(property_name) };
	}

	#[inline(always)]
	pub fn member_property_access(object: BoxExpression, property_name: IdentifierT) -> Expression {
		return Expression::MemberAccess { object, member: MemberIndexer::PropertyName(property_name) };
	}

	#[inline(always)]
	pub fn member_subscript(object: BoxExpression, expr: BoxExpression) -> Expression {
		return Expression::MemberAccess { object, member: MemberIndexer::SubscriptExpression(expr) };
	}

	#[inline(always)]
	pub fn function_call(function: BoxExpression, arguments: Vec<Expression>) -> Expression {
		return Expression::FunctionCall(CallExpression::new(function, arguments));
	}

	#[inline(always)]
	pub fn new_object_expression(class_expr: BoxExpression, arguments: Vec<Expression>) -> Expression {
		return Expression::NewObjectExpression(CallExpression::new(class_expr, arguments));
	}

	#[inline(always)]
	pub fn is_lhs(&self) -> bool {
		return match self {
			Expression::Identifier(_) => true,
			Expression::MemberAccess { .. } => true,
			Expression::DottedIdentifiers(_) => true,
			_ => false,
		};
	}

	#[inline(always)]
	pub fn consume_as_statement(self) -> Statement {
		return Statement::from(self);
	}

	#[inline(always)]
	pub fn consume_as_parenthesized(self) -> Expression {
		return Expression::ParenthesizedExpression(self.into());
	}
}
