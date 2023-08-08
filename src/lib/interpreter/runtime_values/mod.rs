use std::ops::Deref;

use gc::{Finalize, Trace};

use crate::ast::structs::FunctionDeclaration;
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::functions::closure::Closure;
use crate::interpreter::runtime_values::functions::Function;
use crate::utils::cell_ref::{gc_box_from, gc_cell_clone, GcBox};

pub mod ref_to_value;
pub mod functions;

pub trait GcBoxOfPrimitiveValueExt {
	fn is_hoisted(&self) -> bool;
}

impl GcBoxOfPrimitiveValueExt for GcBox<PrimitiveValue> {
	#[inline(always)]
	fn is_hoisted(&self) -> bool {
		return self.deref().borrow().deref().is_hoisted();
	}
}

#[derive(Debug, Clone, PartialEq, Trace, Finalize)]
pub enum PrimitiveValue {
	_HoistedVariable,
	Null,
	Boolean(bool),
	Integer(i64),
	String(String),
	Function(GcBox<Function>),
}

impl PrimitiveValue {
	pub fn new_closure(env: &Environment, decl: FunctionDeclaration) -> Self {
		let closure = Closure::new(
			decl,
			gc_cell_clone(&env.scope),
			gc_cell_clone(&env.global_scope),
		);
		let function_closure = Function::Closure(closure);
		return PrimitiveValue::Function(gc_box_from(function_closure));
	}

	pub fn is_truthy(&self) -> bool {
		return match self {
			PrimitiveValue::Null | PrimitiveValue::_HoistedVariable => false,
			PrimitiveValue::Boolean(v) => *v,
			PrimitiveValue::Integer(i) => *i != 0,
			PrimitiveValue::String(s) => s.len() != 0,
			PrimitiveValue::Function(..) => true,
		};
	}

	#[inline(always)]
	pub fn is_hoisted(&self) -> bool {
		return self == &PrimitiveValue::_HoistedVariable;
	}
}
