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
	let program = r#"
class SuperClass {

}
SuperClass.x = 12;
class Point extends SuperClass {
}
Point.x = 13;
push_res_stack(SuperClass.x, Point.x);
"#;

	/*
let v2 = allocate_object(Object, "Child");
push_res_stack(Object);
push_res_stack(allocate_object());
push_res_stack(v2);
push_res_stack(allocate_object(v2));
*/

	// print_program(program);
	env.eval_program_string(program.to_string()).map_err(|err| {
		dbg!(err);
	}).unwrap();
	dbg!(&env.global_scope.borrow().res_stack);
}
