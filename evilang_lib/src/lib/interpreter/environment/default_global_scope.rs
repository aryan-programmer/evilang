use crate::errors::ResultWithError;
use crate::interpreter::environment::Environment;
use crate::interpreter::environment::native_items::classes::object::ObjectSuperclass;
use crate::interpreter::environment::native_items::classes::vector::Vector;
use crate::interpreter::environment::native_items::make_native_functions_list;
use crate::interpreter::environment::resolver::BoxIResolver;
use crate::interpreter::runtime_values::i_native_struct::{INativeClass, INativeClass_GetClassCached};
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::variables_containers::{GcPtrMutCellToGlobalScope, GlobalScope, VariablesMap};
use crate::interpreter::variables_containers::map::IVariablesMapDelegator;
use crate::types::cell_ref::gc_ptr_cell_from;
use crate::types::consts::OBJECT;

fn make_default_global_variables() -> VariablesMap {
	return VariablesMap::new_direct(
		make_native_functions_list()
			.into_iter()
			.map(|(name, f)| {
				(name, PrimitiveValue::new_native_function(f))
			})
			.chain([
				(OBJECT.into(), PrimitiveValue::Object(ObjectSuperclass::build_and_cache())),
			].into_iter())
			.map(|(name, val)| (name, gc_ptr_cell_from(val)))
			.collect()
	);
}

pub fn get_default_global_scope(resolver: BoxIResolver) -> GcPtrMutCellToGlobalScope {
	return GlobalScope::new_gc_from_variables(make_default_global_variables(), resolver);
}

pub fn setup_environment(env: &mut Environment) -> ResultWithError<()> {
	let vec_obj = Vector::get_class_cached(env)?;
	env.global_scope.borrow().assign_locally(Vector::NAME.into(), PrimitiveValue::Object(vec_obj));
	return Ok(());
}
