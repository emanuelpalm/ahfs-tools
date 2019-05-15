use crate::{Excerpt, Range, Token};
use std::fmt;
use std::result;

/// A parsing result.
pub type Result<T, TokenKind> = result::Result<T, Error<TokenKind>>;

/// A parser error.
#[derive(Debug)]
pub struct Error<Kind: fmt::Debug> {
    pub cause: Option<Kind>,
    pub excerpt: Excerpt,
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
            excerpt: tokens.last()
                .map_or_else(|| Excerpt::default(), |token| {
                    let start = token.span.range.end.saturating_sub(1);
                    let end = token.span.range.end;
                    let lines = token.span.lines();
                    Excerpt {
                        source: token.span.source.clone(),
                        line_number: lines.number,
                        range: Range { start, end },
                    }
                }),
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
            excerpt: Excerpt {
                source: token.span.source.clone(),
                line_number: token.span.lines().number,
                range: token.span.range,
            },
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
        write!(f, ".\n{}", self.excerpt)
    }
}
