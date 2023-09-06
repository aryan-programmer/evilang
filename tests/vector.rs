use crate::common::{run_asserts_in_file, TestRes};

mod common;

#[test]
fn vector() -> TestRes {
	run_asserts_in_file("/resources/tests/vector_test/main.evil".into());
}
