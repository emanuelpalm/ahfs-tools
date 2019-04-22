use crate::parser::{Error, Name, Result, Token};

/// A utility for reading well-defined [`Token`s][tok] sequences from an array.
///
/// [tok]: ../lexer/struct.Token.html
#[derive(Debug)]
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

    /// Whether or not all internal [`Token`s][tok] have been consumed.
    ///
    /// [tok]: ../token/struct.Token.html
    #[inline]
    pub fn at_end(&self) -> bool {
        self.offset >= self.tokens.len()
    }

    /// Returns `names.len()` [`Token`s][tok] from the current offset, only if
    /// those [`Token`s][tok] have [`Name`s][nam] matching those in `names`.
    ///
    /// [tok]: ../token/struct.Token.html
    /// [nam]: ../name/enum.Name.html
    pub fn all(&mut self, names: &'static [Name]) -> Result<&[Token<'a>]> {
        let mut offset = self.offset;
        for name in names {
            let token = match self.tokens.get(offset) {
                Some(token) => token.clone(),
                None => {
                    return Err(Error::unexpected_source_end(&self.tokens, vec![*name]));
                }
            };
            if name != token.name() {
                return Err(Error::unexpected_token(&token, vec![*name]));
            }
            offset += 1;
        }
        let matches = unsafe { self.tokens.get_unchecked(self.offset..offset) };
        self.offset = offset;

        Ok(matches)
    }

    /// Returns next [Token][tok] only if its [`Name`][nam] matches one out of
    /// given `alternatives`.
    ///
    /// [tok]: ../token/struct.Token.html
    /// [nam]: ../name/enum.Name.html
    pub fn any(&mut self, alternatives: &'static [Name]) -> Result<Token<'a>> {
        let token = match self.tokens.get(self.offset) {
            Some(token) => token,
            None => {
                return Err(Error::unexpected_source_end(&self.tokens, alternatives));
            }
        };
        if !alternatives.contains(token.name()) {
            return Err(Error::unexpected_token(token, alternatives));
        }
        self.offset += 1;
        Ok(token.clone())
    }

    /// Returns next [Token][tok] only if its [`Name`][nam] matches `name`.
    ///
    /// [tok]: ../token/struct.Token.html
    /// [nam]: ../name/enum.Name.html
    pub fn one(&mut self, name: Name) -> Result<Token<'a>> {
        match self.tokens.get(self.offset) {
            Some(token) if name == *token.name() => {
                self.offset += 1;
                Ok(token.clone())
            }
            Some(token) => Err(Error::unexpected_token(token, vec![name])),
            _ => Err(Error::unexpected_source_end(&self.tokens, vec![name])),
        }
    }

    /// Returns next [Token][tok] only if its [`Name`][nam] matches `name`. If
    /// the operation fails, `None` is returned instead of an error.
    ///
    /// [tok]: ../token/struct.Token.html
    /// [nam]: ../name/enum.Name.html
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