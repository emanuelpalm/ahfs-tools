use parser::{Error, Name, Result, Token};

/// A utility for reading well-defined [`Token`s][tok] sequences from an array.
///
/// [tok]: ../lexer/struct.Token.html
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Matcher<'a> {
    tokens: Box<[Token<'a>]>,
    offset: usize,
}

impl<'a> Matcher<'a> {
    /// Creates new `State` instance from given `tokens` pointer.
    #[inline]
    pub fn new<T>(tokens: T) -> Self
        where T: Into<Box<[Token<'a>]>>,
    {
        Matcher { tokens: tokens.into(), offset: 0 }
    }

    /// Whether or not all internal [`Token`s][lex] have been consumed.
    ///
    /// [lex]: ../lexer/struct.Token.html
    #[inline]
    pub fn at_end(&self) -> bool {
        self.offset >= self.tokens.len()
    }

    pub fn all(&mut self, names: &'static [Name]) -> Result<&[Token<'a>]> {
        let mut offset = self.offset;
        for name in names {
            let token = match self.tokens.get(offset) {
                Some(token) => token.clone(),
                None => {
                    return Err(self.tokens.last()
                        .map(|token| Error::UnexpectedSourceEnd {
                            excerpt: token.span().end().into(),
                            expected: vec![*name].into(),
                        })
                        .unwrap_or(Error::NoSource));
                }
            };
            if name != token.name() {
                return Err(Error::UnexpectedToken {
                    name: *token.name(),
                    excerpt: token.span().into(),
                    expected: vec![*name].into(),
                });
            }
            offset += 1;
        }
        let matches = unsafe { self.tokens.get_unchecked(self.offset..offset) };
        self.offset = offset;

        Ok(matches)
    }

    /// Returns next [Token][lex] only if its [`Name`][nam] matches one out of
    /// given `alternatives`.
    ///
    /// [lex]: ../lexer/struct.Token.html
    /// [nam]: ../name/enum.Name.html
    pub fn any(&mut self, alternatives: &'static [Name]) -> Result<Token<'a>> {
        let token = match self.tokens.get(self.offset) {
            Some(token) => token.clone(),
            None => {
                return Err(self.tokens.last()
                    .map(|token| Error::UnexpectedSourceEnd {
                        excerpt: token.span().end().into(),
                        expected: alternatives.into(),
                    })
                    .unwrap_or(Error::NoSource));
            }
        };
        if !alternatives.contains(token.name()) {
            return Err(Error::UnexpectedToken {
                name: *token.name(),
                excerpt: token.span().into(),
                expected: alternatives.into(),
            });
        }
        self.offset += 1;
        Ok(token)
    }

    pub fn one(&mut self, name: Name) -> Result<Token<'a>> {
        match self.tokens.get(self.offset) {
            Some(token) if name == *token.name() => {
                self.offset += 1;
                Ok(token.clone())
            }
            Some(token) => Err(Error::UnexpectedToken {
                name: *token.name(),
                excerpt: token.span().into(),
                expected: vec![name].into(),
            }),
            _ => Err(self.tokens.last()
                .map(|token| Error::UnexpectedSourceEnd {
                    excerpt: token.span().end().into(),
                    expected: vec![name].into(),
                })
                .unwrap_or(Error::NoSource)),
        }
    }

    pub fn one_optional(&mut self, name: Name) -> Option<Token<'a>> {
        let token = self.tokens.get(self.offset);
        match token {
            Some(token) if name == *token.name() => {
                self.offset += 1;
                Some(token.clone())
            }
            _ => None
        }
    }
}