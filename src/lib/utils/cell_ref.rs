use gc::{Finalize, Gc, GcCell, Trace};

pub type GcBox<T> = Gc<GcCell<T>>;

#[inline(always)]
pub fn gc_box_from<T: Trace + Finalize>(v: T) -> GcBox<T> {
	return Gc::new(GcCell::new(v));
}

#[inline(always)]
pub fn gc_cell_clone<T>(v: &Gc<T>) -> Gc<T> {
	return Gc::clone(v);
}
