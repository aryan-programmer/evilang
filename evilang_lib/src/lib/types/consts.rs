// use const_str::concat;

pub use const_str::concat as concat_str;

pub const SUPER: &'static str = "super";
pub const OBJECT: &'static str = "Object";
pub const VECTOR: &'static str = "Vector";
pub const INSTANCE_OF_: &'static str = "Instance of ";
pub const CONSTRUCTOR: &'static str = "constructor";
pub const HIDDEN_PREFIX: &'static str = "__HIDDEN";
pub const CURRENT_FILE: &'static str = concat_str!(HIDDEN_PREFIX, "__CURRENT_FILE__");
