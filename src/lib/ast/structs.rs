use crate::ast::expression::{Expression, IdentifierT};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VariableDeclaration {
	pub identifier: IdentifierT,
	pub initializer: Option<Expression>,
}

impl VariableDeclaration {
	pub fn new(identifier: IdentifierT, initializer: Option<Expression>) -> Self {
		Self { identifier, initializer }
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionParameterDeclaration {
	pub identifier: IdentifierT,
}

impl FunctionParameterDeclaration {
	pub fn new(identifier: IdentifierT) -> Self {
		Self { identifier }
	}
}

