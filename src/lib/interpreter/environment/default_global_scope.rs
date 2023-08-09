use crate::interpreter::environment::native_functions::get_native_functions_list;
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::variables_map::{GlobalScope, VariablesMap};
use crate::utils::cell_ref::{gc_box_from, GcBox};

fn get_default_global_variables() -> VariablesMap {
	return VariablesMap::new_direct(
		get_native_functions_list()
			.into_iter()
			.map(|(name, f)| {
				(name, gc_box_from(PrimitiveValue::new_native_function(f)))
			})
			.collect()
	);
}

pub fn get_default_global_scope() -> GcBox<GlobalScope> {
	return GlobalScope::new_gc_from_variables(get_default_global_variables());
}
