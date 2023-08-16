use evilang_lib::interpreter::environment::Environment;
use evilang_lib::interpreter::environment::resolver::DefaultResolver;
use evilang_lib::interpreter::runtime_values::PrimitiveValue;

use crate::common::{TestData, TestRes};

mod common;

#[test]
fn namespaces() -> TestRes {
	TestData::new(r#"
namespace Math {
	let INT32_MAX = 2147483647;
	fn square(n){
		return n * n;
	}
}
push_res_stack(Math.square(2));
push_res_stack(Math.square(3));
push_res_stack(Math.INT32_MAX);
"#.to_string())
		.expect_stack(vec![
			PrimitiveValue::integer(4),
			PrimitiveValue::integer(9),
			PrimitiveValue::integer(2147483647),
		])
		.check()
}

#[test]
fn imports_namespaces() -> TestRes {
	let file = env!("CARGO_MANIFEST_DIR").to_string() + "/resources/tests/import_test/main.evil";
	let mut env = Environment::execute_file(file, DefaultResolver::new_box()).unwrap();
	let prim_true = PrimitiveValue::Boolean(true);
	for x in env.global_scope.borrow().res_stack.iter() {
		assert_eq!(x, &prim_true, "Expected all result stack values to be true");
	}
}
