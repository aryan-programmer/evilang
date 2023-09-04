use std::any::Any;
use std::fmt::Debug;
use std::mem::replace;
use std::ops::Deref;

use gc::{Finalize, GcCell, Trace};

use crate::errors::{Descriptor, ErrorT, EvilangError, ResultWithError, RuntimeError};
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::functions::GcPtrToFunction;
use crate::interpreter::runtime_values::objects::runtime_object::GcPtrToObject;
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::runtime_values::ref_to_value::DerefOfRefToValue;
use crate::interpreter::utils::expect_object_fn;
use crate::interpreter::variables_containers::map::{IVariablesMapConstMembers, IVariablesMapDelegator};
use crate::types::cell_ref::{gc_clone, gc_ptr_cell_from, GcPtr};
use crate::types::number::NumberT;
use crate::types::string::{CowStringT, StringT};

#[macro_export]
macro_rules! implement_get_class_cached {
	($t: ty) => {
		impl $t {
			thread_local! {
				static __CLASS_DEFINITION_OBJECT_CACHE: ::std::cell::OnceCell<$crate::interpreter::runtime_values::objects::runtime_object::GcPtrToObject> = ::std::cell::OnceCell::new();
			}
		}

		impl INativeClass_GetClassCached for $t {
			fn get_class_cached(env: &mut Environment) -> ResultWithError<GcPtrToObject> {
				Self::__CLASS_DEFINITION_OBJECT_CACHE.with(|cell| -> $crate::errors::ResultWithError<$crate::interpreter::runtime_values::objects::runtime_object::GcPtrToObject> {
					return Ok($crate::types::cell_ref::gc_clone(match cell.get() {
						None => {
							let rv = Self::build_class(env)?;
							cell.get_or_init(move || rv)
						}
						Some(rv) => rv
					}));
				})
			}
		}
	};
}

pub use implement_get_class_cached;

#[allow(non_camel_case_types)]
pub trait INativeClass_GetClassCached {
	fn get_class_cached(env: &mut Environment) -> ResultWithError<GcPtrToObject>;
}

#[allow(non_camel_case_types)]
pub trait INativeClass_IsStructWrapper: INativeClass {
	const NATIVE_BOX_WRAP_NAME: &'static str;
}

#[allow(non_camel_case_types)]
pub trait INativeClass_BuildClass {
	fn build_class(env: &mut Environment) -> ResultWithError<GcPtrToObject>;
}

pub trait INativeClass: INativeClass_GetClassCached + INativeClass_BuildClass {
	const NAME: &'static str;

	fn get_parent_class(env: &mut Environment) -> ResultWithError<Option<GcPtrToObject>>;
}

pub type GcPtrToNativeStruct = GcPtr<dyn INativeStruct>;

pub trait INativeStruct: Debug + Trace + Finalize + Any + INativeStructExtras {}

pub trait INativeStructExtras {
	fn as_any(&self) -> &dyn Any;
}

impl<T> INativeStructExtras for T where T: 'static + INativeStruct {
	#[inline(always)]
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl<T: INativeStruct> INativeStruct for GcCell<T> {}

pub fn native_wrap<TSelf: INativeStruct>(object: &GcPtrToObject, box_wrap_name: CowStringT, val: TSelf) {
	let gc_ptr_cell = gc_ptr_cell_from(val);
	let cell_ptr = GcPtr::into_raw(gc_ptr_cell);
	let new_gc = unsafe { GcPtrToNativeStruct::from_raw(cell_ptr) };
	object.assign_locally(box_wrap_name, PrimitiveValue::NativeStruct(new_gc));
}

pub fn native_unwrap_exec_fn<TSelf: INativeStruct, TExecFn, TExecFnRes, TNameFn>(
	object: &PrimitiveValue,
	box_wrap_name: CowStringT,
	exec_fn: TExecFn,
	this_param_name_fn: TNameFn,
) -> ResultWithError<TExecFnRes>
	where TExecFn: FnOnce(&GcCell<TSelf>) -> ResultWithError<TExecFnRes>,
	      TNameFn: Fn() -> CowStringT<'static> {
	let err_f = || EvilangError::new(ErrorT::UnexpectedRuntimeError(RuntimeError::ExpectedNativeObject(Descriptor::NameAndValue {
		name: this_param_name_fn().to_string(),
		value: object.clone__silently_fail(),
	})));
	let ref_to_object_borr = DerefOfRefToValue::DerefRValue(object);
	let object_class_ref = expect_object_fn(ref_to_object_borr.deref(), || Descriptor::Name(this_param_name_fn().to_string()))?;
	let native_box_var = object_class_ref.get_actual(box_wrap_name).ok_or_else(err_f)?;
	let native_box_var_borr = native_box_var.deref().borrow();
	let PrimitiveValue::NativeStruct(native_box_ptr) = native_box_var_borr.deref() else {
		return Err(err_f());
	};
	let native_box_cell = native_box_ptr.deref().as_any().downcast_ref::<GcCell<TSelf>>().ok_or_else(err_f)?;
	return exec_fn(native_box_cell);
}

