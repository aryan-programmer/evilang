use evilang_lib::interpreter::runtime_value::PrimitiveValue;

use crate::common::{ensure_res_stack_matches, TestRes};

mod common;

#[test]
fn block_basic_test() -> TestRes {
	ensure_res_stack_matches(r#"
{
	let x = 10;
	let y = 20;
	push_res_stack(x*y+30);
}
"#, vec![PrimitiveValue::Integer(230)])
}

#[test]
fn variable_shadowing() -> TestRes {
	ensure_res_stack_matches(r#"
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
	ensure_res_stack_matches(r#"
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
	ensure_res_stack_matches(r#"
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
	ensure_res_stack_matches(r#"
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
