use std::fmt;
use std::mem;

/// Classifies the contents of a [`Lexeme`][lexeme].
///
/// [lexeme]: struct.Lexeme.html
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum LexemeKind {
    /// A newline character `'\n'`.
    Newline = ('\n' as u32),

    /// A hash `#`.
    Hash = ('#' as u32),

    /// A start parenthesis `(`.
    ParenthesisLeft = ('(' as u32),

    /// An stop parenthesis `)`.
    ParenthesisRight = (')' as u32),

    /// A colon `:`.
    Colon = (':' as u32),

    /// A semicolon `;`.
    Semicolon = (';' as u32),

    /// A start bracket `[`.
    BracketLeft = ('[' as u32),

    /// A stop bracket `]`.
    BracketRight = (']' as u32),

    /// A start curly brace `{`.
    BraceLeft = ('{' as u32),

    /// A stop curly brace `}`.
    BraceRight = ('}' as u32),

    /// Any other kind of lexeme.
    Word,
}

impl LexemeKind {
    /// Casts special delimiter character into `Kind`.
    ///
    /// The given character `ch` must be one of `\n # ( ) : ; [ ] { }`.
    pub unsafe fn from_delimiter(ch: char) -> Self {
        debug_assert!(match ch {
            '\n' | '#' | '(' | ')' | ':' | ';' | '[' | ']' | '{' | '}' => true,
            _ => false,
        });
        mem::transmute(ch)
    }
}

impl fmt::Display for LexemeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Write;

        match *self {
            LexemeKind::Word => f.write_str("word"),
            kind @ _ => f.write_char(unsafe { mem::transmute(kind) })
        }
    }
}