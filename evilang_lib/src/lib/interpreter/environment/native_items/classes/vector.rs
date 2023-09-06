use gc::{Finalize, GcCell, Trace};

use evilang_traits::derive_build_class;

use crate::errors::ResultWithError;
use crate::implement_get_class_cached;
use crate::interpreter::environment::Environment;
use crate::interpreter::environment::native_items::classes::object::ObjectSuperclass;
use crate::interpreter::runtime_values::functions::GcPtrToFunction;
use crate::interpreter::runtime_values::functions::ifunction::IFunction;
use crate::interpreter::runtime_values::i_native_struct::{auto_unwrap_exec_fn, INativeClass, INativeClass_BuildClass, INativeClass_GetClassCached, native_wrap, NativeClassMemberFunctionContext, NativeClassStaticFunctionContext};
use crate::interpreter::runtime_values::i_native_struct::INativeClass_IsStructWrapper;
use crate::interpreter::runtime_values::i_native_struct::INativeStruct;
use crate::interpreter::runtime_values::objects::runtime_object::{GcPtrToObject, RuntimeObject};
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::types::number::NumberT;

#[derive(Debug, Clone, Trace, Finalize)]
pub struct Vector {
	vec: Vec<PrimitiveValue>,
}

impl INativeClass for Vector {
	const NAME: &'static str = "Vector";

	fn get_parent_class(env: &mut Environment) -> ResultWithError<Option<GcPtrToObject>> {
		Ok(Some(ObjectSuperclass::get_class_cached(env)?))
	}
}

implement_get_class_cached!(Vector);

impl INativeStruct for Vector {}

#[derive_build_class(evilang_lib_crate = crate)]
impl Vector {
	#[export = "constructor"]
	pub fn constructor(_ctx: NativeClassMemberFunctionContext) -> ResultWithError<Self> {
		return Ok(Self {
			vec: vec![],
		});
	}

	#[export(raw)]
	pub fn from(env: &mut Environment, params: Vec<PrimitiveValue>) -> ResultWithError<PrimitiveValue> {
		let res = Self {
			vec: params
		};
		let obj = RuntimeObject::allocate_instance(
			Vector::get_class_cached(env)?,
			None,
		);
		native_wrap(&obj, Vector::NATIVE_BOX_WRAP_NAME.into(), res);
		return Ok(obj.into());
	}

	#[export]
	pub fn repeat(ctx: NativeClassStaticFunctionContext, v: PrimitiveValue, n: NumberT) -> ResultWithError<PrimitiveValue> {
		let res = Self {
			vec: (0..(n.floor_to_int() as usize)).map(|_i| v.try_clone_err()).collect::<ResultWithError<Vec<_>>>()?
		};
		let obj = RuntimeObject::allocate_instance(
			Vector::get_class_cached(ctx.env)?,
			None,
		);
		native_wrap(&obj, Vector::NATIVE_BOX_WRAP_NAME.into(), res);
		return Ok(obj.into());
	}

	#[export]
	pub fn from_exec_n(ctx: NativeClassStaticFunctionContext, n: NumberT, func: GcPtrToFunction) -> ResultWithError<PrimitiveValue> {
		let res = Self {
			vec: (0..(n.floor_to_int() as usize)).map(|i| func.execute(ctx.env, vec![
				NumberT::from(i as i128).into(),
			])).collect::<ResultWithError<Vec<_>>>()?
		};
		let obj = RuntimeObject::allocate_instance(
			Vector::get_class_cached(ctx.env)?,
			None,
		);
		native_wrap(&obj, Vector::NATIVE_BOX_WRAP_NAME.into(), res);
		return Ok(obj.into());
	}

	#[export]
	#[inline]
	pub fn capacity(&self, _ctx: NativeClassMemberFunctionContext) -> ResultWithError<NumberT> {
		return Ok((self.vec.capacity() as i128).into());
	}

	#[export]
	#[inline]
	pub fn len(&self, _ctx: NativeClassMemberFunctionContext) -> ResultWithError<NumberT> {
		return Ok((self.vec.len() as i128).into());
	}

	#[export]
	#[inline]
	pub fn clear(&mut self, _ctx: NativeClassMemberFunctionContext) -> ResultWithError<PrimitiveValue> {
		self.vec.clear();
		return Ok(PrimitiveValue::Null);
	}

	#[export]
	#[inline]
	pub fn insert(&mut self, _ctx: NativeClassMemberFunctionContext, at: NumberT, value: PrimitiveValue) -> ResultWithError<PrimitiveValue> {
		self.vec.insert(at.floor_to_int() as usize, value);
		return Ok(PrimitiveValue::Null);
	}

	#[export]
	#[inline]
	pub fn push(&mut self, _ctx: NativeClassMemberFunctionContext, v: PrimitiveValue) -> ResultWithError<PrimitiveValue> {
		self.vec.push(v);
		return Ok(PrimitiveValue::Null);
	}

