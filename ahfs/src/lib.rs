include!("util/macros.rs");

pub mod meta;
pub mod parser;
pub mod project;
pub mod source;

mod error;
mod triple;

pub use self::error::Error;
pub use self::triple::Triple;