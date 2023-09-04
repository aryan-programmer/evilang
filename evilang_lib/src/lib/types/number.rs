use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num_traits::{Num, One, Zero};

use crate::errors::{ErrorT, EvilangError};

#[derive(Debug, Clone)]
pub enum NumberT {
	Integer(i128),
	Float(f64),
}

impl From<i128> for NumberT {
	#[inline(always)]
	fn from(value: i128) -> Self {
		NumberT::Integer(value)
	}
}

impl From<f64> for NumberT {
	#[inline(always)]
	fn from(value: f64) -> Self {
		NumberT::Float(value)
	}
}

impl Copy for NumberT {}

impl PartialOrd for NumberT {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		return match (self, other) {
			(NumberT::Integer(a), NumberT::Integer(b)) => a.partial_cmp(b),
			(NumberT::Float(a), NumberT::Float(b)) => a.to_float_64().partial_cmp(b),
			(NumberT::Integer(a), NumberT::Float(b)) => a.to_float_64().partial_cmp(b),
			(NumberT::Float(a), NumberT::Integer(b)) => a.to_float_64().partial_cmp(&b.to_float_64()),
		};
	}
}

impl Neg for NumberT {
	type Output = NumberT;

	#[inline(always)]
	fn neg(self) -> Self::Output {
		match self {
			NumberT::Integer(v) => NumberT::Integer(-v),
			NumberT::Float(v) => NumberT::Float(-v),
		}
	}
}

impl Display for NumberT {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			NumberT::Integer(v) => std::fmt::Display::fmt(v, f),
			NumberT::Float(v) => std::fmt::Display::fmt(v, f),
		}
	}
}

trait Helpers {
	fn to_float_64(&self) -> f64;
}

impl Helpers for i128 {
	#[inline(always)]
	fn to_float_64(&self) -> f64 {
		*self as f64
	}
}

impl Helpers for f64 {
	#[inline(always)]
	fn to_float_64(&self) -> f64 {
		*self
	}
}

impl PartialEq for NumberT {
	fn eq(&self, other: &Self) -> bool {
		return match (self, other) {
			(NumberT::Integer(a), NumberT::Integer(b)) => a == b,
			(NumberT::Float(a), NumberT::Float(b)) => a.to_float_64() == b.to_float_64(),
			(NumberT::Integer(a), NumberT::Float(b)) => a.to_float_64() == b.to_float_64(),
			(NumberT::Float(a), NumberT::Integer(b)) => a.to_float_64() == b.to_float_64(),
		};
	}
}

impl Zero for NumberT {
	#[inline(always)]
	fn zero() -> Self {
		NumberT::Integer(0)
	}

	#[inline(always)]
	fn is_zero(&self) -> bool {
		match self {
			NumberT::Integer(v) => *v == 0,
			NumberT::Float(v) => *v == 0.0,
		}
	}
}

impl Add<Self> for NumberT {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		return match (self, rhs) {
			(NumberT::Integer(a), NumberT::Integer(b)) => NumberT::Integer(a + b),
			(NumberT::Float(a), NumberT::Float(b)) => NumberT::Float(a.to_float_64() + b.to_float_64()),
			(NumberT::Integer(a), NumberT::Float(b)) => NumberT::Float(a.to_float_64() + b.to_float_64()),
			(NumberT::Float(a), NumberT::Integer(b)) => NumberT::Float(a.to_float_64() + b.to_float_64()),
		};
	}
}

impl One for NumberT {
	#[inline(always)]
	fn one() -> Self {
		NumberT::Integer(1)
	}
}

impl Mul<Self> for NumberT {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		return match (self, rhs) {
			(NumberT::Integer(a), NumberT::Integer(b)) => NumberT::Integer(a * b),
			(NumberT::Float(a), NumberT::Float(b)) => NumberT::Float(a.to_float_64() * b.to_float_64()),
			(NumberT::Integer(a), NumberT::Float(b)) => NumberT::Float(a.to_float_64() * b.to_float_64()),
			(NumberT::Float(a), NumberT::Integer(b)) => NumberT::Float(a.to_float_64() * b.to_float_64()),
		};
	}
}

impl Sub<Self> for NumberT {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		return match (self, rhs) {
			(NumberT::Integer(a), NumberT::Integer(b)) => NumberT::Integer(a - b),
			(NumberT::Float(a), NumberT::Float(b)) => NumberT::Float(a.to_float_64() - b.to_float_64()),
			(NumberT::Integer(a), NumberT::Float(b)) => NumberT::Float(a.to_float_64() - b.to_float_64()),
			(NumberT::Float(a), NumberT::Integer(b)) => NumberT::Float(a.to_float_64() - b.to_float_64()),
		};
	}
}

impl Div<Self> for NumberT {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		return match (self, rhs) {
			(NumberT::Integer(a), NumberT::Integer(b)) => {
				if a % b == 0 {
					NumberT::Integer(a / b)
				} else {
					NumberT::Float(a.to_float_64() / b.to_float_64())
				}
			}
			(NumberT::Float(a), NumberT::Float(b)) => NumberT::Float(a.to_float_64() / b.to_float_64()),
			(NumberT::Integer(a), NumberT::Float(b)) => NumberT::Float(a.to_float_64() / b.to_float_64()),
			(NumberT::Float(a), NumberT::Integer(b)) => NumberT::Float(a.to_float_64() / b.to_float_64()),
		};
	}
}

impl Rem<Self> for NumberT {
	type Output = Self;

	fn rem(self, rhs: Self) -> Self::Output {
		return match (self, rhs) {
			(NumberT::Integer(a), NumberT::Integer(b)) => NumberT::Integer(a % b),
			(NumberT::Float(a), NumberT::Float(b)) => NumberT::Float(a.to_float_64() % b.to_float_64()),
			(NumberT::Integer(a), NumberT::Float(b)) => NumberT::Float(a.to_float_64() % b.to_float_64()),
			(NumberT::Float(a), NumberT::Integer(b)) => NumberT::Float(a.to_float_64() % b.to_float_64()),
		};
	}
}

impl Num for NumberT {
	type FromStrRadixErr = EvilangError;

	fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
		Ok(match i128::from_str_radix(str, radix) {
			Ok(v) => NumberT::Integer(v),
			Err(_) => NumberT::Float(f64::from_str_radix(str, radix).map_err(|_err| EvilangError::new(ErrorT::InvalidNumericLiteral(str.into()).into()))?),
		})
	}
}

impl NumberT {
	#[inline(always)]
	pub fn round_to_int(&self) -> i128 {
		match self {
			NumberT::Integer(v) => *v,
			NumberT::Float(f) => f.round() as i128,
		}
	}
	#[inline(always)]
	pub fn floor_to_int(&self) -> i128 {
		match self {
			NumberT::Integer(v) => *v,
			NumberT::Float(f) => f.floor() as i128,
		}
	}
	#[inline(always)]
	pub fn ceil_to_int(&self) -> i128 {
		match self {
			NumberT::Integer(v) => *v,
			NumberT::Float(f) => f.ceil() as i128,
		}
	}
	#[inline(always)]
	pub fn as_float(&self) -> f64 {
		match self {
			NumberT::Integer(v) => *v as f64,
			NumberT::Float(f) => *f,
		}
	}
}
