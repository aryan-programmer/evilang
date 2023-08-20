use gc::{Finalize, Gc, GcCell, Trace};

pub type GcPtr<T> = Gc<T>;
pub type GcPtrCell<T> = GcCell<T>;

#[inline(always)]
pub fn gc_ptr_cell_from<T: Trace + Finalize>(v: T) -> GcPtr<GcPtrCell<T>> {
	return Gc::new(GcCell::new(v));
}

#[inline(always)]
pub fn gc_ptr_from<T: Trace>(v: T) -> Gc<T> {
	return Gc::new(v);
}

#[inline(always)]
pub fn gc_clone<T>(v: &Gc<T>) -> Gc<T> {
	return Gc::clone(v);
}
