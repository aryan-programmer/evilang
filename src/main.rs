#![allow(dead_code)]

use std::ops::Deref;

use evilang_lib::ast::statement::StatementList;
use evilang_lib::interpreter::environment::Environment;
use evilang_lib::parser::parse;

type TestRes = ();

fn print_program(input: &str) -> TestRes {
	match parse(input.to_string()) {
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
	match parse(input.to_string()) {
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

fn main() {
	let mut env = Environment::new();
	env.parse_and_run_program(r#"
	let a = "y";
	let b = "1";
	{
		let a = "z";
		b = "2";
		push_res_stack(a);
		push_res_stack(b);
	}
	push_res_stack(a);
	push_res_stack(b);
	push_res_stack(c);
"#.to_string()).unwrap();
	dbg!(&env.global_scope.borrow().res_stack);
}
