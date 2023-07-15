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
}

#[derive(Debug, Clone)]
pub struct EvilangError {
	pub typ: ErrorT,
	pub backtrace: Option<Backtrace>
}

impl Display for EvilangError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if let Some(trace) = &self.backtrace {
			write!(f, "{}: {:?}", error_type_to_string(self.typ), trace)
		} else {
			write!(f, "{}", error_type_to_string(self.typ))
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

static ERROR_TYPE_TO_STRING: Lazy<HashMap<ErrorT, String>> = Lazy::new(|| {
	return HashMap::from_iter([
		(ErrorT::EndOfTokenStream, "End of Token Stream"),
		(ErrorT::InvalidTokenType, "Invalid Token Type"),
		(ErrorT::TokenCannotBeParsed, "Token Cannot be Parsed"),
	].map(|t| (t.0, t.1.parse().unwrap())));
});
static UNKNOWN_ERROR: Lazy<String> = Lazy::new(|| "Unknown error".parse().unwrap());

pub fn error_type_to_string(et: ErrorT) -> &'static String {
	return ERROR_TYPE_TO_STRING.get(&et).unwrap_or(&UNKNOWN_ERROR);
}
