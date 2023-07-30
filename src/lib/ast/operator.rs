use crate::errors::{ErrorT, EvilangError, ResultWithError};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
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
	LogicalNot,
}

impl Operator {
	pub fn is_assignment(&self) -> bool {
		return match self {
			Operator::Assignment |
			Operator::PlusAssignment |
			Operator::MinusAssignment |
			Operator::MultiplicationAssignment |
			Operator::DivisionAssignment |
			Operator::ModulusAssignment => true,
			_ => false,
		};
	}

	pub fn strip_assignment(&self) -> ResultWithError<Operator> {
		return match self {
			Operator::Assignment => Err(ErrorT::CantStripAssignment(Operator::Assignment).into()),
			Operator::PlusAssignment => Ok(Operator::Plus),
			Operator::MinusAssignment => Ok(Operator::Minus),
			Operator::MultiplicationAssignment => Ok(Operator::Multiplication),
			Operator::DivisionAssignment => Ok(Operator::Division),
			Operator::ModulusAssignment => Ok(Operator::Modulus),
			v => Err(ErrorT::CantStripAssignment(v.clone()).into()),
		};
	}
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
			"!" => Ok(Operator::LogicalNot),
			_ => Err(ErrorT::UnknownOperator.into())
		};
	}
}
