use std::fmt;

/// Classifies the contents of a [`Lexeme`][lexeme].
///
/// [lexeme]: struct.Lexeme.html
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LexemeKind {
    Semicolon,
    Description,
    BraceRight,
    Word,
}

impl fmt::Display for LexemeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LexemeKind::Semicolon => f.write_str(";"),
            LexemeKind::Description => f.write_str("{"),
            LexemeKind::BraceRight => f.write_str("}"),
            LexemeKind::Word => f.write_str("word"),
        }
    }
}