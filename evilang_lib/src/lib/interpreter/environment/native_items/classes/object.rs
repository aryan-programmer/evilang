use std::cell::OnceCell;
use std::collections::HashMap;

use crate::errors::ResultWithError;
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::i_native_struct::{INativeClass, INativeClass_BuildClass, INativeClass_GetClassCached};
use crate::interpreter::runtime_values::objects::runtime_object::{GcPtrToObject, RuntimeObject};
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::variables_containers::VariablesMap;
use crate::types::cell_ref::{gc_clone, gc_ptr_cell_from};
use crate::types::consts::{CONSTRUCTOR, OBJECT};

pub struct ObjectSuperclass {}

impl ObjectSuperclass {
	thread_local! {
		static CACHE: OnceCell<GcPtrToObject> = OnceCell::new();
	}

	pub fn build_and_cache() -> GcPtrToObject {
		// println!("Calling build_and_cache");
		ObjectSuperclass::CACHE.with(|cell| {
			gc_clone(cell.get_or_init(|| {
				// println!("Building object");
				RuntimeObject::new_gc(VariablesMap::new_direct(HashMap::from([
					(CONSTRUCTOR.into(), gc_ptr_cell_from(PrimitiveValue::new_native_function(
						|_env, _params| Ok(PrimitiveValue::Null)
					)))
				])), None, OBJECT.into())
			}))
		})
	}
}

impl INativeClass_GetClassCached for ObjectSuperclass {
	#[inline(always)]
	fn get_class_cached(_env: &mut Environment) -> ResultWithError<GcPtrToObject> {
		Ok(Self::build_and_cache())
	}
}

impl INativeClass_BuildClass for ObjectSuperclass {
	#[inline(always)]
	fn build_class(_env: &mut Environment) -> ResultWithError<GcPtrToObject> {
		Ok(Self::build_and_cache())
	}
}

impl INativeClass for ObjectSuperclass {
	const NAME: &'static str = OBJECT;

	fn get_parent_class(_env: &mut Environment) -> ResultWithError<Option<GcPtrToObject>> {
		Ok(None)
	}
}
