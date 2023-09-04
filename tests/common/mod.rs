// Not really dead code
#![allow(dead_code)]

use std::ops::Deref;

use evilang_lib::ast::expression::Expression;
use evilang_lib::ast::expression::Expression::{AssignmentExpression, Identifier};
use evilang_lib::ast::operator::Operator::Assignment;
use evilang_lib::ast::statement::{Statement, StatementList};
use evilang_lib::ast::statement::Statement::ExpressionStatement;
use evilang_lib::errors::ErrorT;
use evilang_lib::interpreter::environment::Environment;
use evilang_lib::interpreter::environment::resolver::DefaultResolver;
use evilang_lib::interpreter::runtime_values::PrimitiveValue;
use evilang_lib::parser::parse;
use evilang_lib::types::string::CowStringT;

pub type TestRes = ();

pub struct TestData {
	input: String,
	expected: Option<StatementList>,
	statement_results: Option<Vec<PrimitiveValue>>,
	stack: Option<Vec<PrimitiveValue>>,
	parsed: Option<StatementList>,
}

impl TestData {
	pub fn new(input: String) -> TestData {
		Self::new_full(input, None, None, None)
	}

	pub fn new_full(input: String, expected: Option<StatementList>, statement_results: Option<Vec<PrimitiveValue>>, stack: Option<Vec<PrimitiveValue>>) -> Self {
		Self { input, expected, statement_results, stack, parsed: None }
	}

	pub fn expect_statements(mut self, expected: StatementList) -> Self {
		if let Some(results) = &self.statement_results {
			assert_eq!(expected.len(), results.len(), "Expected lengths of expected Statements list and expected results list to match");
		}
		self.expected = Some(expected);
		return self;
	}

	pub fn expect_results(mut self, results: Vec<PrimitiveValue>) -> Self {
		if let Some(expected) = &self.expected {
			assert_eq!(expected.len(), results.len(), "Expected lengths of expected Statements list and expected results list to match");
		}
		self.statement_results = Some(results);
		return self;
	}

	pub fn expect_statements_and_results(mut self, expected: StatementList, results: Vec<PrimitiveValue>) -> Self {
		assert_eq!(expected.len(), results.len(), "Expected lengths of expected Statements list and expected results list to match");
		self.expected = Some(expected);
		self.statement_results = Some(results);
		return self;
	}

	pub fn expect_stack(mut self, stack: Vec<PrimitiveValue>) -> Self {
		self.stack = Some(stack);
		return self;
	}

	pub fn parse(&mut self) -> StatementList {
		if let Some(parsed) = &self.parsed {
			return parsed.clone();
		} else {
			match parse(self.input.clone()) {
				Ok(parsed) => {
					// println!("{:?}", parsed);
					self.parsed = Some(parsed);
					let Some(parsed_) = &self.parsed else { panic!(); };
					return parsed_.clone();
				}
				Err(error_type) => {
					panic!("{}", error_type)
				}
			}
		}
	}

	pub fn check_parsing(&mut self) -> TestRes {
		let parsed = self.parse();
		let Some(expected) = &self.expected else {
			return ();
		};
		assert_eq!(&parsed, expected, "Mismatched parsed AST and expected AST");
		return ();
	}

	pub fn exec_and_check_statement_results(&mut self, env: &mut Environment) -> TestRes {
		let parsed = self.parse();
		let Some(results) = &self.statement_results else {
			env.setup_and_eval_statements(&parsed).unwrap();
			return ();
		};
		env.setup_scope(&parsed).unwrap();
		for (stmt, expected_val) in parsed.iter().zip(results.iter()) {
			if let ExpressionStatement(expr) = stmt {
				let value = env.eval(expr).unwrap();
				let borrow = value.borrow();
				let got_val = borrow.deref();
				assert_eq!(got_val, expected_val, "Expected values to match");
			} else {
				env.eval_statement(stmt).unwrap();
				assert_eq!(&PrimitiveValue::Null, expected_val, "Expected expected value to be null for not expression statements");
			}
		}
	}

