use delegate::delegate;
use gc::{Finalize, Trace};

use crate::ast::expression::IdentifierT;
use crate::errors::ResultWithError;
use crate::interpreter::runtime_values::{GcPtrVariable, PrimitiveValue};
use crate::interpreter::utils::cell_ref::{gc_ptr_cell_from, gc_clone, GcPtr};
use crate::interpreter::variables_containers::map::{GcPtrMutCellToVariablesMap, IVariablesMapConstMembers, IVariablesMapDelegator, VariablesMap};
use crate::interpreter::variables_containers::map::IVariablesMap;

pub trait IGenericVariablesScope<T: IGenericVariablesScope<T> + 'static>: Trace + Finalize {
	fn get_variables(&self) -> GcPtrMutCellToVariablesMap;
	fn get_parent(&self) -> Option<GcPtr<T>>;

	fn resolve_variable_scope(&self, name: &IdentifierT) -> GcPtrMutCellToVariablesMap {
		let self_vars = self.get_variables();
		if self_vars.borrow().contains_key(name) {
			return self_vars;
		}
		let mut parent_opt = self.get_parent();
		while let Some(parent) = parent_opt {
			let v_vars = parent.get_variables();
			if v_vars.borrow().contains_key(name) {
				return v_vars;
			}
			parent_opt = parent.get_parent();
		}
		return self_vars;
	}
}

impl<T: IGenericVariablesScope<T> + 'static> IVariablesMapConstMembers for T {
	delegate! {
		to self.resolve_variable_scope(name).borrow() {
			fn get_actual(&self, name: &IdentifierT) -> Option<GcPtrVariable>;
			fn contains_key(&self, name: &IdentifierT) -> bool;
		}
	}
}

impl<T: IGenericVariablesScope<T> + 'static> IVariablesMapDelegator for T {
	delegate! {
		to self.resolve_variable_scope(name).borrow_mut() {
			fn assign(&self, name: &IdentifierT, value: PrimitiveValue) -> Option<PrimitiveValue>;
		}
		to self.get_variables().borrow_mut() {
			#[call(assign)]
			fn assign_locally(&self, name: &IdentifierT, value: PrimitiveValue) -> Option<PrimitiveValue>;
			fn declare(&self, name: &IdentifierT, value: PrimitiveValue) -> ResultWithError<()>;
			fn hoist(&self, name: &IdentifierT) -> ResultWithError<()>;
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