	#[export]
	#[inline]
	pub fn remove(&mut self, _ctx: NativeClassMemberFunctionContext, at: NumberT) -> ResultWithError<PrimitiveValue> {
		return Ok(self.vec.remove(at.floor_to_int() as usize));
	}

	#[export]
	#[inline]
	pub fn pop(&mut self, _ctx: NativeClassMemberFunctionContext) -> ResultWithError<Option<PrimitiveValue>> {
		return Ok(self.vec.pop());
	}

	#[export]
	#[inline]
	pub fn reserve(&mut self, _ctx: NativeClassMemberFunctionContext, extra: NumberT) -> ResultWithError<PrimitiveValue> {
		self.vec.reserve(extra.floor_to_int() as usize);
		return Ok(PrimitiveValue::Null);
	}

	#[export]
	#[inline]
	pub fn resize(&mut self, _ctx: NativeClassMemberFunctionContext, new_len: NumberT, value: PrimitiveValue) -> ResultWithError<PrimitiveValue> {
		self.vec.resize(new_len.floor_to_int() as usize, value);
		return Ok(PrimitiveValue::Null);
	}

	#[export]
	#[inline]
	pub fn get(&self, _ctx: NativeClassMemberFunctionContext, at: NumberT) -> ResultWithError<Option<PrimitiveValue>> {
		return Ok(match self.vec.get(at.floor_to_int() as usize) {
			None => None,
			Some(v) => Some(v.try_clone_err()?)
		});
	}

	#[export]
	#[inline]
	pub fn set(&mut self, _ctx: NativeClassMemberFunctionContext, at: NumberT, value: PrimitiveValue) -> ResultWithError<PrimitiveValue> {
		self.vec[at.floor_to_int() as usize] = value;
		return Ok(PrimitiveValue::Null);
	}

	#[export]
	pub fn for_each(&self, ctx: NativeClassMemberFunctionContext, func: GcPtrToFunction) -> ResultWithError<PrimitiveValue> {
		for (i, v) in self.vec.iter().enumerate() {
			func.execute(ctx.env, vec![
				v.try_clone_err()?,
				NumberT::from(i as i128).into(),
			])?;
		};
		return Ok(PrimitiveValue::Null);
	}

	#[export]
	pub fn reduce(&self, ctx: NativeClassMemberFunctionContext, mut initial: PrimitiveValue, func: GcPtrToFunction) -> ResultWithError<PrimitiveValue> {
		for (i, v) in self.vec.iter().enumerate() {
			let new_value = func.execute(ctx.env, vec![
				initial,
				v.try_clone_err()?,
				NumberT::from(i as i128).into(),
			])?;
			initial = new_value;
		};
		return Ok(initial);
	}

	#[export]
	pub fn map_inline(&mut self, ctx: NativeClassMemberFunctionContext, func: GcPtrToFunction) -> ResultWithError<PrimitiveValue> {
		for (i, v) in self.vec.iter_mut().enumerate() {
			let value = v.try_clone_err()?;
			*v = func.execute(ctx.env, vec![
				value,
				NumberT::from(i as i128).into(),
			])?;
		};
		return Ok(ctx.this_param.try_clone_err()?);
	}

	#[export]
	pub fn map(&self, ctx: NativeClassMemberFunctionContext, func: GcPtrToFunction) -> ResultWithError<PrimitiveValue> {
		let res = Self {
			vec: self.vec.iter().enumerate().map(|(i, value)| -> ResultWithError<PrimitiveValue>{
				func.execute(ctx.env, vec![
					value.try_clone_err()?,
					NumberT::from(i as i128).into(),
				])
			}).collect::<ResultWithError<Vec<_>>>()?
		};
		let obj = RuntimeObject::allocate_instance(
			Vector::get_class_cached(ctx.env)?,
			None,
		);
		native_wrap(&obj, Vector::NATIVE_BOX_WRAP_NAME.into(), res);
		return Ok(obj.into());
	}

	#[export]
	pub fn clone(&self, ctx: NativeClassMemberFunctionContext) -> ResultWithError<PrimitiveValue> {
		let res = Self {
			vec: self.vec.clone()
		};
		let obj = RuntimeObject::allocate_instance(
			Vector::get_class_cached(ctx.env)?,
			None,
		);
		native_wrap(&obj, Vector::NATIVE_BOX_WRAP_NAME.into(), res);
		return Ok(obj.into());
	}

	#[export]
	pub fn equals(&self, _ctx: NativeClassMemberFunctionContext, oth: PrimitiveValue) -> ResultWithError<bool> {
		return auto_unwrap_exec_fn(
			&oth,
			|v: &GcCell<Vector>| Ok(self.vec == v.borrow().vec),
			|| "other parameter of Vector::equals".into(),
		);
	}
}
