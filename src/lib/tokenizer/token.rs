#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Keyword {
	Let,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TokenType {
	Integer,
	String,
	Semicolon,
	OpenBlock,
	CloseBlock,
	MultiplicativeOperator,
	AdditiveOperator,
	OpenParen,
	CloseParen,
	Identifier,
	AssignmentOperator,
	Comma,
	Keyword(Keyword),
}
