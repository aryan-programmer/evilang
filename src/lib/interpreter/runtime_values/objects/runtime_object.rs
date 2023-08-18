use std::ops::Deref;

use gc::{Finalize, Trace};

use crate::ast::structs::CallExpression;
use crate::errors::{Descriptor, ResultWithError, RuntimeError};
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::functions::ifunction::IFunction;
use crate::interpreter::runtime_values::functions::types::{FunctionParameters, FunctionReturnValue};
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::runtime_values::ref_to_value::RefToValue;
use crate::interpreter::utils::cell_ref::{gc_clone, gc_ptr_cell_from, GcPtr};
use crate::interpreter::utils::consts::INSTANCE_OF_;
use crate::interpreter::utils::consume_or_clone::ConsumeOrCloneOf;
use crate::interpreter::variables_containers::map::{GcPtrMutCellToVariablesMap, IVariablesMapConstMembers};
use crate::interpreter::variables_containers::scope::IGenericVariablesScope;
use crate::interpreter::variables_containers::VariablesMap;
use crate::types::string::{CowStringT, StringT};

pub type GcPtrToObject = GcPtr<RuntimeObject>;

#[derive(Debug, PartialEq, Trace, Finalize)]
pub struct RuntimeObject {
	pub properties: GcPtrMutCellToVariablesMap,
	pub parent: Option<GcPtrToObject>,
	pub name: StringT,
}

impl RuntimeObject {
	pub fn new_gc(
		variables: VariablesMap,
		parent: Option<GcPtrToObject>,
		name: StringT,
	) -> GcPtrToObject {
		GcPtr::new(Self {
			properties: gc_ptr_cell_from(variables),
			parent,
			name,
		})
	}

	#[inline(always)]
	pub fn allocate(
		parent: GcPtrToObject,
		name: StringT
	) -> GcPtrToObject {
		return Self::new_gc(
			VariablesMap::new(),
			Some(parent),
			name
		);
	}

	#[inline]
	pub fn allocate_instance(parent: GcPtrToObject) -> GcPtrToObject {
		let instance_name = INSTANCE_OF_.to_string() + &parent.name;
		return Self::new_gc(
			VariablesMap::new(),
			Some(parent),
			instance_name,
		);
	}

	pub fn call_method_on_object_with_args(this: GcPtrToObject, env: &mut Environment, method_name: CowStringT, call_expr: &CallExpression) -> ResultWithError<FunctionReturnValue> {
		let Some(method_prop_box) = this.get_actual(method_name) else {
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
		return Ok(method.execute(env, args_with_this)?.into());
	}
}

impl IGenericVariablesScope<RuntimeObject> for RuntimeObject {
	#[inline(always)]
	fn get_variables(&self) -> GcPtrMutCellToVariablesMap {
		gc_clone(&self.properties)
	}

	#[inline(always)]
	fn get_parent(&self) -> Option<GcPtrToObject> {
		self.parent.clone()
	}
}
