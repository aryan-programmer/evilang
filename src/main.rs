#![allow(dead_code)]

use std::error::Error;
use std::ops::Deref;

use clap::Parser;

use evilang_lib::ast::statement::StatementList;
use evilang_lib::ast::structs::FunctionDeclaration;
use evilang_lib::interpreter::environment::Environment;
use evilang_lib::interpreter::environment::resolver::DefaultResolver;
use evilang_lib::interpreter::runtime_values::functions::{Function, GcPtrToFunction};
use evilang_lib::interpreter::runtime_values::functions::closure::Closure;
use evilang_lib::interpreter::runtime_values::functions::native_function::NativeFunction;
use evilang_lib::interpreter::runtime_values::objects::runtime_object::{GcPtrToObject, RuntimeObject};
use evilang_lib::interpreter::runtime_values::PrimitiveValue;
use evilang_lib::parser::parse;
use evilang_lib::types::number::NumberT;
use evilang_lib::types::string::StringT;

use crate::cli::CliArguments;

pub mod cli;

type TestRes = ();

fn print_program(input: &str) -> TestRes {
	match parse(input.into()) {
		Ok(parsed) => {
			println!("Input: {}\nParsed:\n{:#?}", input, parsed);
		}
		Err(error_type) => {
			panic!("{}", error_type)
		}
	}
	return;
}

fn ensure_program(input: &str, expected: StatementList) -> TestRes {
	match parse(input.into()) {
		Ok(parsed) => {
			println!("Input: {}\nParsed:\n{:#?}", input, parsed);
			assert_eq!(parsed.deref(), &expected, "Mismatched parsed AST and expected AST");
		}
		Err(error_type) => {
			panic!("{}", error_type)
		}
	}
	return;
}

fn main() -> Result<(), Box<dyn Error>> {
	let args = CliArguments::parse();
	if args.src_debug {
		dbg!(std::mem::size_of::<PrimitiveValue>());
		dbg!(std::mem::size_of::<NumberT>());
		dbg!(std::mem::size_of::<StringT>());
		dbg!(std::mem::size_of::<GcPtrToFunction>());
		dbg!(std::mem::size_of::<Function>());
		dbg!(std::mem::size_of::<NativeFunction>());
		dbg!(std::mem::size_of::<Closure>());
		dbg!(std::mem::size_of::<FunctionDeclaration>());
		dbg!(std::mem::size_of::<GcPtrToObject>());
		dbg!(std::mem::size_of::<Option<GcPtrToObject>>());
		dbg!(std::mem::size_of::<RuntimeObject>());
	}
	let Some(file) = args.file else {
		return Ok(());
	};
	let env = Environment::execute_file(file, DefaultResolver::new_box())?;
	dbg!(&env.global_scope.borrow().res_stack);
	return Ok(());
}
