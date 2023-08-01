use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub type RcCell<T> = Rc<RefCell<T>>;

pub fn rc_cell_from<T>(v: T) -> RcCell<T> {
	return Rc::new(RefCell::new(v));
}

#[derive(Debug)]
pub enum CellRef<'a, T> {
	Ref(&'a T),
	FromRefCell(Ref<'a, T>),
}

impl<'a, T> Deref for CellRef<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		return match self {
			CellRef::Ref(v) => *v,
			CellRef::FromRefCell(r) => r.deref(),
		};
	}
}

#[derive(Debug)]
pub enum MutCellRef<'a, T> {
	MutRef(&'a mut T),
	FromRefCell(RefMut<'a, T>),
}

impl<'a, T> Deref for MutCellRef<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		return match self {
			MutCellRef::MutRef(v) => *v,
			MutCellRef::FromRefCell(r) => r.deref(),
		};
	}
}

impl<'a, T> DerefMut for MutCellRef<'a, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return match self {
			MutCellRef::MutRef(v) => *v,
			MutCellRef::FromRefCell(ref mut r) => r.deref_mut(),
		};
	}
}
