#[derive(Debug, Copy, Clone, PartialEq)]
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
	Break,
	Continue,
	Namespace,
	Import,
	As,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
	_EOFDummy,

	Number,
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
	Arrow,
	Keyword(Keyword),
}

impl TokenType {
	#[inline(always)]
	pub fn is_literal(&self) -> bool {
		return match self {
			TokenType::String |
			TokenType::Number |
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
