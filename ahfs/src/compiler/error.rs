use std::error;
use std::fmt;
use super::lexer::Lexeme;

#[derive(Debug)]
pub struct Error<'a> {
    kind: ErrorKind,
    message: Option<Box<str>>,
    lexeme: Lexeme<'a>,
}

impl<'a> Error<'a> {
    #[inline]
    pub fn new<M, L>(kind: ErrorKind, message: Option<M>, lexeme: L) -> Self
        where M: Into<Box<str>>,
              L: Into<Lexeme<'a>>
    {
        Error {
            kind,
            message: message.map(|m| m.into()),
            lexeme: lexeme.into()
        }
    }
}

impl<'a> error::Error for Error<'a> {
    fn description(&self) -> &str {
        if let Some(ref message) = self.message {
            return message;
        }
        self.kind.description()
    }
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self.message {
            Some(ref message) => message,
            None => self.kind.description(),
        };
        write!(f, "{}: {}", message, self.lexeme.as_str())
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
