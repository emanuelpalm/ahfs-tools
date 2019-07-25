use crate::{Error, Token};
use std::fmt;

/// A utility for reading well-defined [`Token`][tok] sequences from an array.
///
/// [tok]: struct.Token.html
#[derive(Debug)]
pub struct Matcher<'a, TokenKind> {
    tokens: Box<[Token<'a, TokenKind>]>,
    offset: usize,
}

impl<'a, K: Copy + Eq + fmt::Debug> Matcher<'a, K> {
    /// Creates new `Matcher` instance from given `tokens`.
    #[inline]
    pub fn new<T>(tokens: T) -> Self
        where T: Into<Box<[Token<'a, K>]>>,
    {
        Matcher { tokens: tokens.into(), offset: 0 }
    }

    /// Whether or not all internal [`Token`s][tok] have been consumed.
    ///
    /// [tok]: struct.Token.html
    #[inline]
    pub fn at_end(&self) -> bool {
        self.offset >= self.tokens.len()
    }

    /// Returns `kinds.len()` [`Tokens`][tok] from the current offset, only if
    /// those [`Tokens`][tok] have kinds matching those in `kinds`.
    ///
    /// [tok]: struct.Token.html
    pub fn all(&mut self, kinds: &'static [K]) -> Result<&[Token<'a, K>], Error<K>> {
        let mut offset = self.offset;
        for kind in kinds {
            let token = match self.tokens.get(offset) {
                Some(token) => token.clone(),
                None => {
                    return Err(Error::unexpected_source_end(self.tokens.last(), vec![*kind]));
                }
            };
            if kind != &token.class {
                return Err(Error::unexpected_token(&token, vec![*kind]));
            }
            offset += 1;
        }
        let matches = unsafe { self.tokens.get_unchecked(self.offset..offset) };
        self.offset = offset;

        Ok(matches)
    }

    /// Returns next [Token][tok] only if its kind matches one out of given
    /// `alternatives`.
    ///
    /// [tok]: struct.Token.html
    pub fn any(&mut self, alternatives: &'static [K]) -> Result<Token<'a, K>, Error<K>> {
        let token = match self.tokens.get(self.offset) {
            Some(token) => token,
            None => {
                return Err(Error::unexpected_source_end(self.tokens.last(), alternatives));
            }
        };
        if !alternatives.contains(&token.class) {
            return Err(Error::unexpected_token(token, alternatives));
        }
        self.offset += 1;
        Ok(token.clone())
    }

    /// Returns next [Token][tok] only if its kind matches `kind`.
    ///
    /// [tok]: struct.Token.html
    pub fn one(&mut self, kind: K) -> Result<Token<'a, K>, Error<K>> {
        match self.tokens.get(self.offset) {
            Some(token) if kind == token.class => {
                self.offset += 1;
                Ok(token.clone())
            }
            Some(token) => Err(Error::unexpected_token(token, vec![kind])),
            _ => Err(Error::unexpected_source_end(self.tokens.last(), vec![kind])),
        }
    }

    /// Returns next [Token][tok] only if its kind matches `kind`. If the
    /// operation fails, `None` is returned instead of an error.
    ///
    /// [tok]: struct.Token.html
    pub fn one_optional(&mut self, kind: K) -> Option<Token<'a, K>> {
        let token = self.tokens.get(self.offset);
        match token {
            Some(token) if kind == token.class => {
                self.offset += 1;
                Some(token.clone())
            }
            _ => None
        }
    }
}