use std::error;
use std::fmt;
use super::lexer::Lexeme;

/// A compiler error.
///
/// Represents a condition preventing a compiler from fulfilling a given
/// request.
pub struct Error<'a, K: 'a> {
    kind: ErrorKind<'a, K>,
    lexeme: Lexeme<K>,
    source: &'a str,
}

impl<'a, K> Error<'a, K> {
    /// Creates new compiler error from given `kind`, `lexeme` and `source`.
    ///
    /// The `kind` identifies the category of the error, while the `lexeme`
    /// identifies the the cause of the error within its `source` string.
    #[inline]
    pub fn new(kind: ErrorKind<'a, K>, lexeme: Lexeme<K>, source: &'a str) ->
    Self {
        Error { kind, lexeme, source }
    }
}

impl<'a, K: fmt::Debug + fmt::Display> error::Error for Error<'a, K> {
    fn description(&self) -> &str {
        self.kind.description()
    }
}

impl<'a, K: fmt::Debug> fmt::Debug for Error<'a, K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {{ kind: {:?}, lexeme: {:?}, source: {:?} }}",
               self.kind, self.lexeme, self.source)
    }
}

impl<'a, K: fmt::Display> fmt::Display for Error<'a, K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.kind)?;
        self.lexeme.fmt_using(f, self.source)
    }
}

/// A compiler error category.
pub enum ErrorKind<'a, K: 'a> {
    /// An unexpected lexeme kind was encountered while parsing.
    ///
    /// Any of the provided kinds would have been accepted if provided instead
    /// of the offending lexeme.
    UnexpectedLexeme { expected: &'a [K] },
}

impl<'a, K> ErrorKind<'a, K> {
    /// Returns a pointer to a string describing the error kind.
    pub fn description(&self) -> &'static str {
        match *self {
            ErrorKind::UnexpectedLexeme { .. } => "Unexpected lexeme",
        }
    }
}

impl<'a, K: fmt::Debug> fmt::Debug for ErrorKind<'a, K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::UnexpectedLexeme { expected } => {
                write!(f, "UnexpectedLexeme {{ expected: {:?} }}", expected)
            },
        }
    }
}

impl<'a, K: fmt::Display> fmt::Display for ErrorKind<'a, K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::UnexpectedLexeme { expected } => match expected.len() {
                0 => f.write_str("Unexpected lexeme"),
                1 => write!(f, "Unexpected lexeme, expected `{}`", expected[0]),
                _ => {
                    write!(f, "Unexpected lexeme, expected one of ")?;
                    let (last, rest) = expected.split_last().unwrap();
                    for e in rest {
                        write!(f, "`{}`, ", e)?;
                    }
                    write!(f, " or `{}`", last)
                }
            },
        }
    }
}