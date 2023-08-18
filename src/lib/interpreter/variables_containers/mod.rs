use gc::{Finalize, Trace};

pub use map::VariablesMap;
pub use scope::VariableScope;

use crate::errors::ResultWithError;
use crate::interpreter::environment::resolver::BoxIResolver;
use crate::interpreter::runtime_values::{GcPtrVariable, PrimitiveValue};
use crate::interpreter::utils::cell_ref::{gc_ptr_cell_from, GcPtr, GcPtrCell};
use crate::interpreter::variables_containers::map::{delegate_ivariables_map, IVariablesMapConstMembers, IVariablesMapDelegator};
use crate::interpreter::variables_containers::scope::GcPtrToVariableScope;
use crate::types::string::CowStringT;

pub mod map;
pub mod scope;

pub type GcPtrMutCellToGlobalScope = GcPtr<GcPtrCell<GlobalScope>>;

#[derive(PartialEq, Trace, Finalize)]
pub struct GlobalScope {
	pub scope: GcPtrToVariableScope,
	pub res_stack: Vec<PrimitiveValue>,
	pub resolver: BoxIResolver,
}

impl GlobalScope {
	pub fn new_gc_from_variables(variables: VariablesMap, resolver: BoxIResolver) -> GcPtrMutCellToGlobalScope {
		gc_ptr_cell_from(GlobalScope {
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
	&self: self.scope,
	&self: (mut) self.scope
);
