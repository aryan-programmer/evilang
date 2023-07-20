use crate::ast::expression::Expression;

pub type BoxStatement = Box<Statement>;

pub type StatementList = Vec<Statement>;

#[derive(Debug, Eq, PartialEq)]
pub enum Statement {
	BlockStatement(StatementList),
	EmptyStatement,
	ExpressionStatement(Expression),
	VariableDeclarations(Vec<VariableDeclaration>),
}

impl Statement {
	pub fn is_lhs(&self) -> bool {
		return match self {
			Statement::ExpressionStatement(ex) => ex.is_lhs(),
			_ => false,
		};
	}
}

impl From<Expression> for Statement {
	fn from(value: Expression) -> Self {
		return Statement::ExpressionStatement(value);
	}
}

#[derive(Debug, Eq, PartialEq)]
pub struct VariableDeclaration {
	pub identifier: String,
	pub initializer: Option<Expression>,
}
