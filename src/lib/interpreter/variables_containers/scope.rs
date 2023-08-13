use delegate::delegate;
use gc::{Finalize, Trace};

use crate::ast::expression::IdentifierT;
use crate::errors::ResultWithError;
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::runtime_values::ref_to_value::RefToValue;
use crate::interpreter::variables_containers::map::{IVariablesMapConstMembers, IVariablesMapDelegator, VariablesMap};
use crate::interpreter::variables_containers::map::IVariablesMap;
use crate::utils::cell_ref::{gc_box_from, gc_cell_clone, GcBox};

pub trait IGenericVariablesScope<T: IGenericVariablesScope<T> + 'static>: Trace + Finalize {
	fn get_variables(&self) -> GcBox<VariablesMap>;
	fn get_parent(&self) -> Option<GcBox<T>>;

	fn resolve_variable_scope(&self, name: &IdentifierT) -> GcBox<VariablesMap> {
		let self_vars = self.get_variables();
		if self_vars.borrow().contains_key(name) {
			return self_vars;
		}
		let mut parent_opt = self.get_parent();
		while let Some(mut parent) = parent_opt {
			let v_borrow = parent.borrow();
			let v_vars = v_borrow.get_variables();
			if v_vars.borrow().contains_key(name) {
				return v_vars;
			}
			parent_opt = v_borrow.get_parent();
		}
		return self_vars;
	}
}

impl<T: IGenericVariablesScope<T> + 'static> IVariablesMapConstMembers for T {
	delegate! {
		to self.resolve_variable_scope(name).borrow() {
			fn get_actual(&self, name: &IdentifierT) -> Option<GcBox<PrimitiveValue>>;
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

#[derive(PartialEq, Trace, Finalize)]
pub struct VariableScope {
	pub variables: GcBox<VariablesMap>,
	pub parent: Option<GcBox<VariableScope>>,
}

impl IGenericVariablesScope<VariableScope> for VariableScope {
	#[inline(always)]
	fn get_variables(&self) -> GcBox<VariablesMap> {
		gc_cell_clone(&self.variables)
	}

	#[inline(always)]
	fn get_parent(&self) -> Option<GcBox<VariableScope>> {
		self.parent.clone()
	}
}

impl VariableScope {
	pub fn new_gc(
		variables: VariablesMap,
		parent: Option<GcBox<VariableScope>>,
	) -> GcBox<VariableScope> {
		gc_box_from(Self {
			variables: gc_box_from(variables),
			parent,
		})
	}
}
