mod source;
mod triple;

pub use self::source::Source;

use std::result;
use super::Error;
use super::lexer::{Lexeme, LexemeKind};

/// The result of a parsing attempt.
pub type Result<'a, T, K:'a = LexemeKind> = result::Result<T, Error<'a, K>>;
