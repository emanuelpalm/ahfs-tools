use std::fmt;

/// Classifies the contents of a [`Lexeme`][lexeme].
///
/// [lexeme]: struct.Lexeme.html
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LexemeKind {
    /// A hash `#`.
    Hash,

    /// A semicolon `;`.
    Semicolon,

    /// Any other kind of lexeme.
    Word,
}

impl fmt::Display for LexemeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LexemeKind::Hash => f.write_str("#"),
            LexemeKind::Semicolon => f.write_str(";"),
            LexemeKind::Word => f.write_str("word"),
        }
    }
}