use std::collections::HashMap;
use std::mem::replace;
use std::ops::DerefMut;

use gc::{Finalize, Trace};

use crate::ast::expression::IdentifierT;
use crate::errors::{ErrorT, ResultWithError};
use crate::interpreter::runtime_values::{GcBoxOfPrimitiveValueExt, PrimitiveValue};
use crate::interpreter::utils::cell_ref::{gc_box_from, gc_cell_clone, GcBox};

pub trait IVariablesMapConstMembers {
	fn get_actual(&self, name: &IdentifierT) -> Option<GcBox<PrimitiveValue>>;
	fn contains_key(&self, name: &IdentifierT) -> bool;
}

pub trait IVariablesMap: IVariablesMapConstMembers {
	///
	///
	/// # Arguments
	///
	/// * `name`:
	/// * `value`:
	///
	/// returns: Option<PrimitiveValue> The previous value stored in the variable
	fn assign(&mut self, name: &IdentifierT, value: PrimitiveValue) -> Option<PrimitiveValue>;
	fn declare(&mut self, name: &IdentifierT, value: PrimitiveValue) -> ResultWithError<()>;
	fn hoist(&mut self, name: &IdentifierT) -> ResultWithError<()>;
}

pub trait IVariablesMapDelegator: IVariablesMapConstMembers {
	fn assign_locally(&self, name: &IdentifierT, value: PrimitiveValue) -> Option<PrimitiveValue>;
	fn assign(&self, name: &IdentifierT, value: PrimitiveValue) -> Option<PrimitiveValue>;
	fn declare(&self, name: &IdentifierT, value: PrimitiveValue) -> ResultWithError<()>;
	fn hoist(&self, name: &IdentifierT) -> ResultWithError<()>;
}

#[macro_export]
macro_rules! delegate_ivariables_map {
	(for $for_type: ty => &$self: ident: $const_delegator: expr, &$mut_self: ident: (mut) $mut_delegator: expr) => {
		impl IVariablesMapConstMembers for $for_type {
			#[inline(always)]
			fn get_actual(&$self, name: &IdentifierT) -> Option<GcBox<PrimitiveValue>> {
				return $const_delegator.get_actual(name);
			}
			#[inline(always)]
			fn contains_key(&$self, name: &IdentifierT) -> bool {
				return $const_delegator.contains_key(name);
			}
		}
		impl IVariablesMapDelegator for $for_type {
			#[inline(always)]
			fn assign(&$mut_self, name: &IdentifierT, value: PrimitiveValue) -> Option<PrimitiveValue> {
				return $mut_delegator.assign(name, value);
			}
			#[inline(always)]
			fn assign_locally(&$mut_self, name: &IdentifierT, value: PrimitiveValue) -> Option<PrimitiveValue> {
				return $mut_delegator.assign_locally(name, value);
			}
			#[inline(always)]
			fn declare(&$mut_self, name: &IdentifierT, value: PrimitiveValue) -> ResultWithError<()> {
				return $mut_delegator.declare(name, value);
			}
			#[inline(always)]
			fn hoist(&$mut_self, name: &IdentifierT) -> ResultWithError<()> {
				return $mut_delegator.hoist(name);
			}
		}
	};
}

pub use delegate_ivariables_map;

#[derive(Debug, PartialEq, Trace, Finalize)]
pub struct VariablesMap {
	pub variables: HashMap<IdentifierT, GcBox<PrimitiveValue>>,
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

impl IVariablesMapConstMembers for VariablesMap {
	fn get_actual(&self, name: &IdentifierT) -> Option<GcBox<PrimitiveValue>> {
		return if let Some(v) = self.variables.get(name) {
			Some(gc_cell_clone(v))
		} else {
			None
		};
	}

	#[inline(always)]
	fn contains_key(&self, name: &IdentifierT) -> bool {
		self.variables.contains_key(name)
	}
}

impl IVariablesMap for VariablesMap {
	fn assign(&mut self, name: &IdentifierT, value: PrimitiveValue) -> Option<PrimitiveValue> {
		return if let Some(v) = self.variables.get(name) {
			let mut res = v.borrow_mut();
			Some(replace(res.deref_mut(), value))
		} else {
			self.variables.insert(name.clone(), gc_box_from(value));
			None
		};
	}

	fn declare(&mut self, name: &IdentifierT, value: PrimitiveValue) -> ResultWithError<()> {
		if let Some(v) = self.variables.get(name) {
			if !v.is_hoisted() {
				return Err(ErrorT::CantRedeclareVariable(name.clone()).into());
			}
		}
		if value.is_hoisted() {
			return Err(ErrorT::CantSetToHoistedValue.into());
		}
		self.assign(name, value);
		return Ok(());
	}

	fn hoist(&mut self, name: &IdentifierT) -> ResultWithError<()> {
		if let Some(_v) = self.variables.get(name) {
			return Err(ErrorT::CantRedeclareVariable(name.clone()).into());
		}
		self.assign(name, PrimitiveValue::_HoistedVariable.into());
		return Ok(());
	}
}
