use std::error::Error;
use std::fmt::{Display, Formatter};

use backtrace::Backtrace;
use thiserror::Error;

use crate::ast::expression::Expression;
use crate::ast::operator::Operator;
use crate::ast::statement::Statement;
use crate::interpreter::runtime_value::PrimitiveValue;

pub type ResultWithError<T, E = EvilangError> = anyhow::Result<T, E>;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Error)]
pub enum ErrorT {
	#[error("This error should never happen")]
	NeverError,
	#[error("End of Token Stream")]
	EndOfTokenStream,
	#[error("Invalid Token Type")]
	InvalidTokenType,
	#[error("Token Cannot be Parsed")]
	TokenCannotBeParsed,
	#[error("Unknown Operator")]
	UnknownOperator,
	#[error("Expected a left hand side expression")]
	ExpectedLhsExpression,
	#[error("Expected a simple assignment operator")]
	ExpectedSimpleAssignmentOperator,
	#[error("Expected at least one variable in declaration")]
	ExpectedVariableDeclaration,
	#[error("The interpreter does not support this statement type: {0:#?}")]
	UnimplementedStatementTypeForInterpreter(Statement),
	#[error("The interpreter does not support this expression type: {0:#?}")]
	UnimplementedExpressionTypeForInterpreter(Expression),
	#[error("The interpreter does not support the operator: {0:?} for the values {1:#?} and {2:#?}")]
	UnimplementedBinaryOperatorForValues(Operator, PrimitiveValue, PrimitiveValue),
	#[error("The interpreter does not support the unary operator: {0:?} for the value {1:#?}")]
	UnimplementedUnaryOperatorForValues(Operator, PrimitiveValue),
	#[error("The following function is not implemented: {0:?}")]
	UnimplementedFunction(Expression),
	#[error("The cannot mutably borrow an r-value reference")]
	InvalidMutableBorrowForRValue,
	#[error("A mutable borrow already exists")]
	InvalidBorrow,
	#[error("The cannot strip assignment from operator: {0:?}")]
	CantStripAssignment(Operator),
}

#[derive(Debug, Clone)]
pub struct EvilangError {
	pub typ: ErrorT,
	pub backtrace: Option<Backtrace>,
}

impl Display for EvilangError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if let Some(trace) = &self.backtrace {
			write!(f, "{}: {:?}", self.typ, trace)
		} else {
			write!(f, "{}", self.typ)
		}
	}
}

impl Error for EvilangError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		return Some(&self.typ);
	}
}

impl From<ErrorT> for EvilangError {
	fn from(value: ErrorT) -> Self {
		return EvilangError::new(value);
	}
}

impl EvilangError {
	pub fn new(typ: ErrorT) -> EvilangError {
		return EvilangError { typ, backtrace: Some(Backtrace::new()) };
	}
}

pub fn ensure(v: bool, err: ErrorT) -> ResultWithError<()> {
	return if v { Ok(()) } else { Err(err.into()) };
}
