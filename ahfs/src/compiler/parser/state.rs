use super::{Error, Lexeme, LexemeKind, Result, Source};

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
    /// Creates new `State` object from given `source` pointer.
    #[inline]
    pub fn new(source: &'a Source<'a, Box<[Lexeme<'a>]>>) -> Self {
        State(TentativeState {
            source,
            offset: 0,
        })
    }

    /// Whether or not all internal [`Lexeme`s][lex] have been consumed.
    ///
    /// [lex]: ../lexer/struct.Lexeme.html
    #[inline]
    pub fn at_end(&self) -> bool {
        self.0.offset >= self.0.source.tree().len()
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
    source: &'a Source<'a, Box<[Lexeme<'a>]>>,
    offset: usize,
}

impl<'a> TentativeState<'a> {
    /// Returns next [Lexeme][lex] only if it has one out of given `kinds`.
    ///
    /// [lex]: ../lexer/struct.Lexeme.html
    pub fn next_if(&mut self, kinds: &'a [LexemeKind]) -> Result<'a, Lexeme<'a>> {
        let lexeme = match self.source.tree().get(self.offset) {
            Some(lexeme) => lexeme.clone(),
            None => {
                if let Some(region) = self.source.end_region() {
                    return Err(
                        Error::new("P001", "Unexpected source end.", region)
                    );
                }
                return Err(Error::new("P000", "No known sources.", None));
            }
        };
        if !kinds.contains(lexeme.kind()) {
            let mut text = String::new();
            text.push_str(&format!("Unexpected `{}`", lexeme.kind()));
            match kinds.len() {
                0 => text.push_str("."),
                1 => text.push_str(&format!(", expected `{}`.", kinds[0])),
                _ => {
                    text.push_str(", expected one of ");
                    let (last, rest) = kinds.split_last().unwrap();
                    for e in rest {
                        text.push_str(&format!("`{}`, ", e));
                    }
                    text.push_str(&format!(" or `{}`.", last));
                }
            }
            return Err(
                Error::new("P002", text, lexeme.region().clone())
            );
        }
        self.offset += 1;
        Ok(lexeme)
    }
}