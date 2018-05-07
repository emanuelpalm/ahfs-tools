use super::{Error, Name, Result, Token};

/// A utility for reading well-defined [`Token`s][lex] sequences from an array.
///
/// # Operation
///
/// To advance an internal read offset, _rules_ are applied to the `State`. If
/// a rule application is successful, the internal offset is updated to reflect
/// the number of [`Token`s][tok] consumed by the applied rule. If, on the
/// other hand, a rule application fails, the internal offset is unchanged.
///
/// Rules are lambdas operating on a given [`RuleState`][rul].
///
/// [tok]: ../lexer/struct.Token.html
/// [rul]: struct.RuleState.html
pub struct State<'a, 'b: 'a>(RuleState<'a, 'b>);

impl<'a, 'b: 'a> State<'a, 'b> {
    /// Creates new `State` instance from given `tokens` pointer.
    #[inline]
    pub fn new(tokens: &'a [Token<'b>]) -> Self {
        State(RuleState { tokens, offset: 0 })
    }

    /// Whether or not all internal [`Token`s][lex] have been consumed.
    ///
    /// [lex]: ../lexer/struct.Token.html
    #[inline]
    pub fn at_end(&self) -> bool {
        self.0.offset >= self.0.tokens.len()
    }

    /// Applies given rule.
    pub fn apply<R, T>(&mut self, rule: R) -> Result<T>
        where R: FnOnce(&mut RuleState<'a, 'b>) -> Result<T>
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
pub struct RuleState<'a, 'b: 'a> {
    tokens: &'a [Token<'b>],
    offset: usize,
}

impl<'a, 'b: 'a> RuleState<'a, 'b> {
    /// Returns next [Token][lex] only if it has one out of given `names`.
    ///
    /// [lex]: ../lexer/struct.Token.html
    pub fn next_if(&mut self, names: &'static [Name]) -> Result<Token<'b>> {
        let token = match self.tokens.get(self.offset) {
            Some(token) => token.clone(),
            None => {
                return Err(self.tokens.last()
                    .map(|token| Error::UnexpectedSourceEnd {
                        excerpt: token.region().end().into(),
                        expected: names,
                    })
                    .unwrap_or(Error::NoSource));
            }
        };
        if !names.contains(token.name()) {
            return Err(Error::UnexpectedToken {
                name: *token.name(),
                excerpt: token.region().into(),
                expected: names,
            });
        }
        self.offset += 1;
        Ok(token)
    }
}