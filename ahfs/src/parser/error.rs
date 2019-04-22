use crate::parser::{Name, Token};
use crate::source::Excerpt;
use std::fmt;

/// A parser error.
#[derive(Debug)]
pub struct Error {
    cause: Option<Name>,
    excerpt: Excerpt,
    expected: Vec<Name>,
}

impl Error {
    /// A parsed [`Source`](../source/struct.Source.html) ended unexpectedly.
    #[inline]
    pub fn unexpected_source_end<E>(tokens: &[Token], expected: E) -> Self
        where E: Into<Vec<Name>>,
    {
        Error {
            cause: None,
            excerpt: tokens.last()
                .map_or_else(|| Excerpt::default(), |token| {
                    token.span().end().into()
                }),
            expected: expected.into(),
        }
    }

    /// An unexpected [`Token`](struct.Token.html) was read while parsing.
    #[inline]
    pub fn unexpected_token<E>(token: &Token, expected: E) -> Self
        where E: Into<Vec<Name>>,
    {
        Error {
            cause: Some(*token.name()),
            excerpt: token.span().into(),
            expected: expected.into(),
        }
    }

    #[inline]
    pub fn cause(&self) -> Option<Name> {
        self.cause
    }

    #[inline]
    pub fn excerpt(&self) -> &Excerpt {
        &self.excerpt
    }

    #[inline]
    pub fn expected(&self) -> &[Name] {
        &self.expected
    }

    #[inline]
    pub fn expected_mut(&mut self) -> &mut Vec<Name> {
        &mut self.expected
    }
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cause {
            None => write!(f, "Unexpected source end"),
            Some(name) => write!(f, "Unexpected `{}`", name),
        }?;
        match self.expected.as_ref() as &[Name] {
            &[] => {},
            &[name] => write!(f, ", expected `{}`", name)?,
            _ => {
                write!(f, ", expected one of ")?;

                let (first, rest) = self.expected.split_first().unwrap();
                write!(f, "`{}`", first)?;

                let (last, rest) = rest.split_last().unwrap();
                for item in rest {
                    write!(f, ", `{}`", item)?;
                }
                write!(f, " or `{}`", last)?
            }
        };
        write!(f, ".\n{}", self.excerpt)
    }
}

impl crate::Error for Error {
    #[inline]
    fn code(&self) -> &'static str {
        match self.cause {
            None => "P001",
            Some(_) => "P002",
        }
    }
}