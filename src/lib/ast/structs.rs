use crate::ast::expression::{BoxExpression, Expression, IdentifierT};
use crate::ast::statement::BoxStatement;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VariableDeclaration {
	pub identifier: IdentifierT,
	pub initializer: Option<Expression>,
}

impl VariableDeclaration {
	#[inline(always)]
	pub fn new(identifier: IdentifierT, initializer: Option<Expression>) -> Self {
		Self { identifier, initializer }
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionParameterDeclaration {
	pub identifier: IdentifierT,
}

impl FunctionParameterDeclaration {
	#[inline(always)]
	pub fn new(identifier: IdentifierT) -> Self {
		Self { identifier }
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionDeclaration {
	pub name: IdentifierT,
	pub parameters: Vec<FunctionParameterDeclaration>,
	pub body: BoxStatement,
}

impl FunctionDeclaration {
	#[inline(always)]
	pub fn new(name: IdentifierT, parameters: Vec<FunctionParameterDeclaration>, body: BoxStatement) -> Self {
		Self { name, parameters, body }
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CallExpression {
	pub callee: BoxExpression,
	pub arguments: Vec<Expression>,
}

impl CallExpression {
	#[inline(always)]
	pub fn new(callee: BoxExpression, arguments: Vec<Expression>) -> Self {
		Self { callee, arguments }
	}
}

