#![allow(unused_parens)]

pub use evilang_proc_macros::*;

pub struct CloningFailureErrorT;

impl CloningFailureErrorT {
	#[inline(always)]
	pub fn new() -> Self {
		Self {}
	}
}

pub trait TryClone: Sized {
	fn try_clone(&self) -> Result<Self, CloningFailureErrorT>;
}

/**
Clone__SilentlyFail is meant to replace uncloneable data with null data, intended to be used for error logging, where perfect cloning is not necessary.
 */
#[allow(non_camel_case_types)]
pub trait Clone__SilentlyFail {
	#[allow(non_snake_case)]
	fn clone__silently_fail(&self) -> Self;
}

impl<T: Clone> TryClone for T {
	#[inline(always)]
	fn try_clone(&self) -> Result<Self, CloningFailureErrorT> {
		Ok(self.clone())
	}
}

impl<T: Clone> Clone__SilentlyFail for T {
	#[inline(always)]
	fn clone__silently_fail(&self) -> Self {
		self.clone()
	}
}

// macro_rules! impl_try_clone_n_tuple {
// 	($($args: ident),*) => {
// 		impl<$($args: TryClone),*> TryClone for ($($args),*) {
// 			#[inline(always)]
// 			fn try_clone(&self) -> Result<Self, CloningFailureErrorT> {
// 				let ($(ref $args),*) = self;
// 				return Ok(($(TryClone::try_clone($args)?),*));
// 			}
// 		}
// 	};
// }
//
// macro_rules! impl_try_clone_1_to_n_tuple {
// 	($first:ident, $($args: ident),*) => {
// 		impl_try_clone_n_tuple!($first, $($args),*);
// 		impl_try_clone_1_to_n_tuple!($($args),*);
// 	};
// 	($last: ident) => {
// 		impl_try_clone_n_tuple!($last);
// 	}
// }

// ,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z
//impl_try_clone_1_to_n_tuple!(A,B,C,D);
