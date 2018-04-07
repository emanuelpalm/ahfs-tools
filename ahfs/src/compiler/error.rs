use std::error;
use std::fmt;
use super::lexer::Lexeme;

/// A compiler error.
///
/// Represents a condition preventing a compiler from fulfilling a given
/// request.
#[derive(Debug)]
pub enum Error<'a, K: 'a> {
    /// End of source was unexpectedly encountered while parsing.
    ///
    /// Any of the provided kinds would have been accepted if provided.
    UnexpectedEnd {
        /// Expected lexeme kinds.
        expected: &'a [K],

        /// Compiler source string causing error.
        source: &'a str,
    },

    /// An unexpected lexeme kind was encountered while parsing.
    ///
    /// Any of the provided kinds would have been accepted if provided instead
    /// of the offending lexeme.
    UnexpectedLexeme {
        /// Expected lexeme kinds.
        expected: &'a [K],

        /// Offending lexeme.
        lexeme: Lexeme<'a, K>,

        /// Compiler source string causing error.
        source: &'a str,
    },
}

impl<'a, K> Error<'a, K> {
    /// Returns machine-readable error identifier.
    pub fn code(&self) -> &'static str {
        match *self {
            Error::UnexpectedEnd { .. } => meta::UNEXPECTED_END.code,
            Error::UnexpectedLexeme { .. } => meta::UNEXPECTED_LEXEME.code,
        }
    }

    /// Returns human-readable error description.
    pub fn text(&self) -> &'static str {
        match *self {
            Error::UnexpectedEnd { .. } => meta::UNEXPECTED_END.text,
            Error::UnexpectedLexeme { .. } => meta::UNEXPECTED_LEXEME.text,
        }
    }
}

impl<'a, K: fmt::Debug + fmt::Display> error::Error for Error<'a, K> {
    fn description(&self) -> &str {
        self.text()
    }
}

impl<'a, K: fmt::Display> fmt::Display for Error<'a, K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match *self {
            Error::UnexpectedEnd { expected, source } => {
                fmt_meta(f, &meta::UNEXPECTED_END)?;
                fmt_expected(f, expected)?;
                Lexeme::new((), &source[source.len()..]).fmt(f, source)
            }
            Error::UnexpectedLexeme { expected, ref lexeme, source } => {
                fmt_meta(f, &meta::UNEXPECTED_LEXEME)?;
                fmt_expected(f, expected)?;
                lexeme.fmt(f, source)
            }
        };

        fn fmt_meta(f: &mut fmt::Formatter, meta: &meta::Error) -> fmt::Result {
            write!(f, "Error {}: {}", meta.code, meta.text)
        }

        fn fmt_expected<'a, K: fmt::Display>(
            f: &mut fmt::Formatter,
            expected: &'a [K],
        ) -> fmt::Result
        {
            match expected.len() {
                0 => Ok(()),
                1 => writeln!(f, ", expected `{}`.", expected[0]),
                _ => {
                    write!(f, ", expected one of ")?;
                    let (last, rest) = expected.split_last().unwrap();
                    for e in rest {
                        write!(f, "`{}`, ", e)?;
                    }
                    writeln!(f, " or `{}`.", last)
                }
            }
        }
    }
}

/// Various error meta data.
mod meta {
    /// Error meta data.
    pub struct Error {
        /// A machine-readable error identifier.
        ///
        /// Uniquely identifies error kind. Must consist of exactly four ASCII
        /// characters, making it require 4 bytes of memory.
        pub code: &'static str,

        /// A human-readable error description.
        ///
        /// Should be short, to the point, start with a capitalized letter, and
        /// end _without_ a period.
        pub text: &'static str,
    }

    pub const UNEXPECTED_END: Error = Error {
        code: "P001",
        text: "Unexpected source end",
    };
    pub const UNEXPECTED_LEXEME: Error = Error {
        code: "P002",
        text: "Unexpected lexeme",
    };
}
