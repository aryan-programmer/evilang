use std::ops::DerefMut;

use gc::{Finalize, Trace};

pub use map::VariablesMap;
pub use scope::VariableScope;

use crate::ast::expression::IdentifierT;
use crate::errors::ResultWithError;
use crate::interpreter::runtime_values::{GcBoxOfPrimitiveValueExt, PrimitiveValue};
use crate::interpreter::runtime_values::ref_to_value::RefToValue;
use crate::interpreter::variables_containers::map::{delegate_ivariables_map, IVariablesMapConstMembers, IVariablesMapDelegator};
use crate::utils::cell_ref::{gc_box_from, GcBox};

pub mod map;
pub mod scope;

#[derive(PartialEq, Trace, Finalize)]
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
	&self: (mut) self.scope.borrow_mut()
);
