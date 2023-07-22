#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Keyword {
	Let,
	If,
	Else,
	True,
	False,
	Null,
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
	EqualityOperator,
	LogicalAndOperator,
	LogicalOrOperator,
	AssignmentOperator,
	OpenParen,
	CloseParen,
	Identifier,
	Comma,
	Keyword(Keyword),
}

impl TokenType {
	pub fn is_literal(&self) -> bool {
		return match self {
			TokenType::String |
			TokenType::Integer |
			TokenType::Keyword(
				Keyword::True |
				Keyword::False |
				Keyword::Null
			) => true,
			_ => false,
		};
	}
}
