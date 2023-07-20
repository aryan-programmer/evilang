#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Keyword {
	Let,
	If,
	Else,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TokenType {
	_EOFDummy,

	Integer,
	String,
	Semicolon,
	OpenBlock,
	CloseBlock,
	MultiplicativeOperator,
	AdditiveOperator,
	RelationalOperator,
	OpenParen,
	CloseParen,
	Identifier,
	AssignmentOperator,
	Comma,
	Keyword(Keyword),
}

impl TokenType {
	pub fn is_literal(&self) -> bool {
		return match self {
			TokenType::String | TokenType::Integer => true,
			_ => false,
		};
	}
}
