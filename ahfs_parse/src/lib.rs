//! # Parsing Utilities
//!
//! This crate contains various utilities useful when creating hand-written
//! parsers. The most straightforward way to use the package is to implement
//! the [`Parser`](trait.Parser.html) trait.

mod error;
mod excerpt;
mod lines;
mod matcher;
mod range;
mod scanner;
mod source;
mod span;
mod token;

pub use self::error::{Error, Result};
pub use self::excerpt::Excerpt;
pub use self::lines::{Line, Lines};
pub use self::matcher::Matcher;
pub use self::range::Range;
pub use self::scanner::Scanner;
pub use self::source::Source;
pub use self::span::Span;
pub use self::token::Token;

use std::fmt;

/// Some text parser.
pub trait Parser<'a> {
    /// Whatever type is produced by a successful parser execution.
    type Output: 'a;

    /// The type used to enumerate tokens identified in parsed source strings.
    type TokenKind: Copy + Eq + fmt::Debug + fmt::Display;

    /// Attempts to parse referenced [`source`](struct.Source.html) text.
    #[inline]
    fn parse(source: &'a Source) -> Result<Self::Output, Self::TokenKind> {
        let scanner = Scanner::new(source);
        let tokens = Self::scan_(scanner);
        let matcher = Matcher::new(tokens);
        Self::match_(matcher)
    }

    /// Scans for [`tokens`][tok] using given [`scanner`][sca].
    ///
    /// [tok]: struct.Token.html
    /// [sca]: struct.Scanner.html
    fn scan_(scanner: Scanner<'a>) -> Vec<Token<'a, Self::TokenKind>>;

    /// Attempts to find valid token patterns using given
    /// [`matcher`](struct.Matcher.html).
    fn match_(matcher: Matcher<'a, Self::TokenKind>) -> Result<Self::Output, Self::TokenKind>;
}
