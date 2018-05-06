include!("util/macros.rs");

pub mod meta;
pub mod parser;
pub mod project;
pub mod source;

mod error_code;
mod triple;

pub use self::error_code::ErrorCode;
pub use self::triple::Triple;