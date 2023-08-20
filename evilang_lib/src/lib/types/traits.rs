pub use evilang_traits::*;

use crate::errors::ResultWithError;

pub trait ConsumeOrCloneOf {
	type Target;

	fn consume_or_clone(self) -> ResultWithError<Self::Target>;
}
