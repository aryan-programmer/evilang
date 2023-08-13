pub mod consts;
pub mod consume_or_clone;

use crate::errors::{EvilangError, ResultWithError, RuntimeError};
use crate::interpreter::utils::consts::{OBJECT, INSTANCE_OF_};
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::variables_containers::map::IVariablesMapConstMembers;

pub fn get_object_superclass(env: &mut Environment) -> ResultWithError<PrimitiveValue> {
	let object_class_name = OBJECT.to_string();
	Ok(env
		.global_scope
		.borrow()
		.get_actual(&object_class_name)
		.ok_or_else(|| EvilangError::new(RuntimeError::ExpectedClassObject(object_class_name.into()).into()))?
		.borrow()
		.clone())
}
