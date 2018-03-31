use super::{Error, Lexeme, LexemeKind, Result};

pub struct Source<'a, K = LexemeKind>(State<'a, K>);

impl<'a, K: Clone> Source<'a, K> {
    #[inline]
    pub fn new<L, S>(lexemes: L, source: S) -> Self
        where L: Into<Box<[Lexeme<K>]>>,
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
}

pub struct State<'a, K> {
    lexemes: Box<[Lexeme<K>]>,
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
    pub fn peek(&mut self) -> Option<&Lexeme<K>> {
        self.lexemes.get(self.offset)
    }

    #[inline]
    pub fn skip(&mut self) {
        self.offset += 1;
    }
}

impl<'a, K: Clone + PartialEq> State<'a, K> {
    pub fn next(&mut self, kinds: &'static [K]) -> Result<'a, Lexeme<K>, K> {
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

    pub fn join(&mut self, kinds: &'static [K]) -> Lexeme<()> {
        let (start, mut end) = match self.peek().map(|lexeme| lexeme.clone()) {
            Some(lexeme) => (lexeme.start(), lexeme.end()),
            None => {
                let end = match self.lexemes.len() {
                    0 => 0,
                    _ => self.lexemes[self.offset - 1].end()
                };
                return Lexeme::new((), end, end);
            }
        };
        loop {
            match self.peek() {
                Some(lexeme) if kinds.contains(lexeme.kind()) => {
                    end = lexeme.end();
                },
                _ => { break; },
            }
            self.skip();
        }
        Lexeme::new((), start, end)
    }
}