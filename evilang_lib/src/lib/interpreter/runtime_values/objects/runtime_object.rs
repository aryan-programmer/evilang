use std::ops::Deref;

use gc::{Finalize, Trace};
use maybe_owned::MaybeOwned;

use crate::ast::structs::{CallExpression, ClassDeclaration};
use crate::errors::{Descriptor, ResultWithError, RuntimeError};
use crate::interpreter::environment::Environment;
use crate::interpreter::environment::native_items::classes::object::ObjectSuperclass;
use crate::interpreter::runtime_values::functions::Function;
use crate::interpreter::runtime_values::functions::ifunction::IFunction;
use crate::interpreter::runtime_values::functions::types::{FunctionParameters, FunctionReturnValue};
use crate::interpreter::runtime_values::i_native_struct::INativeClass_GetClassCached;
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::runtime_values::ref_to_value::RefToValue;
use crate::interpreter::utils::expect_object;
use crate::interpreter::variables_containers::map::{GcPtrMutCellToVariablesMap, IVariablesMapConstMembers, IVariablesMapDelegator};
use crate::interpreter::variables_containers::scope::IGenericVariablesScope;
use crate::interpreter::variables_containers::VariablesMap;
use crate::types::cell_ref::{gc_clone, gc_ptr_cell_from, GcPtr};
use crate::types::consts::{INSTANCE_OF_, SUPER};
use crate::types::string::{CowStringT, StringT};
use crate::types::traits::ConsumeOrCloneOf;

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

	#[inline]
	pub fn allocate_instance(parent: GcPtrToObject, name: Option<String>) -> GcPtrToObject {
		let instance_name = name.unwrap_or_else(|| INSTANCE_OF_.to_string() + &parent.name);
		return Self::new_gc(
			VariablesMap::new(),
			Some(parent),
			instance_name,
		);
	}

	pub fn new_class_decl(env: &mut Environment, decl: &ClassDeclaration) -> ResultWithError<GcPtrToObject> {
		let ClassDeclaration {
			name,
			super_class,
			methods
		} = decl;
		let super_class = if let Some(v) = super_class {
			expect_object(env.eval(v)?, Some(v))?
		} else {
			ObjectSuperclass::get_class_cached(env)?
		};
		let scope = Environment::new_with_parent(env)?;
		scope.declare(SUPER.into(), PrimitiveValue::Object(gc_clone(&super_class)))?;
		let sub_class = RuntimeObject::new_gc(
			VariablesMap::new_direct(methods
				.clone()
				.into_iter()
				.map(|fdecl| {
					(fdecl.name.clone(), gc_ptr_cell_from(Function::new_closure(&scope, fdecl).into()))
				})
				.collect()
			),
			Some(super_class),
			name.clone(),
		);
		return Ok(sub_class);
	}

	pub fn call_method_on_object_with_args(this: GcPtrToObject, env: &mut Environment, method_name: CowStringT, call_expr: &CallExpression) -> ResultWithError<FunctionReturnValue> {
		let Some(method_prop_box) = this.get_actual(method_name).map(MaybeOwned::into_owned) else {
			return Err(RuntimeError::ExpectedFunction(Descriptor::Expression((*call_expr.callee).clone())).into());
		};
		let method_prop = method_prop_box.borrow();
		let PrimitiveValue::Function(ref method) = method_prop.deref() else {
			return Err(RuntimeError::ExpectedFunction(Descriptor::new_both(method_prop.deref().into(), call_expr.callee.deref().into())).into());
		};
		let args_with_this = Some(Ok(PrimitiveValue::Object(this)) as ResultWithError<PrimitiveValue>)
			.into_iter()
			.chain(
				call_expr
					.arguments
					.iter()
					.map(|v| env
						.eval(v)
						.and_then(RefToValue::consume_or_clone)
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
