include!("util/macros.rs");

pub mod meta;
pub mod parser;
pub mod project;
pub mod source;

mod triple;

pub use self::triple::Triple;