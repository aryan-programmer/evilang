use std::ops::Deref;

use gc::{Finalize, Trace};
use num_traits::Zero;

use crate::ast::structs::{ClassDeclaration, FunctionDeclaration};
use crate::errors::ResultWithError;
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::functions::closure::Closure;
use crate::interpreter::runtime_values::functions::Function;
use crate::interpreter::runtime_values::functions::native_function::NativeFunction;
use crate::interpreter::runtime_values::objects::runtime_object::{GcPtrToObject, RuntimeObject};
use crate::interpreter::utils::{expect_object, get_object_superclass};
use crate::interpreter::utils::cell_ref::{gc_ptr_cell_from, gc_clone, GcPtr, GcPtrCell};
use crate::interpreter::utils::consts::SUPER;
use crate::interpreter::variables_containers::map::IVariablesMapDelegator;
use crate::interpreter::variables_containers::VariablesMap;
use crate::math::number::NumberT;

pub mod ref_to_value;
pub mod functions;
pub mod objects;

pub type GcPtrVariable = GcPtr<GcPtrCell<PrimitiveValue>>;

pub trait GcPtrVariableExt {
	fn is_hoisted(&self) -> bool;
}

impl GcPtrVariableExt for GcPtrVariable {
	#[inline(always)]
	fn is_hoisted(&self) -> bool {
		return self.deref().borrow().deref().is_hoisted();
	}
}

#[derive(Debug, Clone, Trace, Finalize)]
pub enum PrimitiveValue {
	_HoistedVariable,
	Null,
	Boolean(bool),
	Number(#[unsafe_ignore_trace] NumberT),
	String(String),
	Function(GcPtr<Function>),
	Object(GcPtrToObject),
}

impl PartialEq for PrimitiveValue {
	#[inline]
	fn eq(&self, other: &PrimitiveValue) -> bool {
		match (self, other) {
			(
				PrimitiveValue::Boolean(self_0),
				PrimitiveValue::Boolean(othr_0),
			) => *self_0 == *othr_0,
			(
				PrimitiveValue::Number(self_0),
				PrimitiveValue::Number(othr_0),
			) => *self_0 == *othr_0,
			(
				PrimitiveValue::String(self_0),
				PrimitiveValue::String(othr_0),
			) => *self_0 == *othr_0,
			(
				PrimitiveValue::Function(self_0),
				PrimitiveValue::Function(othr_0),
			) => GcPtr::ptr_eq(self_0, othr_0),
			(
				PrimitiveValue::Object(self_0),
				PrimitiveValue::Object(othr_0),
			) => GcPtr::ptr_eq(self_0, othr_0),
			(PrimitiveValue::Null, PrimitiveValue::Null) => true,
			(PrimitiveValue::_HoistedVariable, PrimitiveValue::_HoistedVariable) => true,
			_ => false,
		}
	}
}

impl PrimitiveValue {
	pub fn float(v: f64) -> Self {
		PrimitiveValue::Number(NumberT::Float(v))
	}

	pub fn integer(v: i64) -> Self {
		PrimitiveValue::Number(NumberT::Integer(v as i128))
	}

	pub fn new_native_function(f: NativeFunction) -> Self {
		return PrimitiveValue::Function(GcPtr::new(Function::NativeFunction(f)));
	}

	pub fn new_closure(env: &Environment, decl: FunctionDeclaration) -> Self {
		let closure = Closure::new(
			decl,
			env.clone(),
		);
		let function_closure = Function::Closure(closure);
		return PrimitiveValue::Function(GcPtr::new(function_closure));
	}

	pub fn new_class_by_eval(env: &mut Environment, decl: &ClassDeclaration) -> ResultWithError<Self> {
		let ClassDeclaration {
			name,
			super_class,
			methods
		} = decl;
		let super_class = if let Some(v) = super_class {
			expect_object(env.eval(v)?, Some(v))?
		} else {
			get_object_superclass(env)?
		};
		let scope = Environment::new_with_parent(env);
		scope.declare(&SUPER.to_string(), PrimitiveValue::Object(gc_clone(&super_class)))?;
		let sub_class = RuntimeObject::new_gc(
			VariablesMap::new_direct(methods
				.clone()
				.into_iter()
				.map(|fdecl| {
					(fdecl.name.clone(), gc_ptr_cell_from(PrimitiveValue::new_closure(&scope, fdecl)))
				})
				.collect()
			),
			Some(super_class),
			name.clone(),
		);
		return Ok(PrimitiveValue::Object(sub_class));
	}

	pub fn is_truthy(&self) -> bool {
		return match self {
			PrimitiveValue::Null | PrimitiveValue::_HoistedVariable => false,
			PrimitiveValue::Boolean(v) => *v,
			PrimitiveValue::Number(i) => !i.is_zero(),
			PrimitiveValue::String(s) => s.len() != 0,
			PrimitiveValue::Function(..) | PrimitiveValue::Object(..) => true,
		};
	}

	#[inline(always)]
	pub fn is_hoisted(&self) -> bool {
		return self == &PrimitiveValue::_HoistedVariable;
	}
}
