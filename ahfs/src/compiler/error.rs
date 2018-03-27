use std::error;
use std::fmt;
use super::lexer::Lexeme;

#[derive(Debug)]
pub struct Error<'a> {
    kind: ErrorKind,
    lexeme: Lexeme<'a>,
}

impl<'a> Error<'a> {
    #[inline]
    pub fn new<L>(kind: ErrorKind, lexeme: Lexeme<'a>) -> Self
        where L: Into<Lexeme<'a>>
    {
        Error { kind, lexeme: lexeme.into() }
    }
}

impl<'a> error::Error for Error<'a> {
    fn description(&self) -> &str {
        self.kind.description()
    }
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.kind.description(), self.lexeme)
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Syntax,
}

impl ErrorKind {
    fn description(&self) -> &'static str {
        match *self {
            ErrorKind::Syntax => "Invalid syntax",
        }
    }
}
