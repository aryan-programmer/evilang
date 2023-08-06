#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum UnrollingReason {
	EncounteredBreak(i64),
	EncounteredContinue(i64),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum StatementExecution {
	NormalFlow,
	Unrolling(UnrollingReason),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum StatementMetaGeneration {
	NormalGeneration,
}
