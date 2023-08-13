pub trait ConsumeOrCloneOf {
	type Target;

	fn consume_or_clone(self) -> Self::Target;
}