#[inline(always)]
pub fn auto_unwrap_exec_fn<TSelf: INativeStruct + INativeClass_IsStructWrapper, TExecFn, TExecFnRes, TNameFn>(
	object: &PrimitiveValue,
	exec_fn: TExecFn,
	this_param_name_fn: TNameFn,
) -> ResultWithError<TExecFnRes>
	where TExecFn: FnOnce(&GcCell<TSelf>) -> ResultWithError<TExecFnRes>,
	      TNameFn: Fn() -> CowStringT<'static> {
	return native_unwrap_exec_fn(
		object,
		<TSelf as INativeClass_IsStructWrapper>::NATIVE_BOX_WRAP_NAME.into(),
		exec_fn,
		this_param_name_fn,
	);
}

pub struct NativeClassMemberFunctionContext<'a, 'b> {
	pub env: &'a mut Environment,
	pub this_param: &'b PrimitiveValue,
}

impl<'a, 'b> NativeClassMemberFunctionContext<'a, 'b> {
	#[inline(always)]
	pub fn new(env: &'a mut Environment, this_param: &'b PrimitiveValue) -> Self {
		Self { env, this_param }
	}
}

pub struct NativeClassStaticFunctionContext<'a> {
	pub env: &'a mut Environment,
}

impl<'a, 'b> NativeClassStaticFunctionContext<'a> {
	#[inline(always)]
	pub fn new(env: &'a mut Environment) -> Self {
		Self { env }
	}
}

pub trait FromOptionOfPrimitiveValue: Sized {
	fn from_option_of_primitive_value(v: Option<PrimitiveValue>) -> ResultWithError<Self>;
}

#[inline(always)]
pub fn from_option_of_primitive_value<U: FromOptionOfPrimitiveValue>(v: Option<PrimitiveValue>) -> ResultWithError<U> {
	U::from_option_of_primitive_value(v)
}

impl FromOptionOfPrimitiveValue for PrimitiveValue {
	#[inline(always)]
	fn from_option_of_primitive_value(v: Option<PrimitiveValue>) -> ResultWithError<Self> {
		return v.ok_or_else(|| EvilangError::new(ErrorT::UnexpectedRuntimeError(RuntimeError::UnexpectedNullValue(Descriptor::None))));
	}
}

impl<T: FromOptionOfPrimitiveValue> FromOptionOfPrimitiveValue for Option<T> {
	#[inline(always)]
	fn from_option_of_primitive_value(v: Option<PrimitiveValue>) -> ResultWithError<Self> {
		return match v {
			None => Ok(None),
			v => T::from_option_of_primitive_value(v).map(|v| Some(v))
		};
	}
}

impl FromOptionOfPrimitiveValue for bool {
	fn from_option_of_primitive_value(v: Option<PrimitiveValue>) -> ResultWithError<Self> {
		return match v {
			Some(PrimitiveValue::Boolean(ref v)) => Ok(*v),
			Some(v) => Err(RuntimeError::ExpectedBoolean(Descriptor::Value(v)).into()),
			None => Err(RuntimeError::UnexpectedNullValue(Descriptor::None).into())
		};
	}
}

impl FromOptionOfPrimitiveValue for NumberT {
	fn from_option_of_primitive_value(v: Option<PrimitiveValue>) -> ResultWithError<Self> {
		return match v {
			Some(PrimitiveValue::Number(ref v)) => Ok(*v),
			Some(v) => Err(RuntimeError::ExpectedNumber(Descriptor::Value(v)).into()),
			None => Err(RuntimeError::UnexpectedNullValue(Descriptor::None).into())
		};
	}
}

impl FromOptionOfPrimitiveValue for StringT {
	fn from_option_of_primitive_value(mut v: Option<PrimitiveValue>) -> ResultWithError<Self> {
		return match v {
			Some(PrimitiveValue::String(ref mut v)) => Ok(replace(v, StringT::new())),
			Some(v) => Err(RuntimeError::ExpectedString(Descriptor::Value(v)).into()),
			None => Err(RuntimeError::UnexpectedNullValue(Descriptor::None).into())
		};
	}
}

impl FromOptionOfPrimitiveValue for GcPtrToFunction {
	fn from_option_of_primitive_value(v: Option<PrimitiveValue>) -> ResultWithError<Self> {
		return match v {
			Some(PrimitiveValue::Function(ref v)) => Ok(gc_clone(v)),
			Some(v) => Err(RuntimeError::ExpectedFunction(Descriptor::Value(v)).into()),
			None => Err(RuntimeError::UnexpectedNullValue(Descriptor::None).into())
		};
	}
}

impl FromOptionOfPrimitiveValue for GcPtrToObject {
	fn from_option_of_primitive_value(v: Option<PrimitiveValue>) -> ResultWithError<Self> {
		return match v {
			Some(v) => Ok(gc_clone(expect_object_fn(&v, || Descriptor::Value(v.clone__silently_fail()))?)),
			None => Err(RuntimeError::UnexpectedNullValue(Descriptor::None).into())
		};
	}
}

