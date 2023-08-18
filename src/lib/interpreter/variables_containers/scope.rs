use std::ops::Deref;

use delegate::delegate;
use gc::{Finalize, Trace};

use crate::errors::ResultWithError;
use crate::interpreter::runtime_values::{GcPtrVariable, PrimitiveValue};
use crate::interpreter::utils::cell_ref::{gc_clone, gc_ptr_cell_from, GcPtr};
use crate::interpreter::variables_containers::map::{GcPtrMutCellToVariablesMap, IVariablesMapConstMembers, IVariablesMapDelegator, VariablesMap};
use crate::interpreter::variables_containers::map::IVariablesMap;
use crate::types::string::CowStringT;

pub trait IGenericVariablesScope<T: IGenericVariablesScope<T> + 'static>: Trace + Finalize {
	fn get_variables(&self) -> GcPtrMutCellToVariablesMap;
	fn get_parent(&self) -> Option<GcPtr<T>>;

	fn resolve_variable_scope(&self, name: CowStringT) -> GcPtrMutCellToVariablesMap {
		let self_vars = self.get_variables();
		let name_ref = name.deref();
		if self_vars.borrow().contains_key(name_ref.into()) {
			return self_vars;
		}
		let mut parent_opt = self.get_parent();
		while let Some(parent) = parent_opt {
			let v_vars = parent.get_variables();
			if v_vars.borrow().contains_key(name_ref.into()) {
				return v_vars;
			}
			parent_opt = parent.get_parent();
		}
		return self_vars;
	}
}

impl<T: IGenericVariablesScope<T> + 'static> IVariablesMapConstMembers for T {
	delegate! {
		to self.resolve_variable_scope(name.deref().into()).borrow() {
			fn get_actual(&self, name: CowStringT) -> Option<GcPtrVariable>;
			fn contains_key(&self, name: CowStringT) -> bool;
		}
	}
}

impl<T: IGenericVariablesScope<T> + 'static> IVariablesMapDelegator for T {
	delegate! {
		to self.resolve_variable_scope(name.deref().into()).borrow_mut() {
			fn assign(&self, name: CowStringT, value: PrimitiveValue) -> Option<PrimitiveValue>;
		}
		to self.get_variables().borrow_mut() {
			#[call(assign)]
			fn assign_locally(&self, name: CowStringT, value: PrimitiveValue) -> Option<PrimitiveValue>;
			fn declare(&self, name: CowStringT, value: PrimitiveValue) -> ResultWithError<()>;
			fn hoist(&self, name: CowStringT) -> ResultWithError<()>;
		}
	}
}

pub type GcPtrToVariableScope = GcPtr<VariableScope>;

#[derive(PartialEq, Trace, Finalize)]
pub struct VariableScope {
	pub variables: GcPtrMutCellToVariablesMap,
	pub parent: Option<GcPtrToVariableScope>,
}

impl IGenericVariablesScope<VariableScope> for VariableScope {
	#[inline(always)]
	fn get_variables(&self) -> GcPtrMutCellToVariablesMap {
		gc_clone(&self.variables)
	}

	#[inline(always)]
	fn get_parent(&self) -> Option<GcPtrToVariableScope> {
		self.parent.clone()
	}
}

impl VariableScope {
	#[inline(always)]
	pub fn new_gc(variables: GcPtrMutCellToVariablesMap, parent: Option<GcPtrToVariableScope>) -> GcPtrToVariableScope {
		GcPtr::new(Self { variables, parent })
	}

	pub fn new_gc_from_map(
		variables: VariablesMap,
		parent: Option<GcPtrToVariableScope>,
	) -> GcPtrToVariableScope {
		GcPtr::new(Self {
			variables: gc_ptr_cell_from(variables),
			parent,
		})
	}
}
