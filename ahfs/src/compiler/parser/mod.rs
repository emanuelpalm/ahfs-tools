mod source;
mod triple;

pub use self::triple::Triple;

use self::source::State;
use std::result;
use super::Error;
use super::lexer::{Lexeme, LexemeKind};

const TRIPLE_END: &'static [LexemeKind] = &[
    LexemeKind::Semicolon,
    LexemeKind::Description
];
const WORD: &'static [LexemeKind] = &[
    LexemeKind::Word
];

/// The result of a parsing attempt.
pub type Result<'a, T, K: 'a = LexemeKind> = result::Result<T, Error<'a, K>>;

/// Parses given array of [`Lexeme`s][lex] into vector of [`Triple`s][tri].
///
/// # Panics
///
/// If any of the given [`Lexeme`s][lex] would have offsets out of the `source`
/// string bounds, the method panics. This is generally avoided by ensuring that
/// the lexemes were first extracted from the same `source` string.
///
/// [lex]: ../lexer/struct.Lexeme.html
/// [tri]: struct.Triple.html
pub fn parse<'a>(lx: &'a [Lexeme], source: &'a str) -> Result<'a, Vec<Triple>> {
    let mut source = State::new(lx, source);
    let mut triples = Vec::new();
    while !source.at_end() {
        triples.push(source.apply(|state| Ok(Triple::new(
            state.next_if(WORD)?,
            state.next_if(WORD)?,
            state.next_if(WORD)?,
            state.next_if(TRIPLE_END)?,
        )))?);
    }
    return Ok(triples);
}

#[cfg(test)]
mod tests {
    use super::{Lexeme, LexemeKind, Triple};

    const SOURCE: &'static str = concat!(
            "A type System;\n",
            "B type Service { Emojis 😜🤖💩! }");

    #[test]
    fn parse() {
        let lexemes = [
            Lexeme::new(LexemeKind::Word, 0, 1),
            Lexeme::new(LexemeKind::Word, 2, 6),
            Lexeme::new(LexemeKind::Word, 7, 13),
            Lexeme::new(LexemeKind::Semicolon, 13, 14),
            Lexeme::new(LexemeKind::Word, 15, 16),
            Lexeme::new(LexemeKind::Word, 17, 21),
            Lexeme::new(LexemeKind::Word, 22, 29),
            Lexeme::new(LexemeKind::Description, 30, 54),
        ];
        assert_eq!(super::parse(&lexemes, SOURCE).unwrap(), vec![
            Triple::new(
                Lexeme::new((), 0, 1),
                Lexeme::new((), 2, 6),
                Lexeme::new((), 7, 13),
                Lexeme::new(LexemeKind::Semicolon, 13, 14)),
            Triple::new(
                Lexeme::new((), 15, 16),
                Lexeme::new((), 17, 21),
                Lexeme::new((), 22, 29),
                Lexeme::new(LexemeKind::Description, 30, 54)),
        ]);
    }
}