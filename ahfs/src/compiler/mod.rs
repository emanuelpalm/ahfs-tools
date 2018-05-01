//! Arrowhead specification compilation utilities.
//!
//! This module contains tools useful for compiling specification source texts
//! into useful data structures.

pub mod lexer;
pub mod parser;

mod tree;

pub use self::tree::Tree;

use super::source::{Error, Range, Region, Result, Source, Text};