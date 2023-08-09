#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
	Captures,
	Return,
	Class,
	Extends,
	New,
	Super,
	// This,
	Break,
	Continue,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
	#[inline(always)]
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

	#[inline(always)]
	pub fn is_unary_operator(&self) -> bool {
		return match self {
			TokenType::AdditiveOperator |
			TokenType::LogicalNotOperator => true,
			_ => false,
		};
	}
}