	pub fn check_stack_results_no_exec(&mut self, env: &mut Environment) -> TestRes {
		let Some(stack) = &self.stack else {
			return ();
		};
		assert_eq!(&env.global_scope.borrow().res_stack, stack, "Expected result stack values to match");
	}

	pub fn check_with_env(&mut self, env: &mut Environment) -> TestRes {
		self.check_parsing();
		self.exec_and_check_statement_results(env);
		self.check_stack_results_no_exec(env);
		return ();
	}

	pub fn check(&mut self) -> TestRes {
		let mut env = Environment::new().unwrap();
		self.check_parsing();
		self.exec_and_check_statement_results(&mut env);
		self.check_stack_results_no_exec(&mut env);
		return ();
	}
}

pub fn ensure_program(input: &str, expected: StatementList) -> TestRes {
	return TestData::new(input.to_string()).expect_statements(expected).check_parsing();
}

pub fn ensure_parsing_fails(input: &str, typ: Option<ErrorT>) -> TestRes {
	match parse(input.to_string()) {
		Ok(parsed) => {
			// println!("{:?}", parsed);
			panic!("Program {} expected to fail parsed as {:#?}", input, parsed);
		}
		Err(error_type) => {
			if let Some(t) = typ {
				assert_eq!(t, error_type.typ, "Expected error types to match");
			}
		}
	}
	return;
}

pub fn ensure_execution_fails(input: String, typ: Option<ErrorT>) -> TestRes {
	let mut env = Environment::new().unwrap();
	match env.eval_program_string(input.clone()) {
		Ok(exec_res) => {
			panic!("Program {} expected to fail resulted in {:#?}", input, exec_res);
		}
		Err(error_type) => {
			if let Some(t) = typ {
				assert_eq!(t, error_type.typ, "Expected error types to match");
			}
		}
	}
	return;
}

pub fn ensure_program_statement_results(
	input: &str,
	expected: StatementList,
	results: Vec<PrimitiveValue>,
) -> TestRes {
	return TestData::new(input.to_string()).expect_statements_and_results(expected, results).check();
}

pub fn ensure_res_stack_matches(input: &str, results: Vec<PrimitiveValue>) -> TestRes {
	return TestData::new(input.to_string()).expect_stack(results).check();
}

pub fn ensure_res_stack_matches_with_env(
	input: &str,
	results: Vec<PrimitiveValue>,
	env: &mut Environment,
) -> TestRes {
	env.eval_program_string(input.into()).unwrap();
	assert_eq!(env.global_scope.borrow().res_stack, results, "Expected result values to match");
	return;
}

pub fn test_expression_and_assignment(input: &str, expr: Expression) -> TestRes {
	ensure_program(input, vec![expr.clone().consume_as_statement()]);
	let new_input = "y = ".to_string() + input;
	ensure_program(new_input.as_str(), vec![ExpressionStatement(
		AssignmentExpression {
			operator: Assignment,
			left: Identifier("y".to_string()).into(),
			right: expr.into(),
		},
	)]);
}

pub fn identifier_stmt(iden: &str) -> Statement {
	return Identifier(iden.into()).consume_as_statement();
}

pub fn push_res_stack_stmt(val: Expression) -> Statement {
	return ExpressionStatement(Expression::function_call(
		Identifier("push_res_stack".to_string()).into(),
		vec![val],
	));
}

pub fn run_asserts_in_file(file: CowStringT) {
	let file = env!("CARGO_MANIFEST_DIR").to_string() + file.as_ref();
	let env = Environment::execute_file(file, DefaultResolver::new_box()).unwrap();
	let prim_true = PrimitiveValue::Boolean(true);
	let global_scope_borrow = env.global_scope.borrow();
	let res_stack = &global_scope_borrow.res_stack;
	let trues = vec![prim_true; res_stack.len()];
	assert_eq!(res_stack, &trues, "Expected all result stack values to be true");
}
