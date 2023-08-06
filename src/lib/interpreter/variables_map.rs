use std::collections::HashMap;
use std::mem::replace;
use std::ops::DerefMut;

use delegate::delegate;
use gc::{Finalize, Trace};

use crate::ast::expression::IdentifierT;
use crate::errors::{ErrorT, ResultWithError};
use crate::interpreter::runtime_value::{GcBoxOfPrimitiveValueExt, PrimitiveValue, RefToValue};
use crate::utils::cell_ref::{gc_box_from, gc_cell_clone, GcBox};

pub trait IVariablesMap {
	///
	///
	/// # Arguments
	///
	/// * `name`:
	/// * `value`:
	///
	/// returns: Option<PrimitiveValue> The previous value stored in the variable
	fn set_actual(&mut self, name: &IdentifierT, value: RefToValue) -> Option<PrimitiveValue>;
	fn declare(&mut self, name: &IdentifierT, value: RefToValue) -> ResultWithError<()>;
	fn hoist(&mut self, name: &IdentifierT) -> ResultWithError<()>;
	fn get_actual(&self, name: &IdentifierT) -> Option<RefToValue>;
	fn contains_key(&self, name: &IdentifierT) -> bool;
	// fn get_variable(&self, name: &IdentifierT) -> ResultWithError<RefToValue> {
	// 	let gotten_val = self.get_actual(name);
	// 	let Some(ref_to_val) = gotten_val else {
	// 		return Err(ErrorT::CantAccessUndeclaredVariable(name.clone()).into());
	// 	};
	// 	return match &ref_to_val {
	// 		RefToValue::RValue(_) => Err(ErrorT::ExpectedLhsExpression.into()),
	// 		RefToValue::LValue(v) => if v.is_hoisted() {
	// 			Err(ErrorT::CantAccessHoistedVariable(name.clone()).into())
	// 		} else {
	// 			Ok(ref_to_val)
	// 		},
	// 	};
	// }

	fn get_variable_or_null(&self, name: &IdentifierT) -> ResultWithError<RefToValue> {
		let gotten_val = self.get_actual(name);
		let Some(ref_to_val) = gotten_val else {
			return Ok(RefToValue::RValue(PrimitiveValue::Null));
		};
		return match &ref_to_val {
			RefToValue::RValue(_) => Err(ErrorT::ExpectedLhsExpression.into()),
			RefToValue::LValue(v) => if v.is_hoisted() {
				Err(ErrorT::CantAccessHoistedVariable(name.clone()).into())
			} else {
				Ok(ref_to_val)
			},
		};
	}
}

#[macro_export]
macro_rules! delegate_ivariables_map {
	(for $for_type: ty => &$self: ident: $const_delegator: expr, &mut $mut_self: ident: $mut_delegator: expr) => {
		impl IVariablesMap for $for_type {
			#[inline(always)]
			fn get_actual(&$self, name: &IdentifierT) -> Option<RefToValue> {
				return $const_delegator.get_actual(name);
			}
			#[inline(always)]
			fn contains_key(&$self, name: &IdentifierT) -> bool {
				return $const_delegator.contains_key(name);
			}
			#[inline(always)]
			fn set_actual(&mut $mut_self, name: &IdentifierT, value: RefToValue) -> Option<PrimitiveValue> {
				return $mut_delegator.set_actual(name, value);
			}
			#[inline(always)]
			fn declare(&mut $mut_self, name: &IdentifierT, value: RefToValue) -> ResultWithError<()> {
				return $mut_delegator.declare(name, value);
			}
			#[inline(always)]
			fn hoist(&mut $mut_self, name: &IdentifierT) -> ResultWithError<()> {
				return $mut_delegator.hoist(name);
			}
		}
	};
}

pub use delegate_ivariables_map;

#[derive(Trace, Finalize)]
pub struct VariablesMap {
	variables: HashMap<IdentifierT, GcBox<PrimitiveValue>>,
}

impl VariablesMap {
	#[inline(always)]
	pub fn new() -> Self {
		Self { variables: HashMap::new() }
	}
	#[inline(always)]
	pub fn new_direct(variables: HashMap<IdentifierT, GcBox<PrimitiveValue>>) -> Self {
		Self { variables }
	}
	pub fn new_from_primitives(variables: HashMap<IdentifierT, PrimitiveValue>) -> Self {
		Self {
			variables: variables
				.into_iter()
				.map(|(iden, val)| (iden, gc_box_from(val)))
				.collect(),
		}
	}
}

