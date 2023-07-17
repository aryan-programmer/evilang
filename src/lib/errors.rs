use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use backtrace::Backtrace;
use once_cell::sync::Lazy;
use thiserror::Error;

pub type ResultWithError<T, E = EvilangError> = anyhow::Result<T, E>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Error)]
pub enum ErrorT {
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
}

#[derive(Debug, Clone)]
pub struct EvilangError {
	pub typ: ErrorT,
	pub backtrace: Option<Backtrace>
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
		return EvilangError::new(value)
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
