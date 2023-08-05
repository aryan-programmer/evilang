use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

use delegate::delegate;

use crate::ast::expression::IdentifierT;
use crate::errors::{ErrorT, ResultWithError};
use crate::interpreter::runtime_value::{PrimitiveValue, RcCellValue, RcCellValueExt, RefToValue};
use crate::utils::cell_ref::{rc_cell_from, RcCell};

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

pub struct VariablesMap {
	variables: HashMap<IdentifierT, RcCellValue>,
}

impl VariablesMap {
	pub fn new() -> Self {
		Self { variables: HashMap::new() }
	}
	pub fn new_direct(variables: HashMap<IdentifierT, RcCellValue>) -> Self {
		Self { variables }
	}
	pub fn new_from_primitives(variables: HashMap<IdentifierT, PrimitiveValue>) -> Self {
		Self {
			variables: variables
				.into_iter()
				.map(|(iden, val)| (iden, rc_cell_from(val)))
				.collect(),
		}
	}
}

impl IVariablesMap for VariablesMap {
	fn set_actual(&mut self, name: &IdentifierT, value: RefToValue) -> Option<PrimitiveValue> {
		return if let Some(v) = self.variables.get(name) {
			Some(v.replace(value.consume_or_clone()))
		} else {
			self.variables.insert(name.clone(), rc_cell_from(value.consume_or_clone()));
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
			Some(RefToValue::from_rc(v))
		} else {
			None
		};
	}

	fn contains_key(&self, name: &IdentifierT) -> bool {
		self.variables.contains_key(name)
	}
}

pub struct VariableScope {
	variables: RcCell<VariablesMap>,
	parent: Option<Weak<RefCell<VariableScope>>>,
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
	pub fn new_rc(
		variables: VariablesMap,
		parent: Option<Rc<RefCell<VariableScope>>>,
	) -> RcCell<VariableScope> {
		rc_cell_from(Self {
			variables: rc_cell_from(variables),
			parent: parent.and_then(|v| Some(Rc::downgrade(&v))),
		})
	}

	fn resolve_variable_scope(&self, name: &IdentifierT) -> RcCell<VariablesMap> {
		if self.variables.borrow().contains_key(name) {
			return Rc::clone(&self.variables);
		}
		let Some(mut v) = self.parent.as_ref().and_then(|v| v.upgrade()) else {
			return Rc::clone(&self.variables);
		};
		loop {
			let v_borrow = v.borrow();
			if v_borrow.variables.borrow().contains_key(name) {
				return Rc::clone(&v_borrow.variables);
			}
			if let Some(parent) = v_borrow.parent.as_ref().and_then(|v| v.upgrade()) {
				drop(v_borrow);
				v = parent;
			} else {
				return Rc::clone(&self.variables);
			}
		}
	}
}

pub struct GlobalScope {
	pub scope: RcCell<VariableScope>,
	pub res_stack: Vec<PrimitiveValue>,
}

impl GlobalScope {
	pub fn new_rc_from_variables(variables: VariablesMap) -> RcCell<GlobalScope> {
		rc_cell_from(GlobalScope {
			scope: VariableScope::new_rc(
				variables,
				None,
			),
			res_stack: Vec::new(),
		})
	}
}

impl IVariablesMap for GlobalScope {
	delegate! {
		to self.scope.borrow_mut() {
			fn set_actual(&mut self, name: &IdentifierT, value: RefToValue) -> Option<PrimitiveValue>;
			fn declare(&mut self, name: &IdentifierT, value: RefToValue) -> ResultWithError<()>;
			fn hoist(&mut self, name: &IdentifierT) -> ResultWithError<()>;
		}
		to self.scope.borrow() {
			fn get_actual(&self, name: &IdentifierT) -> Option<RefToValue>;
			fn contains_key(&self, name: &IdentifierT) -> bool;
		}
	}
}
