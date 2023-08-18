use std::borrow::Cow;

pub type StringT = String;
pub type CowStringT<'a> = Cow<'a, str>;
