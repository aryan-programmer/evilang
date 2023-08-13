use std::error::Error;
use std::fmt::{Display, Formatter};

use backtrace::Backtrace;
use thiserror::Error;

use crate::ast::expression::{Expression, IdentifierT};
use crate::ast::operator::Operator;
use crate::ast::statement::Statement;
use crate::interpreter::environment::statement_result::StatementExecution;
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::tokenizer::Token;

pub type ResultWithError<T, E = EvilangError> = anyhow::Result<T, E>;

#[derive(Debug, Clone, PartialEq)]
pub enum Descriptor {
	None,
	Name(IdentifierT),
	Value(PrimitiveValue),
	Expression(Expression),
	Both { value: PrimitiveValue, expression: Expression },
}

impl From<&str> for Descriptor {
	fn from(value: &str) -> Self {
		Descriptor::Name(value.to_string())
	}
}

impl From<IdentifierT> for Descriptor {
	fn from(value: IdentifierT) -> Self {
		Descriptor::Name(value)
	}
}

impl From<PrimitiveValue> for Descriptor {
	fn from(value: PrimitiveValue) -> Self {
		Descriptor::Value(value)
	}
}

impl From<Expression> for Descriptor {
	fn from(value: Expression) -> Self {
		Descriptor::Expression(value)
	}
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum RuntimeError {
	#[error("{0}")]
	GenericError(String),
	#[error("Expected {0:#?} to be a function")]
	ExpectedFunction(Descriptor),
	#[error("Expected {0:#?} to be a class object")]
	ExpectedClassObject(Descriptor),
	#[error("Expected {0:#?} to be an object")]
	ExpectedObject(Descriptor),
	#[error("Invalid number of arguments {got:?} expected {expected:?} to be function {func:#?}")]
	InvalidNumberArgumentsToFunction { got: usize, expected: Option<String>, func: Descriptor },
	#[error("Expected {0:#?} to be a valid subscript expression")]
	ExpectedValidSubscript(Descriptor),
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ErrorT {
	#[error("This error should never happen: {0}")]
	NeverError(String),
	#[error(transparent)]
	UnexpectedRuntimeError(#[from] RuntimeError),
	#[error("End of Token Stream")]
	EndOfTokenStream,
	#[error("Invalid Token Type: {0:#?}")]
	InvalidTokenType(Token),
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
	UnimplementedBinaryOperatorForValues(Operator, Expression, Expression),
	#[error("The interpreter does not support the unary operator: {0:?} for the value {1:#?}")]
	UnimplementedUnaryOperatorForValues(Operator, Expression),
	#[error("A mutable borrow already exists")]
	InvalidBorrow,
	#[error("The cannot strip assignment from operator: {0:?}")]
	CantStripAssignment(Operator),
	#[error("Can't access variable '{0:?}' before the point in time at which it has been declared")]
	CantAccessHoistedVariable(IdentifierT),
	#[error("Can't declare variable '{0:?}' since it already exists in this scope")]
	CantRedeclareVariable(IdentifierT),
	#[error("Can't set a variable to be hoisted")]
	CantSetToHoistedValue,
	#[error("Invalid unrolling from function {0:?}: {1:#?}")]
	InvalidUnrollingOfFunction(IdentifierT, StatementExecution),
	#[error("Member functions accessed by the arrow notation mus be immediately called: {0:#?}")]
	InvalidMethodArrowAccess(Expression),
}

#[derive(Debug, Clone)]
pub struct EvilangError {
	pub typ: ErrorT,
	pub backtrace: Option<Backtrace>,
}

impl Display for EvilangError {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if let Some(trace) = &self.backtrace {
			write!(f, "{}: {:?}", self.typ, trace)
		} else {
			write!(f, "{}", self.typ)
		}
	}
}

impl Error for EvilangError {
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		return Some(&self.typ);
	}
}

impl From<ErrorT> for EvilangError {
	#[inline(always)]
	fn from(value: ErrorT) -> Self {
		return EvilangError::new(value);
	}
}

impl From<RuntimeError> for EvilangError {
	#[inline(always)]
	fn from(value: RuntimeError) -> Self {
		return EvilangError::new(ErrorT::UnexpectedRuntimeError(value));
	}
}

impl EvilangError {
	#[inline(always)]
	pub fn new(typ: ErrorT) -> EvilangError {
		return EvilangError { typ, backtrace: Some(Backtrace::new()) };
	}
}

#[inline(always)]
pub fn ensure(v: bool, err: ErrorT) -> ResultWithError<()> {
	return if v { Ok(()) } else { Err(err.into()) };
}
