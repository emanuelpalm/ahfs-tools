//! Arrowhead specification compilation utilities.
//!
//! This module contains tools useful for compiling specification source texts
//! into useful data structures.

pub mod lexer;
pub mod parser;
pub mod source;

use self::source::{Result, Source};

/// A compiler pass.
pub trait Compile<'a, I, O> {
    /// Takes a [`source`](source/struct.Source.html) with a tree of type `I`
    /// and tried to transform it into another with type `O`.
    fn compile(source: &'a Source<'a, I>) -> Result<'a, Source<'a, O>>;
}