use crate::ast::operator::Operator;
use crate::ast::statement::Statement;
use crate::ast::structs::CallExpression;

pub type BoxExpression = Box<Expression>;

pub type IdentifierT = String;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum MemberIndexer {
	PropertyName(IdentifierT),
	SubscriptExpression(BoxExpression),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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
}

impl Expression {
	pub fn binary_expression(operator: Operator, left: BoxExpression, right: BoxExpression) -> Expression {
		return Expression::BinaryExpression { operator, left, right };
	}

	pub fn unary_expression(operator: Operator, argument: BoxExpression) -> Expression {
		return Expression::UnaryExpression { operator, argument };
	}

	pub fn assignment_expression(operator: Operator, left: BoxExpression, right: BoxExpression) -> Expression {
		return Expression::AssignmentExpression { operator, left, right };
	}

	pub fn member_property_access(object: BoxExpression, property_name: IdentifierT) -> Expression {
		return Expression::MemberAccess { object, member: MemberIndexer::PropertyName(property_name) };
	}

	pub fn member_subscript(object: BoxExpression, expr: BoxExpression) -> Expression {
		return Expression::MemberAccess { object, member: MemberIndexer::SubscriptExpression(expr) };
	}

	pub fn function_call(function: BoxExpression, arguments: Vec<Expression>) -> Expression {
		return Expression::FunctionCall(CallExpression::new(function, arguments));
	}

	pub fn new_object_expression(class_expr: BoxExpression, arguments: Vec<Expression>) -> Expression {
		return Expression::NewObjectExpression(CallExpression::new(class_expr, arguments));
	}

	pub fn is_lhs(&self) -> bool {
		return match self {
			Expression::Identifier(_) => true,
			Expression::MemberAccess { .. } => true,
			_ => false,
		};
	}

	pub fn consume_as_statement(self) -> Statement {
		return Statement::from(self);
	}
}
