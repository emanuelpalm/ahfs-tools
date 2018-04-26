use super::{Error, Name, Result, Token, Tree};

/// A utility for reading well-defined [`Token`s][lex] sequences from an array.
///
/// # Operation
///
/// To advance an internal read offset, _rules_ are applied to the `State`. If
/// a rule application is successful, the internal offset is updated to reflect
/// the number of [`Token`s][tok] consumed by the applied rule. If, on the
/// other hand, a rule application fails, the internal offset is unchanged.
///
/// Rules are lambdas operating on a given [`TentativeState`][ten].
///
/// [tok]: ../lexer/struct.Token.html
/// [ten]: struct.TentativeState.html
pub struct State<'a: 'b, 'b>(TentativeState<'a, 'b>);

impl<'a: 'b, 'b> State<'a, 'b> {
    /// Creates new `State` object from given `source` pointer.
    #[inline]
    pub fn new(tree: &'b Tree<'a, [Token<'a>]>) -> Self {
        State(TentativeState { tree, offset: 0 })
    }

    /// Whether or not all internal [`Token`s][lex] have been consumed.
    ///
    /// [lex]: ../lexer/struct.Token.html
    #[inline]
    pub fn at_end(&self) -> bool {
        self.0.offset >= self.0.tree.root().len()
    }

    /// Applies given rule.
    pub fn apply<R, T>(&mut self, rule: R) -> Result<'a, T>
        where R: FnOnce(&mut TentativeState<'a, 'b>) -> Result<'a, T>
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
pub struct TentativeState<'a: 'b, 'b> {
    tree: &'b Tree<'a, [Token<'a>]>,
    offset: usize,
}

impl<'a: 'b, 'b> TentativeState<'a, 'b> {
    /// Returns next [Token][lex] only if it has one out of given `names`.
    ///
    /// [lex]: ../lexer/struct.Token.html
    pub fn next_if(&mut self, names: &'b [Name]) -> Result<'a, Token<'a>> {
        let token = match self.tree.root().get(self.offset) {
            Some(token) => token.clone(),
            None => {
                if let Some(region) = self.tree.source().end_region() {
                    return Err(
                        Error::new("P001", "Unexpected source end.", region)
                    );
                }
                return Err(Error::new("P000", "No known sources.", None));
            }
        };
        if !names.contains(token.name()) {
            let mut text = String::new();
            text.push_str(&format!("Unexpected `{}`", token.name()));
            match names.len() {
                0 => text.push_str("."),
                1 => text.push_str(&format!(", expected `{}`.", names[0])),
                _ => {
                    text.push_str(", expected one of ");
                    let (last, rest) = names.split_last().unwrap();
                    for e in rest {
                        text.push_str(&format!("`{}`, ", e));
                    }
                    text.push_str(&format!(" or `{}`.", last));
                }
            }
            return Err(
                Error::new("P002", text, token.region().clone())
            );
        }
        self.offset += 1;
        Ok(token)
    }
}