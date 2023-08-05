use crate::ast::expression::{BoxExpression, Expression, IdentifierT};
use crate::ast::statement::BoxStatement;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct VariableDeclaration {
	pub identifier: IdentifierT,
	pub initializer: Option<Expression>,
}

impl VariableDeclaration {
	pub fn new(identifier: IdentifierT, initializer: Option<Expression>) -> Self {
		Self { identifier, initializer }
	}
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct FunctionParameterDeclaration {
	pub identifier: IdentifierT,
}

impl FunctionParameterDeclaration {
	pub fn new(identifier: IdentifierT) -> Self {
		Self { identifier }
	}
}

// #[derive(Debug, Clone, Hash, Eq, PartialEq)]
// pub struct FunctionCaptureDeclaration {
// 	pub identifier: IdentifierT,
// }
//
// impl FunctionCaptureDeclaration {
// 	pub fn new(identifier: IdentifierT) -> Self {
// 		Self { identifier }
// 	}
// }

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct FunctionDeclaration {
	pub name: IdentifierT,
	pub parameters: Vec<FunctionParameterDeclaration>,
	pub body: BoxStatement,
	// pub captures: Vec<FunctionCaptureDeclaration>,
}

impl FunctionDeclaration {
	pub fn new(name: IdentifierT, parameters: Vec<FunctionParameterDeclaration>, body: BoxStatement) -> Self {
		Self { name, parameters, body/*, captures: vec![] */ }
	}

	// pub fn new_closure(
	// 	name: IdentifierT,
	// 	parameters: Vec<FunctionParameterDeclaration>,
	// 	body: BoxStatement,
	// 	captures: Vec<FunctionCaptureDeclaration>,
	// ) -> Self {
	// 	Self { name, parameters, body, captures }
	// }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CallExpression {
	pub callee: BoxExpression,
	pub arguments: Vec<Expression>,
}

impl CallExpression {
	pub fn new(callee: BoxExpression, arguments: Vec<Expression>) -> Self {
		Self { callee, arguments }
	}
}

