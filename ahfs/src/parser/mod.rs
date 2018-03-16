//! This module contains utilities for parsing various kinds of AHF
//! specification strings.
//!
//! # Scope
//!
//! It should be noted successfully parsed strings are only guaranteed to be
//! syntactically correct. It is up to the user of parsed strings to verify
//! that they are also logically correct.

mod document;
mod error;
mod grammar;
mod lexeme;
mod triple;

pub use self::document::Document;
pub use self::error::Error;
pub use self::lexeme::Lexeme;
pub use self::triple::Triple;
