#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Keyword {
	Let,
	If,
	Else,
	True,
	False,
	Null,
	While,
	Do,
	For,
	Fn,
	Return,
	Class,
	Extends,
	New,
	Super,
	This,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TokenType {
	_EOFDummy,

	//	Number,
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
	LogicalNotOperator,
	AssignmentOperator,
	OpenParen,
	CloseParen,
	OpenSquareBracket,
	CloseSquareBracket,
	Identifier,
	Comma,
	Dot,
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

	pub fn is_unary_operator(&self) -> bool {
		return match self {
			TokenType::AdditiveOperator |
			TokenType::LogicalNotOperator => true,
			_ => false,
		};
	}
}
