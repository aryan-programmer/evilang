use evilang_lib::ast::BoxStatement;
use evilang_lib::ast::Operator::{Minus, Multiplication, Plus};
use evilang_lib::ast::Statement::{BinaryExpression, IntegerLiteral};

use crate::common::{ensure_program, TestRes};

mod common;

#[test]
fn addition() -> TestRes {
	ensure_program("1+2;", vec![BinaryExpression {
		operator: Plus,
		left: BoxStatement::from(IntegerLiteral(1)),
		right: BoxStatement::from(IntegerLiteral(2)),
	}]);
}

#[test]
fn addition_and_subtraction() -> TestRes {
	ensure_program("1+2-3;", vec![BinaryExpression {
		operator: Minus,
		left: BoxStatement::from(BinaryExpression {
			operator: Plus,
			left: BoxStatement::from(IntegerLiteral(1)),
			right: BoxStatement::from(IntegerLiteral(2)),
		}),
		right: BoxStatement::from(IntegerLiteral(3)),
	}, ]);
}

#[test]
fn multiplication() -> TestRes {
	ensure_program("2*3;", vec![BinaryExpression {
		operator: Multiplication,
		left: BoxStatement::from(IntegerLiteral(2)),
		right: BoxStatement::from(IntegerLiteral(3)),
	}, ]);
}

#[test]
fn multiplication_2() -> TestRes {
	ensure_program("2*3*4;", vec![BinaryExpression {
		operator: Multiplication,
		left: BoxStatement::from(BinaryExpression {
			operator: Multiplication,
			left: BoxStatement::from(IntegerLiteral(2)),
			right: BoxStatement::from(IntegerLiteral(3)),
		}),
		right: BoxStatement::from(IntegerLiteral(4)),
	}, ]);
}

#[test]
fn addition_and_multiplication() -> TestRes {
	ensure_program("2+3*4;", vec![BinaryExpression {
		operator: Plus,
		left: BoxStatement::from(IntegerLiteral(2)),
		right: BoxStatement::from(BinaryExpression {
			operator: Multiplication,
			left: BoxStatement::from(IntegerLiteral(3)),
			right: BoxStatement::from(IntegerLiteral(4)),
		}),
	}, ]);

	ensure_program("2+3*4+5;", vec![BinaryExpression {
		operator: Plus,
		left: BoxStatement::from(BinaryExpression {
			operator: Plus,
			left: BoxStatement::from(IntegerLiteral(2)),
			right: BoxStatement::from(BinaryExpression {
				operator: Multiplication,
				left: BoxStatement::from(IntegerLiteral(3)),
				right: BoxStatement::from(IntegerLiteral(4)),
			}),
		}),
		right: BoxStatement::from(IntegerLiteral(5)),
	}, ]);
}

#[test]
fn parenthesis() -> TestRes {
	ensure_program("(2+3)*4;", vec![
		BinaryExpression {
			operator: Multiplication,
			left: BoxStatement::from(BinaryExpression {
				operator: Plus,
				left: BoxStatement::from(IntegerLiteral(2)),
				right: BoxStatement::from(IntegerLiteral(3)),
			}),
			right: BoxStatement::from(IntegerLiteral(4)),
		},
	]);
}
