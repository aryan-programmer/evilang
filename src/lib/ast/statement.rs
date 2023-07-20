use crate::ast::expression::Expression;

pub type BoxStatement = Box<Statement>;

pub type StatementList = Vec<Statement>;

#[derive(Debug, Eq, PartialEq)]
pub enum Statement {
	BlockStatement(StatementList),
	EmptyStatement,
	ExpressionStatement(Expression),
	VariableDeclarations(Vec<VariableDeclaration>),
	IfStatement {
		condition: Expression,
		if_branch: BoxStatement,
		else_branch: Option<BoxStatement>,
	}
}

impl Statement {
	pub fn is_lhs(&self) -> bool {
		return match self {
			Statement::ExpressionStatement(ex) => ex.is_lhs(),
			_ => false,
		};
	}

	pub fn if_statement(
		condition: Expression,
		if_branch: BoxStatement,
		else_branch: Option<BoxStatement>,
	) -> Statement {
		return Statement::IfStatement {
			condition,
			if_branch,
			else_branch,
		}
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
