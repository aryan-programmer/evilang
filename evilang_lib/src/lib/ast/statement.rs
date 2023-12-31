use crate::ast::expression::{DottedIdentifiers, Expression, IdentifierT};
use crate::ast::structs::{ClassDeclaration, FunctionDeclaration, FunctionParameterDeclaration, VariableDeclaration};

pub type BoxStatement = Box<Statement>;

pub type StatementList = Vec<Statement>;

#[derive(Debug, Clone, PartialEq)]
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
	BreakStatement(i64),
	ContinueStatement(i64),
	FunctionDeclarationStatement(FunctionDeclaration),
	ClassDeclarationStatement(ClassDeclaration),
	NamespaceStatement {
		namespace: DottedIdentifiers,
		body: StatementList,
	},
	ImportStatement {
		file_name: Expression,
		as_object: DottedIdentifiers,
	},
}

impl Statement {
	#[inline(always)]
	pub fn if_statement(
		condition: Expression,
		if_branch: BoxStatement,
		else_branch: Option<BoxStatement>,
	) -> Statement {
		return Statement::IfStatement { condition, if_branch, else_branch };
	}

	#[inline(always)]
	pub fn while_loop(condition: Expression, body: BoxStatement) -> Statement {
		return Statement::WhileLoop { condition, body };
	}

	#[inline(always)]
	pub fn do_while_loop(condition: Expression, body: BoxStatement) -> Statement {
		return Statement::DoWhileLoop { condition, body };
	}

	#[inline(always)]
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

	#[inline(always)]
	pub fn function_declaration(
		name: IdentifierT,
		parameters: Vec<FunctionParameterDeclaration>,
		body: BoxStatement,
	) -> Statement {
		return Statement::FunctionDeclarationStatement(FunctionDeclaration::new(name, parameters, body));
	}

	#[inline(always)]
	pub fn class_declaration(
		name: IdentifierT,
		super_class: Option<Expression>,
		methods: Vec<FunctionDeclaration>,
	) -> Statement {
		return Statement::ClassDeclarationStatement(ClassDeclaration::new(name, super_class, methods));
	}

	#[inline(always)]
	pub fn namespace_statement(
		namespace: DottedIdentifiers,
		body: StatementList,
	) -> Statement {
		return Statement::NamespaceStatement { namespace, body };
	}

	#[inline(always)]
	pub fn import_statement(
		file_name: Expression,
		as_object: DottedIdentifiers,
	) -> Statement {
		return Statement::ImportStatement { file_name, as_object };
	}
}

impl From<Expression> for Statement {
	#[inline(always)]
	fn from(value: Expression) -> Self {
		return Statement::ExpressionStatement(value);
	}
}
