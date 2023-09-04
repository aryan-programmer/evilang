use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;

use backtrace::Backtrace;
use maybe_owned::MaybeOwned;
use thiserror::Error;

use crate::ast::expression::{Expression, IdentifierT};
use crate::ast::operator::Operator;
use crate::ast::statement::Statement;
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::tokenizer::Token;
use crate::types::string::StringT;

pub type ResultWithError<T, E = EvilangError> = anyhow::Result<T, E>;

#[derive(Debug, PartialEq)]
pub enum Descriptor {
	None,
	Name(IdentifierT),
	Value(PrimitiveValue),
	Expression(Expression),
	ExpressionAndValue { value: PrimitiveValue, expression: Expression },
	NameAndValue { name: IdentifierT, value: PrimitiveValue },
}

impl Descriptor {
	pub fn new_both(value: MaybeOwned<PrimitiveValue>, expression: MaybeOwned<Expression>) -> Descriptor {
		return Descriptor::ExpressionAndValue {
			value: Self::value_to_owned(value),
			expression: expression.into_owned(),
		};
	}

	fn value_to_owned(value: MaybeOwned<PrimitiveValue>) -> PrimitiveValue {
		match value {
			MaybeOwned::Owned(v) => v,
			MaybeOwned::Borrowed(v) => v.clone__silently_fail()
		}
	}

	pub fn with_value(self, value: MaybeOwned<PrimitiveValue>) -> Descriptor {
		let v = Self::value_to_owned(value);
		return match self {
			Descriptor::None => Descriptor::Value(v),
			Descriptor::Name(name) => Descriptor::NameAndValue { name, value: v },
			Descriptor::Value(_) => Descriptor::Value(v),
			Descriptor::Expression(expression) => Descriptor::ExpressionAndValue { expression, value: v },
			Descriptor::ExpressionAndValue { expression, value: _ } => Descriptor::ExpressionAndValue { expression, value: v },
			Descriptor::NameAndValue { name, value: _ } => Descriptor::NameAndValue { name, value: v },
		};
	}
}

impl Clone for Descriptor {
	#[inline]
	fn clone(&self) -> Descriptor {
		match self {
			Descriptor::None => Descriptor::None,
			Descriptor::Name(name) => {
				Descriptor::Name(Clone::clone(name))
			}
			Descriptor::Value(value) => {
				Descriptor::Value(value.clone__silently_fail())
			}
			Descriptor::Expression(expr) => {
				Descriptor::Expression(Clone::clone(expr))
			}
			Descriptor::ExpressionAndValue { value, expression } => {
				Descriptor::ExpressionAndValue {
					value: value.clone__silently_fail(),
					expression: Clone::clone(expression),
				}
			}
			Descriptor::NameAndValue { name, value } => {
				Descriptor::NameAndValue {
					name: Clone::clone(name),
					value: value.clone__silently_fail(),
				}
			}
		}
	}
}

impl From<&str> for Descriptor {
	#[inline(always)]
	fn from(value: &str) -> Self {
		Descriptor::Name(value.into())
	}
}

impl From<IdentifierT> for Descriptor {
	#[inline(always)]
	fn from(value: IdentifierT) -> Self {
		Descriptor::Name(value)
	}
}

impl From<PrimitiveValue> for Descriptor {
	#[inline(always)]
	fn from(value: PrimitiveValue) -> Self {
		Descriptor::Value(value)
	}
}

impl From<Expression> for Descriptor {
	#[inline(always)]
	fn from(value: Expression) -> Self {
		Descriptor::Expression(value)
	}
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum RuntimeError {
	#[error("{0}")]
	GenericError(StringT),
	#[error("Expected {0:#?} to not be null")]
	UnexpectedNullValue(Descriptor),
	#[error("Expected {0:#?} to be a boolean")]
	ExpectedBoolean(Descriptor),
	#[error("Expected {0:#?} to be a number")]
	ExpectedNumber(Descriptor),
	#[error("Expected {0:#?} to be a string")]
	ExpectedString(Descriptor),
	#[error("Expected {0:#?} to be a function")]
	ExpectedFunction(Descriptor),
	#[error("Expected {0:#?} to be a class object")]
	ExpectedClassObject(Descriptor),
	#[error("Expected {0:#?} to be a namespace object")]
	ExpectedNamespaceObject(Descriptor),
	#[error("Expected {0:#?} to be an object")]
	ExpectedObject(Descriptor),
	#[error("Expected {0:#?} to be a native struct object")]
	ExpectedNativeObject(Descriptor),
	#[error("Invalid arguments {0:#?}: {1:#?}")]
	InvalidArgumentsToFunction(String, Descriptor),
	#[error("Invalid number of arguments {got:?} expected {expected:?} to be function {func:#?}")]
	InvalidNumberArgumentsToFunction { got: usize, expected: Option<StringT>, func: Descriptor },
	#[error("Expected {0:#?} to be a valid subscript expression")]
	ExpectedValidSubscript(Descriptor),
	#[error("Expected {0:#?} to be a valid file name expression")]
	ExpectedValidFileName(Descriptor),
	#[error("{0}")]
	IOError(StringT),
	#[error("Expression can not be cloned: {0:#?}")]
	CantCloneSafely(Descriptor),
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ErrorT {
	#[error("This error should never happen: {0}")]
	NeverError(StringT),
	#[error(transparent)]
	UnexpectedRuntimeError(#[from] RuntimeError),
	#[error("End of Token Stream")]
	EndOfTokenStream,
	#[error("Invalid Token Type: {0:#?}")]
	InvalidTokenType(Token),
	#[error("Token Cannot be Parsed")]
	TokenCannotBeParsed,
	#[error("Invalid numeric literal: {0}")]
	InvalidNumericLiteral(StringT),
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
	InvalidUnrollingOfFunction(IdentifierT, StringT),
	#[error("Member functions accessed by the arrow notation mus be immediately called: {0:#?}")]
	InvalidMethodArrowAccess(Expression),
}

#[derive(Debug, Clone)]
pub struct EvilangError {
	pub typ: ErrorT,
	pub backtrace: Option<Backtrace>,
}

impl From<io::Error> for EvilangError {
	#[inline(always)]
	fn from(value: io::Error) -> Self {
		return EvilangError::new(RuntimeError::IOError(format!("{0}", value)).into());
	}
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
