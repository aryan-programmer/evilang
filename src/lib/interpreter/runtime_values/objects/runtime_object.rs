use std::ops::Deref;

use gc::{Finalize, Trace};

use crate::ast::expression::IdentifierT;
use crate::ast::structs::CallExpression;
use crate::errors::{Descriptor, ResultWithError, RuntimeError};
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::functions::ifunction::IFunction;
use crate::interpreter::runtime_values::functions::types::{FunctionParameters, FunctionReturnValue};
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::runtime_values::ref_to_value::RefToValue;
use crate::interpreter::utils::cell_ref::{gc_box_from, gc_cell_clone, GcBox};
use crate::interpreter::utils::consts::INSTANCE_OF_;
use crate::interpreter::utils::consume_or_clone::ConsumeOrCloneOf;
use crate::interpreter::variables_containers::map::IVariablesMapConstMembers;
use crate::interpreter::variables_containers::scope::IGenericVariablesScope;
use crate::interpreter::variables_containers::VariablesMap;

#[derive(Debug, PartialEq, Trace, Finalize)]
pub struct RuntimeObject {
	pub properties: GcBox<VariablesMap>,
	pub parent: Option<GcBox<RuntimeObject>>,
	pub name: String,
}

impl RuntimeObject {
	pub fn new_gc(
		variables: VariablesMap,
		parent: Option<GcBox<RuntimeObject>>,
		name: String,
	) -> GcBox<RuntimeObject> {
		gc_box_from(Self {
			properties: gc_box_from(variables),
			parent,
			name,
		})
	}

	#[inline(always)]
	pub fn allocate(parent: GcBox<RuntimeObject>, name: String) -> GcBox<RuntimeObject> {
		return Self::new_gc(VariablesMap::new(), Some(parent), name);
	}

	#[inline]
	pub fn allocate_instance(parent: GcBox<RuntimeObject>) -> GcBox<RuntimeObject> {
		let instance_name = INSTANCE_OF_.to_string() + &parent.borrow().name;
		return Self::new_gc(
			VariablesMap::new(),
			Some(parent),
			instance_name,
		);
	}

	pub fn call_method_on_object_with_args(this: GcBox<RuntimeObject>, env: &mut Environment, method_name: &IdentifierT, call_expr: &CallExpression) -> ResultWithError<FunctionReturnValue> {
		let Some(method_prop_box) = this.borrow().get_actual(method_name) else {
			return Err(RuntimeError::ExpectedFunction(Descriptor::Expression((*call_expr.callee).clone())).into());
		};
		let method_prop = method_prop_box.borrow();
		let PrimitiveValue::Function(ref method) = method_prop.deref() else {
			return Err(RuntimeError::ExpectedFunction(Descriptor::Both {
				value: method_prop.deref().clone(),
				expression: (*call_expr.callee).clone(),
			}).into());
		};
		let args_with_this = Some(Ok(PrimitiveValue::Object(this)) as ResultWithError<PrimitiveValue>)
			.into_iter()
			.chain(
				call_expr
					.arguments
					.iter()
					.map(|v| env
						.eval(v)
						.map(RefToValue::consume_or_clone)
					)
			)
			.collect::<ResultWithError<FunctionParameters>>()?;
		return Ok(method.borrow().execute(env, args_with_this)?.into());
	}
}

impl IGenericVariablesScope<RuntimeObject> for RuntimeObject {
	#[inline(always)]
	fn get_variables(&self) -> GcBox<VariablesMap> {
		gc_cell_clone(&self.properties)
	}

	#[inline(always)]
	fn get_parent(&self) -> Option<GcBox<RuntimeObject>> {
		self.parent.clone()
	}
}