impl IVariablesMap for VariablesMap {
	fn set_actual(&mut self, name: &IdentifierT, value: RefToValue) -> Option<PrimitiveValue> {
		return if let Some(v) = self.variables.get(name) {
			let mut res = v.borrow_mut();
			Some(replace(res.deref_mut(), value.consume_or_clone()))
		} else {
			self.variables.insert(name.clone(), gc_box_from(value.consume_or_clone()));
			None
		};
	}

	fn declare(&mut self, name: &IdentifierT, value: RefToValue) -> ResultWithError<()> {
		if let Some(v) = self.variables.get(name) {
			if !v.is_hoisted() {
				return Err(ErrorT::CantRedeclareVariable(name.clone()).into());
			}
		}
		if value.is_hoisted() {
			return Err(ErrorT::CantSetToHoistedValue.into());
		}
		self.set_actual(name, value);
		return Ok(());
	}

	fn hoist(&mut self, name: &IdentifierT) -> ResultWithError<()> {
		if let Some(_v) = self.variables.get(name) {
			return Err(ErrorT::CantRedeclareVariable(name.clone()).into());
		}
		self.set_actual(name, PrimitiveValue::_HoistedVariable.into());
		return Ok(());
	}

	fn get_actual(&self, name: &IdentifierT) -> Option<RefToValue> {
		return if let Some(v) = self.variables.get(name) {
			Some(RefToValue::LValue(gc_cell_clone(v)))
		} else {
			None
		};
	}

	#[inline(always)]
	fn contains_key(&self, name: &IdentifierT) -> bool {
		self.variables.contains_key(name)
	}
}

#[derive(Trace, Finalize)]
pub struct VariableScope {
	variables: GcBox<VariablesMap>,
	parent: Option<GcBox<VariableScope>>,
}

impl IVariablesMap for VariableScope {
	delegate! {
		to self.resolve_variable_scope(name).borrow_mut() {
			fn set_actual(&mut self, name: &IdentifierT, value: RefToValue) -> Option<PrimitiveValue>;
		}
		to self.resolve_variable_scope(name).borrow() {
			fn get_actual(&self, name: &IdentifierT) -> Option<RefToValue>;
			fn contains_key(&self, name: &IdentifierT) -> bool;
		}
		to self.variables.borrow_mut() {
			fn declare(&mut self, name: &IdentifierT, value: RefToValue) -> ResultWithError<()>;
			fn hoist(&mut self, name: &IdentifierT) -> ResultWithError<()>;
		}
	}
}

impl VariableScope {
	pub fn new_gc(
		variables: VariablesMap,
		parent: Option<GcBox<VariableScope>>,
	) -> GcBox<VariableScope> {
		gc_box_from(Self {
			variables: gc_box_from(variables),
			parent: parent.and_then(|v| Some(v)),
		})
	}

	fn resolve_variable_scope(&self, name: &IdentifierT) -> GcBox<VariablesMap> {
		if self.variables.borrow().contains_key(name) {
			return gc_cell_clone(&self.variables);
		}
		let Some(mut v) = self.parent.as_ref().and_then(|v| Some(gc_cell_clone(v))) else {
			return gc_cell_clone(&self.variables);
		};
		loop {
			let v_borrow = v.borrow();
			if v_borrow.variables.borrow().contains_key(name) {
				return gc_cell_clone(&v_borrow.variables);
			}
			if let Some(parent) = v_borrow.parent.as_ref().and_then(|v| Some(gc_cell_clone(v))) {
				drop(v_borrow);
				v = parent;
			} else {
				return gc_cell_clone(&self.variables);
			}
		}
	}
}

#[derive(Trace, Finalize)]
pub struct GlobalScope {
	pub scope: GcBox<VariableScope>,
	pub res_stack: Vec<PrimitiveValue>,
}

impl GlobalScope {
	pub fn new_gc_from_variables(variables: VariablesMap) -> GcBox<GlobalScope> {
		gc_box_from(GlobalScope {
			scope: VariableScope::new_gc(
				variables,
				None,
			),
			res_stack: Vec::new(),
		})
	}
}

delegate_ivariables_map!(for GlobalScope =>
	&self: self.scope.borrow(),
	&mut self: self.scope.borrow_mut()
);