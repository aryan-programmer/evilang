use std::collections::HashMap;
use std::mem::replace;
use std::ops::{Deref, DerefMut};

use gc::{Finalize, Trace};
use maybe_owned::MaybeOwned;

use crate::ast::expression::IdentifierT;
use crate::errors::{ErrorT, ResultWithError};
use crate::interpreter::runtime_values::{GcPtrVariable, GcPtrVariableExt, PrimitiveValue};
use crate::types::cell_ref::{gc_ptr_cell_from, GcPtr, GcPtrCell};
use crate::types::string::CowStringT;

pub trait IVariablesMapConstMembers {
	fn get_actual(&self, name: CowStringT) -> Option<MaybeOwned<GcPtrVariable>>;
	fn contains_key(&self, name: CowStringT) -> bool;
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
	fn assign(&mut self, name: CowStringT, value: PrimitiveValue) -> Option<PrimitiveValue>;
	fn declare(&mut self, name: CowStringT, value: PrimitiveValue) -> ResultWithError<()>;
	fn hoist(&mut self, name: CowStringT) -> ResultWithError<()>;
}

pub trait IVariablesMapDelegator: IVariablesMapConstMembers {
	fn assign_locally(&self, name: CowStringT, value: PrimitiveValue) -> Option<PrimitiveValue>;
	fn assign(&self, name: CowStringT, value: PrimitiveValue) -> Option<PrimitiveValue>;
	fn declare(&self, name: CowStringT, value: PrimitiveValue) -> ResultWithError<()>;
	fn hoist(&self, name: CowStringT) -> ResultWithError<()>;
}

#[macro_export]
macro_rules! delegate_ivariables_map {
	(for $for_type: ty => &$self: ident: $const_delegator: expr, &$mut_self: ident: (mut) $mut_delegator: expr) => {
		impl IVariablesMapConstMembers for $for_type {
			#[inline(always)]
			fn get_actual(&$self, name: CowStringT) -> Option<::maybe_owned::MaybeOwned<GcPtrVariable>> {
				return $const_delegator.get_actual(name);
			}
			#[inline(always)]
			fn contains_key(&$self, name: CowStringT) -> bool {
				return $const_delegator.contains_key(name);
			}
		}
		impl IVariablesMapDelegator for $for_type {
			#[inline(always)]
			fn assign(&$mut_self, name: CowStringT, value: PrimitiveValue) -> Option<PrimitiveValue> {
				return $mut_delegator.assign(name, value);
			}
			#[inline(always)]
			fn assign_locally(&$mut_self, name: CowStringT, value: PrimitiveValue) -> Option<PrimitiveValue> {
				return $mut_delegator.assign_locally(name, value);
			}
			#[inline(always)]
			fn declare(&$mut_self, name: CowStringT, value: PrimitiveValue) -> ResultWithError<()> {
				return $mut_delegator.declare(name, value);
			}
			#[inline(always)]
			fn hoist(&$mut_self, name: CowStringT) -> ResultWithError<()> {
				return $mut_delegator.hoist(name);
			}
		}
	};
}

pub use delegate_ivariables_map;

pub type GcPtrMutCellToVariablesMap = GcPtr<GcPtrCell<VariablesMap>>;

#[derive(Debug, PartialEq, Trace, Finalize)]
pub struct VariablesMap {
	pub variables: HashMap<IdentifierT, GcPtrVariable>,
}

impl VariablesMap {
	#[inline(always)]
	pub fn new() -> Self {
		Self { variables: HashMap::new() }
	}
	#[inline(always)]
	pub fn new_direct(variables: HashMap<IdentifierT, GcPtrVariable>) -> Self {
		Self { variables }
	}
	pub fn new_from_primitives(variables: HashMap<IdentifierT, PrimitiveValue>) -> Self {
		Self {
			variables: variables
				.into_iter()
				.map(|(iden, val)| (iden, gc_ptr_cell_from(val)))
				.collect(),
		}
	}
}

impl IVariablesMapConstMembers for VariablesMap {
	fn get_actual(&self, name: CowStringT) -> Option<MaybeOwned<GcPtrVariable>> {
		return if let Some(v) = self.variables.get(name.deref()) {
			Some(v.into())
		} else {
			None
		};
	}

	#[inline(always)]
	fn contains_key(&self, name: CowStringT) -> bool {
		self.variables.contains_key(name.deref())
	}
}

impl IVariablesMap for VariablesMap {
	fn assign(&mut self, name: CowStringT, value: PrimitiveValue) -> Option<PrimitiveValue> {
		return if let Some(v) = self.variables.get(name.deref()) {
			let mut res = v.borrow_mut();
			Some(replace(res.deref_mut(), value))
		} else {
			self.variables.insert(name.into(), gc_ptr_cell_from(value));
			None
		};
	}

	fn declare(&mut self, name: CowStringT, value: PrimitiveValue) -> ResultWithError<()> {
		if let Some(v) = self.variables.get(name.deref()) {
			if !v.is_hoisted() {
				return Err(ErrorT::CantRedeclareVariable(name.into()).into());
			}
		}
		if value.is_hoisted() {
			return Err(ErrorT::CantSetToHoistedValue.into());
		}
		self.assign(name, value);
		return Ok(());
	}

	fn hoist(&mut self, name: CowStringT) -> ResultWithError<()> {
		if let Some(_v) = self.variables.get(name.deref()) {
			return Err(ErrorT::CantRedeclareVariable(name.into()).into());
		}
		self.assign(name, PrimitiveValue::_HoistedVariable.into());
		return Ok(());
	}
}
