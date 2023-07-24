use crate::ast::expression::{Expression, IdentifierT};
use crate::ast::structs::{FunctionParameterDeclaration, VariableDeclaration};

pub type BoxStatement = Box<Statement>;

pub type StatementList = Vec<Statement>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Statement {
	BlockStatement(StatementList),
	EmptyStatement,
	ExpressionStatement(Expression),
	ReturnStatement(Option<Expression>),
	VariableDeclarations(Vec<VariableDeclaration>),
	IfStatement {
		condition: Expression,
		if_branch: BoxStatement,
		else_branch: Option<BoxStatement>,
	},
	WhileLoop {
		condition: Expression,
		body: BoxStatement,
	},
	DoWhileLoop {
		condition: Expression,
		body: BoxStatement,
	},
	ForLoop {
		initialization: BoxStatement,
		condition: Expression,
		increment: BoxStatement,
		body: BoxStatement,
	},
	FunctionDeclaration {
		name: IdentifierT,
		parameters: Vec<FunctionParameterDeclaration>,
		body: BoxStatement,
	},
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

	pub fn while_loop(condition: Expression, body: BoxStatement) -> Statement {
		return Statement::WhileLoop { condition, body };
	}

	pub fn do_while_loop(condition: Expression, body: BoxStatement) -> Statement {
		return Statement::DoWhileLoop { condition, body };
	}

	pub fn for_loop(
		initialization: BoxStatement,
		condition: Expression,
		increment: BoxStatement,
		body: BoxStatement,
	) -> Statement {
		return Statement::ForLoop {
			initialization,
			condition,
			increment,
			body,
		};
	}

	pub fn function_declaration(
		name: IdentifierT,
		parameters: Vec<FunctionParameterDeclaration>,
		body: BoxStatement,
	) -> Statement {
		return Statement::FunctionDeclaration { name, parameters, body };
	}
}

impl From<Expression> for Statement {
	fn from(value: Expression) -> Self {
		return Statement::ExpressionStatement(value);
	}
}
