//! This crate contains various utilities useful when creating hand-written
//! parsers. The most straightforward way to use the package is to implement
//! the [`Parser`](trait.Parser.html) trait.

mod error;
mod excerpt;
mod lines;
mod matcher;
mod range;
mod scanner;
mod span;
mod text;
mod token;

pub use self::error::Error;
pub use self::excerpt::Excerpt;
pub use self::lines::{Line, Lines};
pub use self::matcher::Matcher;
pub use self::range::Range;
pub use self::scanner::Scanner;
pub use self::span::Span;
pub use self::text::Text;
pub use self::token::Token;

use std::fmt;

/// Some text parser.
pub trait Parser<'a> {
    /// The type used to enumerate tokens identified in parsed source strings.
    type Class: Copy + Eq + fmt::Debug + fmt::Display;

    /// Whatever type is produced by a successful [`parse()`][par] invocation.
    ///
    /// [par]: trait.Parser.html#method.parse
    type Output: 'a;

    /// Attempts to parse referenced [`text`](struct.Text.html) text.
    #[inline]
    fn parse(text: &'a Text) -> Result<Self::Output, Error<Self::Class>> {
        let scanner = Scanner::new(text);
        let tokens = Self::analyze(scanner);
        let matcher = Matcher::new(tokens);
        Self::combine(matcher)
    }

    /// Produces vector of [`tokens`][tok] from [`Text`][txt] referenced by
    /// given [`scanner`][sca].
    ///
    /// [sca]: struct.Scanner.html
    /// [txt]: struct.Text.html
    /// [tok]: struct.Token.html
    fn analyze(scanner: Scanner<'a>) -> Vec<Token<'a, Self::Class>>;

    /// Produces `Output` instance or [`Error][err] from [`Tokens`][tok] owned
    /// by given [`matcher`](mtc).
    ///
    /// [err]: struct.Error.html
    /// [mtc]: struct.Matcher.html
    /// [tok]: struct.Token.html
    fn combine(matcher: Matcher<'a, Self::Class>) -> Result<Self::Output, Error<Self::Class>>;
}
