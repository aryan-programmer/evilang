use std::collections::HashMap;

use crate::interpreter::environment::native_items::make_native_functions_list;
use crate::interpreter::environment::resolver::BoxIResolver;
use crate::interpreter::runtime_values::functions::native_function::NativeFunction;
use crate::interpreter::runtime_values::objects::runtime_object::{GcPtrToObject, RuntimeObject};
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::utils::cell_ref::gc_ptr_cell_from;
use crate::interpreter::utils::consts::{CONSTRUCTOR, OBJECT};
use crate::interpreter::variables_containers::{GcPtrMutCellToGlobalScope, GlobalScope, VariablesMap};

pub fn make_object_class() -> GcPtrToObject {
	return RuntimeObject::new_gc(VariablesMap::new_direct(HashMap::from([
		(CONSTRUCTOR.into(), gc_ptr_cell_from(PrimitiveValue::new_native_function(
			NativeFunction::new(|_env, _params| Ok(PrimitiveValue::Null))
		)))
	])), None, OBJECT.into());
}

fn make_default_global_variables() -> VariablesMap {
	return VariablesMap::new_direct(
		make_native_functions_list()
			.into_iter()
			.map(|(name, f)| {
				(name, PrimitiveValue::new_native_function(f))
			})
			.chain([
				(OBJECT.into(), PrimitiveValue::Object(make_object_class()))
			].into_iter())
			.map(|(name, val)| (name, gc_ptr_cell_from(val)))
			.collect()
	);
}

pub fn get_default_global_scope(resolver: BoxIResolver) -> GcPtrMutCellToGlobalScope {
	return GlobalScope::new_gc_from_variables(make_default_global_variables(), resolver);
}
