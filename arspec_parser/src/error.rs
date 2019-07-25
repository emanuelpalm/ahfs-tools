use crate::{Excerpt, Token};
use std::fmt;

/// A parser error.
///
/// Used to signal that an unexpected sequence of characters or the end of the
/// parsed [`Text`][txt] forced a [`Parser][par] to fail.
///
/// [txt]: struct.Text.html
/// [par]: trait.Parser.html
#[derive(Debug)]
pub struct Error<Kind: fmt::Debug> {
    /// `Kind` of token actually encountered.
    pub actual: Option<Kind>,

    /// Vector of `Kinds` expected to be encountered.
    pub expected: Vec<Kind>,

    /// The `Excerpt` of the original source [`Text`][txt] where the token
    /// causing this `Error` to be created is located.
    ///
    /// [txt]: struct.Text.html
    pub excerpt: Option<Excerpt>,
}

impl<Kind: Copy + fmt::Debug> Error<Kind> {
    /// Create new `Error` signifying that parsing of some [`Text`][txt] ended
    /// unexpectedly.
    ///
    /// [txt]: struct.Text.html
    #[inline]
    pub fn unexpected_source_end<E>(last_token: Option<&Token<Kind>>, expected: E) -> Self
        where E: Into<Vec<Kind>>,
    {
        Error {
            actual: None,
            expected: expected.into(),
            excerpt: last_token.map(|token| token.span.to_excerpt()),
        }
    }

    /// Create new `Error` signifying that an unexpected [`Token`][tok] was
    /// encountered while parsing.
    ///
    /// [tok]: struct.Token.html
    #[inline]
    pub fn unexpected_token<E>(actual: &Token<Kind>, expected: E) -> Self
        where E: Into<Vec<Kind>>,
    {
        Error {
            actual: Some(actual.class),
            expected: expected.into(),
            excerpt: Some(actual.span.to_excerpt()),
        }
    }
}

impl<Kind: Copy + fmt::Debug + fmt::Display> fmt::Display for Error<Kind> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.actual {
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
