use std::any::Any;
use std::fs;
use std::ops::Deref;
use std::path::{Component, Path, PathBuf};

use gc::{Finalize, Trace};

use crate::ast::statement::StatementList;
use crate::errors::{Descriptor, EvilangError, ResultWithError, RuntimeError};
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_values::PrimitiveValue;
use crate::interpreter::variables_containers::map::IVariablesMapConstMembers;
use crate::parser::parse;
use crate::types::consts::CURRENT_FILE;
use crate::types::string::StringT;

pub struct ResolveResult {
	pub absolute_file_path: StringT,
	pub statements: StatementList,
}

pub type BoxIResolver = Box<dyn IResolver>;

pub trait IResolver: IResolverExtras + Trace + Finalize {
	fn resolve(&self, env: Option<&Environment>, file_name: StringT) -> ResultWithError<ResolveResult>;
}

pub trait IResolverExtras {
	fn clone_box(&self) -> Box<dyn IResolver>;
	fn as_any(&self) -> &dyn Any;
	fn equals_resolver(&self, other: &dyn IResolver) -> bool;
}

impl<T> IResolverExtras for T where T: 'static + IResolver + Clone + PartialEq {
	fn clone_box(&self) -> Box<dyn IResolver> {
		Box::new(self.clone())
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn equals_resolver(&self, other: &dyn IResolver) -> bool {
		other
			.as_any()
			.downcast_ref::<T>()
			.map_or(false, |a| self == a)
	}
}

impl PartialEq for dyn IResolver {
	fn eq(&self, other: &Self) -> bool {
		self.equals_resolver(other)
	}
}

impl PartialEq<&BoxIResolver> for BoxIResolver {
	fn eq(&self, other: &&Self) -> bool {
		self.equals_resolver(&***other)
	}
}

impl Clone for Box<dyn IResolver> {
	fn clone(&self) -> Self {
		self.clone_box()
	}
}

#[derive(Clone, PartialEq, Trace, Finalize)]
pub struct DefaultResolver {}

pub fn normalize_path(path: &Path) -> PathBuf {
	let mut components = path.components().peekable();
	let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
		components.next();
		PathBuf::from(c.as_os_str())
	} else {
		PathBuf::new()
	};

	for component in components {
		match component {
			Component::Prefix(..) => unreachable!(),
			Component::RootDir => {
				ret.push(component.as_os_str());
			}
			Component::CurDir => {}
			Component::ParentDir => {
				ret.pop();
			}
			Component::Normal(c) => {
				ret.push(c);
			}
		}
	}
	ret
}

impl DefaultResolver {
	#[inline(always)]
	pub fn new() -> Self {
		Self {}
	}

	#[inline(always)]
	pub fn new_box() -> BoxIResolver {
		Box::new(Self {})
	}

	pub fn resolve_file_path(env_opt: Option<&Environment>, file_name: StringT) -> ResultWithError<PathBuf> {
		let Some(this_file_path_box) = env_opt
			.and_then(|env| env.get_actual(CURRENT_FILE.into()))
			.and_then(|v| if v.borrow().deref() == &PrimitiveValue::Null { None } else { Some(v) }) else {
			return fs::canonicalize(file_name).map_err(EvilangError::from);
			//return Ok(normalize_path(Path::new(&file_name)).to_path_buf());//.map_err(EvilangError::from);
		};
		let this_file_path_borr = this_file_path_box.borrow();
		let PrimitiveValue::String(this_file_path_str_ref) = this_file_path_borr.deref() else {
			return Err(RuntimeError::ExpectedValidFileName(Descriptor::Value(this_file_path_borr.deref().clone__silently_fail())).into());
		};
		// dbg!(this_file_path_str_ref);
		// let this_file_path_str = this_file_path_str_ref.clone();
		let mut this_file_path_buf = PathBuf::from(this_file_path_str_ref);
		if this_file_path_buf.is_relative() {
			this_file_path_buf = this_file_path_buf.canonicalize().map_err(EvilangError::from)?;
		}
		if this_file_path_buf.is_file() {
			this_file_path_buf.pop();
		}
		this_file_path_buf.push(file_name);
		if this_file_path_buf.is_relative() {
			this_file_path_buf = this_file_path_buf.canonicalize().map_err(EvilangError::from)?;
		}
		return Ok(this_file_path_buf);
	}
}

impl IResolver for DefaultResolver {
	fn resolve(&self, env: Option<&Environment>, file_name: StringT) -> ResultWithError<ResolveResult> {
		// dbg!(&file_name);
		let f_path = DefaultResolver::resolve_file_path(env, file_name)?;
		let absolute_file_path: StringT = match f_path.to_str() {
			None => f_path.to_string_lossy().into(),
			Some(v) => v.into()
		};
		// dbg!(&absolute_file_path);
		let contents: StringT = fs::read_to_string(f_path).map_err(EvilangError::from)?;
		return Ok(ResolveResult {
			statements: parse(contents)?,
			absolute_file_path,
		});
	}
}
