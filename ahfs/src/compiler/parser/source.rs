use super::{Error, Lexeme, LexemeKind, Result};

/// A utility for reading well-defined [`Lexeme`s][lex] sequences from an array.
///
/// # Operation
///
/// To advance an internal read offset, _rules_ are applied to the `State`. If
/// a rule application is successful, the internal offset is updated to reflect
/// the number of [`Lexeme`s][lex] consumed by the applied rule. If, on the
/// other hand, a rule application fails, the internal offset is unchanged.
///
/// Rules are lambdas operating on a given [`TentativeState`][ten].
///
/// [lex]: ../lexer/struct.Lexeme.html
/// [ten]: struct.TentativeState.html
pub struct State<'a>(TentativeState<'a>);

impl<'a> State<'a> {
    #[inline]
    pub fn new<L, S>(lexemes: L, source: S) -> Self
        where L: Into<&'a [Lexeme]>,
              S: Into<&'a str>,
    {
        State(TentativeState {
            lexemes: lexemes.into(),
            offset: 0,
            source: source.into(),
        })
    }

    /// Whether or not all internal [`Lexeme`s][lex] have been consumed.
    ///
    /// [lex]: ../lexer/struct.Lexeme.html
    #[inline]
    pub fn at_end(&self) -> bool {
        self.0.offset >= self.0.lexemes.len()
    }

    /// Applies given rule.
    pub fn apply<R, T>(&mut self, rule: R) -> Result<'a, T>
        where R: FnOnce(&mut TentativeState<'a>) -> Result<'a, T>
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

/// A tentative state, used while attempting to fulfill rules.
pub struct TentativeState<'a> {
    lexemes: &'a [Lexeme],
    offset: usize,
    source: &'a str,
}

impl<'a> TentativeState<'a> {
    /// Returns next [Lexeme][lex] only if it has one out of given `kinds`.
    ///
    /// [lex]: ../lexer/struct.Lexeme.html
    pub fn next_if(&mut self, kinds: &'a [LexemeKind]) -> Result<'a, Lexeme> {
        let lexeme = match self.lexemes.get(self.offset) {
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
}