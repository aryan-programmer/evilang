use gc::{Finalize, Trace};

pub use map::VariablesMap;
pub use scope::VariableScope;

use crate::ast::expression::IdentifierT;
use crate::errors::ResultWithError;
use crate::interpreter::environment::resolver::BoxIResolver;
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::utils::cell_ref::{gc_box_from, GcBox};
use crate::interpreter::variables_containers::map::{delegate_ivariables_map, IVariablesMapConstMembers, IVariablesMapDelegator};

pub mod map;
pub mod scope;

#[derive(PartialEq, Trace, Finalize)]
pub struct GlobalScope {
	pub scope: GcBox<VariableScope>,
	pub res_stack: Vec<PrimitiveValue>,
	pub resolver: BoxIResolver,
}

impl GlobalScope {
	pub fn new_gc_from_variables(variables: VariablesMap, resolver: BoxIResolver) -> GcBox<GlobalScope> {
		gc_box_from(GlobalScope {
			scope: VariableScope::new_gc_from_map(
				variables,
				None,
			),
			res_stack: Vec::new(),
			resolver,
		})
	}
}

delegate_ivariables_map!(for GlobalScope =>
	&self: self.scope.borrow(),
	&self: (mut) self.scope.borrow_mut()
);
