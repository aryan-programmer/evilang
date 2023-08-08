use crate::ast::operator::Operator;
use crate::ast::statement::Statement;
use crate::ast::structs::{CallExpression, FunctionDeclaration};

pub type BoxExpression = Box<Expression>;

pub type IdentifierT = String;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MemberIndexer {
	PropertyName(IdentifierT),
	SubscriptExpression(BoxExpression),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expression {
	NullLiteral,
	BooleanLiteral(bool),
	IntegerLiteral(i64),
	StringLiteral(String),
	ThisExpression,
	SuperExpression,
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
	Identifier(IdentifierT),
	MemberAccess {
		object: BoxExpression,
		member: MemberIndexer,
	},
	FunctionCall(CallExpression),
	NewObjectExpression(CallExpression),
	FunctionExpression(FunctionDeclaration),
}

impl Expression {
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
			_ => false,
		};
	}

	#[inline(always)]
	pub fn consume_as_statement(self) -> Statement {
		return Statement::from(self);
	}
}
