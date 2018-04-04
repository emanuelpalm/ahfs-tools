use super::{Error, Lexeme, LexemeKind, Result};

pub struct Source<'a, K: 'a = LexemeKind>(State<'a, K>);

impl<'a, K: Clone> Source<'a, K> {
    #[inline]
    pub fn new<L, S>(lexemes: L, source: S) -> Self
        where L: Into<&'a [Lexeme<K>]>,
              S: Into<&'a str>,
    {
        Source(State {
            lexemes: lexemes.into(),
            offset: 0,
            source: source.into(),
        })
    }

    #[inline]
    pub fn lexemes(&self) -> &[Lexeme<K>] {
        &self.0.lexemes
    }

    #[inline]
    pub fn source(&self) -> &'a str {
        self.0.source
    }

    #[inline]
    pub fn at_end(&self) -> bool {
        self.0.offset >= self.0.lexemes.len()
    }

    pub fn apply<R, T>(&mut self, rule: R) -> Result<'a, T, K>
        where R: FnOnce(&mut State<'a, K>) -> Result<'a, T, K>
    {
        let offset = self.0.offset;
        match rule(&mut self.0) {
            Ok(result) => Ok(result),
            Err(error) => {
                self.0.offset = offset;
                Err(error)
            }
        }
    }

    pub fn ignore<R>(&mut self, rule: R) -> bool
        where R: FnOnce(&mut State<'a, K>) -> bool
    {
        rule(&mut self.0)
    }
}

pub struct State<'a, K: 'a> {
    lexemes: &'a [Lexeme<K>],
    offset: usize,
    source: &'a str,
}

impl<'a, K> State<'a, K> {
    #[inline]
    pub fn lexemes(&self) -> &[Lexeme<K>] {
        &self.lexemes
    }

    #[inline]
    pub fn source(&self) -> &'a str {
        self.source
    }

    #[inline]
    pub fn at_end(&self) -> bool {
        self.offset >= self.lexemes.len()
    }

    #[inline]
    pub fn peek(&self) -> Option<&Lexeme<K>> {
        self.lexemes.get(self.offset)
    }

    #[inline]
    pub fn skip(&mut self) {
        self.offset += 1;
    }
}

impl<'a, K: Clone + PartialEq> State<'a, K> {
    pub fn next_if(&mut self, kinds: &'static [K]) -> Result<'a, Lexeme<K>, K> {
        let lexeme = match self.peek() {
            Some(lexeme) => lexeme.clone(),
            None => return Err(Error::UnexpectedEnd {
                expected: kinds,
                source: self.source,
            }),
        };
        if !kinds.contains(lexeme.kind()) {
            return Err(Error::UnexpectedLexeme {
                expected: kinds,
                lexeme,
                source: self.source,
            });
        }
        self.offset += 1;
        Ok(lexeme)
    }

    pub fn is_next(&self, kinds: &'static [K]) -> bool {
        if let Some(lexeme) = self.peek() {
            return kinds.contains(lexeme.kind());
        }
        false
    }

    pub fn skip_until(&mut self, kinds: &'static [K]) {
        loop {
            match self.peek() {
                Some(ref lexeme) if !kinds.contains(lexeme.kind()) => {},
                _ => { return; },
            }
            self.skip();
        }
    }
}