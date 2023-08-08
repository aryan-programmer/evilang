use crate::interpreter::runtime_values::PrimitiveValue;

#[derive(Debug, Clone, PartialEq)]
pub enum UnrollingReason {
	EncounteredBreak(i64),
	EncounteredContinue(i64),
	ReturningValue(PrimitiveValue),
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementExecution {
	NormalFlow,
	Unrolling(UnrollingReason),
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementMetaGeneration {
	NormalGeneration,
}

#[macro_export]
macro_rules! handle_unrolling {
	($val: expr) => {
		if let StatementExecution::Unrolling(imm_exit) = $val {
			return Ok(StatementExecution::Unrolling(imm_exit));
		}
	}
}

#[macro_export]
macro_rules! handle_unrolling_in_loop {
	($val: expr) => {
		handle_unrolling_in_loop!($val => break: break; continue: continue;)
	};
	($val: expr => break: $break_stmt: stmt; continue: $continue_stmt: stmt;) => {
		if let StatementExecution::Unrolling(imm_exit) = $val {
			match imm_exit {
				UnrollingReason::EncounteredBreak(v) => {
					if v <= 1 {
						$break_stmt
					}
					return Ok(StatementExecution::Unrolling(UnrollingReason::EncounteredBreak(v-1)));
				},
				UnrollingReason::EncounteredContinue(v) => {
					if v <= 1 {
						$continue_stmt
					}
					return Ok(StatementExecution::Unrolling(UnrollingReason::EncounteredContinue(v-1)));
				},
				imm_e => {
					return Ok(StatementExecution::Unrolling(imm_e));
				},
			};
		}
	}
}

pub use handle_unrolling;
pub use handle_unrolling_in_loop;
