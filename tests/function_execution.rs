use evilang_lib::interpreter::runtime_values::PrimitiveValue::Integer;

use crate::common::{ensure_res_stack_matches, TestRes};

mod common;

#[test]
fn square_function() -> TestRes {
	ensure_res_stack_matches(r#"
fn square(n){
	return n * n;
}
push_res_stack(square(2));
push_res_stack(square(3));
"#, vec![
		Integer(4),
		Integer(9),
	]);
}

#[test]
fn local_variable() -> TestRes {
	ensure_res_stack_matches(r#"
fn calc(x, y){
	let z = 30;
	return x * y + z;
}
push_res_stack(calc(2, 5));
push_res_stack(calc(7, 3));
"#, vec![
		Integer(40),
		Integer(51),
	]);
}

#[test]
fn recursion() -> TestRes {
	ensure_res_stack_matches(r#"
fn factorial(n) {
	if (n == 0){
		return 1;
	}
	return n * factorial(n - 1);
}
push_res_stack(factorial(20));
"#, vec![
		Integer(2432902008176640000),
	]);
}

#[test]
fn closure_outer_scope_access() -> TestRes {
	ensure_res_stack_matches(r#"
let value = 100;
fn calc(x, y){
	let z = x + y;
	fn inner(foo){
		return foo + z + value;
	}
	return inner;
}
let func = calc(10, 20);
push_res_stack(func(30));
push_res_stack(func(35));

value = -100;
push_res_stack(func(30));
"#, vec![
		Integer(160),
		Integer(165),
		Integer(-40),
	]);
}

#[test]
fn complex_closure() -> TestRes {
	ensure_res_stack_matches(r#"
fn useState(default) {
	let value = default;
	fn returner(name) {
		if(name == "get") {
			return get;
		} else if(name == "set") {
			return set;
		}
	}
	fn get() {
		return value;
	}
	fn set(nval) {
		value = nval;
	}
	return returner;
}
let state = useState(10);
let getX = state("get");
let setX = state("set");

push_res_stack(getX());
setX(getX() + 32);
push_res_stack(getX());
setX(-13);
push_res_stack(getX());
"#, vec![
		Integer(10),
		Integer(42),
		Integer(-13),
	]);
}

#[test]
fn complex_closure_with_hoisting() -> TestRes {
	ensure_res_stack_matches(r#"
fn useState(default) {
	let value = default;
	fn returner(name) {
		if(name == "get") {
			return get;
		} else if(name == "set") {
			return set;
		}
		fn get() {
			return value;
		}
		fn set(nval) {
			value = nval;
		}
	}
	return returner;
}
let state = useState(10);
let getX = state("get");
let setX = state("set");

push_res_stack(getX());
setX(getX() + 32);
push_res_stack(getX());
setX(-13);
push_res_stack(getX());
"#, vec![
		Integer(10),
		Integer(42),
		Integer(-13),
	]);
}

#[test]
fn callbacks() -> TestRes {
	ensure_res_stack_matches(r#"
fn eventTrigger(v, callback) {
	fn trigger(x, y) {
		callback(v + (x + y) * 2);
	}
	return trigger;
}
let trigger = eventTrigger(10, fn pusher(v){push_res_stack(v);});
trigger(3, 7);
trigger(7, 9);
"#, vec![
		Integer(30),
		Integer(42),
	]);
}

#[test]
fn iife() -> TestRes {
	ensure_res_stack_matches(r#"
push_res_stack(fn square(n){
	return n * n;
}(10));
(fn pusher(v){push_res_stack(v);}(fn square(n){
	return n * n;
}(-12)));
"#, vec![
		Integer(100),
		Integer(144),
	]);
}

#[test]
fn save_function_to_variable() -> TestRes {
	ensure_res_stack_matches(r#"
let square = fn square_fn(n){
	return n * n;
};
fn double_fn(n){
	return n + n;
}
let double = double_fn;
push_res_stack(square(2));
push_res_stack(square_fn(3));
push_res_stack(double(4));
push_res_stack(double_fn(5));
"#, vec![
		Integer(4),
		Integer(9),
		Integer(8),
		Integer(10),
	]);
}
