use crate::errors::{ErrorT, EvilangError};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
	LessThan,
	GreaterThan,
	LessThanOrEqualTo,
	GreaterThanOrEqualTo,
	Equals,
	NotEquals,
	LogicalAnd,
	LogicalOr,
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
			"<" => Ok(Operator::LessThan),
			">" => Ok(Operator::GreaterThan),
			"<=" => Ok(Operator::LessThanOrEqualTo),
			">=" => Ok(Operator::GreaterThanOrEqualTo),
			"==" => Ok(Operator::Equals),
			"!=" => Ok(Operator::NotEquals),
			"&&" => Ok(Operator::LogicalAnd),
			"||" => Ok(Operator::LogicalOr),
			_ => Err(ErrorT::UnknownOperator.into())
		};
	}
}
