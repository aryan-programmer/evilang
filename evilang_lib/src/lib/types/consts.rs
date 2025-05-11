// use const_str::concat;

pub use const_str::concat as concat_str;

pub const SUPER: &str = "super";
pub const OBJECT: &str = "Object";
pub const VECTOR: &str = "Vector";
pub const INSTANCE_OF_: &str = "Instance of ";
pub const CONSTRUCTOR: &str = "constructor";
pub const HIDDEN_PREFIX: &str = "__HIDDEN";
pub const CURRENT_FILE: &str = concat_str!(HIDDEN_PREFIX, "__CURRENT_FILE__");
