include!("util/macros.rs");

pub mod meta;
pub mod project;
pub mod source;

mod parser;
mod triple;

pub use parser::parse;
pub use triple::Triple;

use parser::{Name, Token};