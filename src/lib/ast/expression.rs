use crate::ast::operator::Operator;
use crate::ast::statement::Statement;

pub type BoxExpression = Box<Expression>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expression {
	IntegerLiteral(i64),
	StringLiteral(String),
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
	Identifier(String),
}

impl Expression {
	pub fn binary_expression(operator: Operator, left: BoxExpression, right: BoxExpression) -> Expression {
		return Expression::BinaryExpression {
			operator,
			left,
			right,
		};
	}

	pub fn assignment_expression(operator: Operator, left: BoxExpression, right: BoxExpression) -> Expression {
		return Expression::AssignmentExpression {
			operator,
			left,
			right,
		};
	}

	pub fn is_lhs(&self) -> bool {
		return match self {
			Expression::Identifier(_) => true,
			_ => false,
		};
	}

	pub fn consume_as_statement(self) -> Statement {
		return Statement::from(self);
	}
}
