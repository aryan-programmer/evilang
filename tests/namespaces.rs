use evilang_lib::interpreter::runtime_values::PrimitiveValue;

use crate::common::{run_asserts_in_file, TestData, TestRes};

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
push_res_stack(Math::square(2));
push_res_stack(Math::square(3));
push_res_stack(Math::INT32_MAX);
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
	run_asserts_in_file("/resources/tests/import_test/main.evil".into());
}
