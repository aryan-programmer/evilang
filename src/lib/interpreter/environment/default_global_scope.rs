use crate::interpreter::environment::native_functions::make_native_functions_list;
use crate::interpreter::environment::resolver::BoxIResolver;
use crate::interpreter::runtime_values::objects::runtime_object::RuntimeObject;
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::utils::cell_ref::{gc_box_from, GcBox};
use crate::interpreter::utils::consts::OBJECT;
use crate::interpreter::variables_containers::{GlobalScope, VariablesMap};

pub fn make_object_class() -> GcBox<RuntimeObject> {
	return RuntimeObject::new_gc(VariablesMap::new(), None, OBJECT.into());
}

fn make_default_global_variables() -> VariablesMap {
	return VariablesMap::new_direct(
		make_native_functions_list()
			.into_iter()
			.map(|(name, f)| {
				(name, PrimitiveValue::new_native_function(f))
			})
			.chain([
				(OBJECT.to_string(), PrimitiveValue::Object(make_object_class()))
			].into_iter())
			.map(|(name, val)| (name, gc_box_from(val)))
			.collect()
	);
}

pub fn get_default_global_scope(resolver: BoxIResolver) -> GcBox<GlobalScope> {
	return GlobalScope::new_gc_from_variables(make_default_global_variables(), resolver);
}
