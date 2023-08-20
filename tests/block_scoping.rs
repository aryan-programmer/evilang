use evilang_lib::errors::ErrorT;
use evilang_lib::interpreter::runtime_values::PrimitiveValue;
use evilang_lib::types::traits::Clone__SilentlyFail;

use crate::common::{ensure_execution_fails, ensure_res_stack_matches, TestData, TestRes};

mod common;

const BLOCK_OPTS: [(&str, &str); 6] = [
	("{", "}"),
	("do{", "}while(false);"),
	("while(true){", "break;}"),
	("for(;;){", "break;}"),
	("if(true){", "}"),
	("if(false){}else{", "}"),
];

fn block_scope_tests(input: &str, results: Vec<PrimitiveValue>) -> TestRes {
	let inp_str = input.to_string();
	for (left, right) in BLOCK_OPTS.iter() {
		TestData::new(inp_str.replace("{", left).replace("}", right))
			.expect_stack(results.iter().map(|v| v.clone__silently_fail()).collect())
			.check();
	}
	return ();
}

fn block_scope_failure_tests(input: &str, typ: Option<ErrorT>) -> TestRes {
	let inp_str = input.to_string();
	for (left, right) in BLOCK_OPTS.iter() {
		ensure_execution_fails(
			inp_str
				.replace("{", left)
				.replace("}", right),
			typ.clone(),
		);
	}
	return ();
}

#[test]
fn lost_variables() -> TestRes {
	ensure_res_stack_matches(r#"
let i = -100;
for(let i = 0; i < 1; i += 1)let i = -1;
push_res_stack(i);
"#, vec![PrimitiveValue::integer(-100)]);
	ensure_res_stack_matches(r#"
let i = -100;
let x = -200;
for({let i = 0;}; i < 1; {let x = 1; i += x;})let i = -1;
push_res_stack(i);
push_res_stack(x);
"#, vec![PrimitiveValue::integer(-100), PrimitiveValue::integer(-200)]);
	ensure_res_stack_matches(r#"
let x = -200;
for(let i = 0; i < 1; i += 1)let x = 10;
push_res_stack(x);
"#, vec![PrimitiveValue::integer(-200)]);
	ensure_res_stack_matches(r#"
let x = -200;
while(false)let x = 10;
push_res_stack(x);
"#, vec![PrimitiveValue::integer(-200)]);
	ensure_res_stack_matches(r#"
let x = -200;
if(true)let x = 10;
push_res_stack(x);
"#, vec![PrimitiveValue::integer(-200)]);
	ensure_res_stack_matches(r#"
let x = -200;
if(false);else let x = 10;
push_res_stack(x);
"#, vec![PrimitiveValue::integer(-200)]);
}

#[test]
fn block_basic_test() -> TestRes {
	block_scope_tests(r#"
{
	let x = 10;
	let y = 20;
	push_res_stack(x*y+30);
}
"#, vec![PrimitiveValue::integer(230)])
}

#[test]
fn variable_shadowing() -> TestRes {
	block_scope_tests(r#"
{
	{
		let x = "inner_x";
		push_res_stack(x);
	}
	push_res_stack(x);
}
"#, vec![
		PrimitiveValue::String("inner_x".into()),
		PrimitiveValue::Null,
	]);

	block_scope_tests(r#"
{
	let x = "outer_x";
	{
		let x = "inner_x";
		push_res_stack(x);
	}
	push_res_stack(x);
}
"#, vec![
		PrimitiveValue::String("inner_x".into()),
		PrimitiveValue::String("outer_x".into()),
	])
}

#[test]
fn outer_variable_access_no_inner_access() -> TestRes {
	block_scope_tests(r#"
{
	let x = "is_x";
	{
		let y = "is_y";
		push_res_stack(x + " " + y);
	}
	push_res_stack(x);
	push_res_stack(y);
}
"#, vec![
		PrimitiveValue::String("is_x is_y".into()),
		PrimitiveValue::String("is_x".into()),
		PrimitiveValue::Null,
	])
}

#[test]
fn update_outer_variables() -> TestRes {
	block_scope_tests(r#"
{
	let a = "y";
	{
		a = "z";
	}
	push_res_stack(a);
}
"#, vec![
		PrimitiveValue::String("z".into()),
	])
}

#[test]
fn full_test() -> TestRes {
	block_scope_tests(r#"
{
	let a = "y";
	let b = "1";
	push_res_stack(a);
	push_res_stack(b);
	push_res_stack(c);
	{
		let a = "z";
		b = "2";
		let c = "x";
		push_res_stack(a);
		push_res_stack(b);
		push_res_stack(c);
	}
	push_res_stack(a);
	push_res_stack(b);
	push_res_stack(c);
}
"#, vec![
		PrimitiveValue::String("y".into()),
		PrimitiveValue::String("1".into()),
		PrimitiveValue::Null,
		PrimitiveValue::String("z".into()),
		PrimitiveValue::String("2".into()),
		PrimitiveValue::String("x".into()),
		PrimitiveValue::String("y".into()),
		PrimitiveValue::String("2".into()),
		PrimitiveValue::Null,
	])
}

#[test]
fn no_redeclaration() -> TestRes {
	block_scope_failure_tests(r#"{let a, a;}"#, Some(ErrorT::CantRedeclareVariable("a".into())));
	block_scope_failure_tests(r#"{let b; let b;}"#, Some(ErrorT::CantRedeclareVariable("b".into())));
	block_scope_failure_tests(r#"{let c; let c = 1;}"#, Some(ErrorT::CantRedeclareVariable("c".into())));
}

#[test]
fn no_access_of_undeclared_variables() -> TestRes {
	block_scope_failure_tests(r#"{a; let a;}"#, Some(ErrorT::CantAccessHoistedVariable("a".into())));
	block_scope_failure_tests(r#"{let v = b; let b;}"#, Some(ErrorT::CantAccessHoistedVariable("b".into())));
}
