pub type BoxStatement = Box<Statement>;

pub type StatementList = Vec<Statement>;

#[derive(Debug, Eq, PartialEq)]
pub enum Statement {
	IntegerLiteral(i64),
	StringLiteral(String),
	BlockStatement(StatementList),
	EmptyStatement,
}
