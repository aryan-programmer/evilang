use crate::errors::{ErrorT, EvilangError};

pub type BoxStatement = Box<Statement>;

pub type StatementList = Vec<Statement>;

#[derive(Debug, Eq, PartialEq)]
pub enum Statement {
	IntegerLiteral(i64),
	StringLiteral(String),
	BlockStatement(StatementList),
	EmptyStatement,
	BinaryExpression { operator: Operator, left: BoxStatement, right: BoxStatement },
}

#[derive(Debug, Eq, PartialEq)]
pub enum Operator {
	Plus,
	Minus,
	Multiplication,
	Division,
	Modulus,
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
			_ => Err(ErrorT::UnknownOperator.into())
		};
	}
}
