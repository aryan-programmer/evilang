use evilang_lib::interpreter::runtime_values::PrimitiveValue;

use crate::common::{run_asserts_in_file, TestData, TestRes};

mod common;

#[test]
fn vector() -> TestRes {
	run_asserts_in_file("/resources/tests/vector_test/main.evil".into());
}
