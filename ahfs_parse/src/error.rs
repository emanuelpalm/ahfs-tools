use crate::{Excerpt, Token};
use std::fmt;

/// A parser error.
#[derive(Debug)]
pub struct Error<Kind: fmt::Debug> {
    pub cause: Option<Kind>,
    pub excerpt: Option<Excerpt>,
    pub expected: Vec<Kind>,
}

impl<Kind: Copy + fmt::Debug> Error<Kind> {
    /// A parsed [`Source`](../source/struct.Source.html) ended unexpectedly.
    #[inline]
    pub fn unexpected_source_end<E>(tokens: &[Token<Kind>], expected: E) -> Self
        where E: Into<Vec<Kind>>,
    {
        Error {
            cause: None,
            excerpt: tokens.last().map(|token| token.span.to_excerpt()),
            expected: expected.into(),
        }
    }

    /// An unexpected [`Token`](struct.Token.html) was read while parsing.
    #[inline]
    pub fn unexpected_token<E>(token: &Token<Kind>, expected: E) -> Self
        where E: Into<Vec<Kind>>,
    {
        Error {
            cause: Some(token.kind),
            excerpt: Some(token.span.to_excerpt()),
            expected: expected.into(),
        }
    }
}

impl<Kind: Copy + fmt::Debug + fmt::Display> fmt::Display for Error<Kind> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cause {
            None => write!(f, "Unexpected source end"),
            Some(name) => write!(f, "Unexpected `{}`", name),
        }?;
        match self.expected.as_ref() as &[Kind] {
            &[] => {},
            &[kind] => write!(f, ", expected `{}`", kind)?,
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
        write!(f, ".")?;
        if let Some(ref excerpt) = self.excerpt {
            write!(f, "\n{}", excerpt)?;
        }
        Ok(())
    }
}
