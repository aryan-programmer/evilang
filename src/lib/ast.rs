use crate::errors::{ErrorT, EvilangError};

pub type BoxStatement = Box<Statement>;

pub type StatementList = Vec<Statement>;

#[derive(Debug, Eq, PartialEq)]
pub enum Statement {
	IntegerLiteral(i64),
	StringLiteral(String),
	BlockStatement(StatementList),
	EmptyStatement,
	BinaryExpression {
		operator: Operator,
		left: BoxStatement,
		right: BoxStatement,
	},
	AssignmentExpression {
		operator: Operator,
		left: BoxStatement,
		right: BoxStatement,
	},
	Identifier(String),
}

impl Statement {
	pub fn binary_expression(operator: Operator, left: BoxStatement, right: BoxStatement) -> Statement {
		return Statement::BinaryExpression {
			operator,
			left,
			right,
		};
	}

	pub fn assignment_expression(operator: Operator, left: BoxStatement, right: BoxStatement) -> Statement {
		return Statement::AssignmentExpression {
			operator,
			left,
			right,
		};
	}

	pub fn is_lhs(&self) -> bool {
		return match self {
			Statement::Identifier(_) => true,
			_ => false,
		};
	}
}

#[derive(Debug, Eq, PartialEq)]
pub enum Operator {
	Plus,
	Minus,
	Multiplication,
	Division,
	Modulus,
	Assignment,
	PlusAssignment,
	MinusAssignment,
	MultiplicationAssignment,
	DivisionAssignment,
	ModulusAssignment,
}

impl TryFrom<&String> for Operator {
	type Error = EvilangError;

	fn try_from(value: &String) -> Result<Self, Self::Error> {
		return match value.as_str() {
			"+" => Ok(Operator::Plus),
			"-" => Ok(Operator::Minus),
			"*" => Ok(Operator::Multiplication),
			"/" => Ok(Operator::Division),
			"%" => Ok(Operator::Modulus),
			"=" => Ok(Operator::Assignment),
			"+=" => Ok(Operator::PlusAssignment),
			"-=" => Ok(Operator::MinusAssignment),
			"*=" => Ok(Operator::MultiplicationAssignment),
			"/=" => Ok(Operator::DivisionAssignment),
			"%=" => Ok(Operator::ModulusAssignment),
			_ => Err(ErrorT::UnknownOperator.into())
		};
	}
}
