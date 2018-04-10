mod state;
mod triple;

pub use self::triple::Triple;

use self::state::State;
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
pub fn parse<'a>(lx: &'a [Lexeme], src: &'a str) -> Result<'a, Vec<Triple<'a>>>
{
    let mut source = State::new(lx, src);
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
        let lexeme = |kind, range| Lexeme::new(kind, &SOURCE[range]);
        let lexemes = [
            lexeme(LexemeKind::Word, 0..1),
            lexeme(LexemeKind::Word, 2..6),
            lexeme(LexemeKind::Word, 7..13),
            lexeme(LexemeKind::Semicolon, 13..14),
            lexeme(LexemeKind::Word, 15..16),
            lexeme(LexemeKind::Word, 17..21),
            lexeme(LexemeKind::Word, 22..29),
            lexeme(LexemeKind::Description, 30..54),
        ];
        let lexeme0 = |kind, range| Lexeme::new(kind, &SOURCE[range]);
        assert_eq!(super::parse(&lexemes, SOURCE).unwrap(), vec![
            Triple::new(
                lexeme0((), 0..1),
                lexeme0((), 2..6),
                lexeme0((), 7..13),
                lexeme(LexemeKind::Semicolon, 13..14)),
            Triple::new(
                lexeme0((), 15..16),
                lexeme0((), 17..21),
                lexeme0((), 22..29),
                lexeme(LexemeKind::Description, 30..54)),
        ]);
    }
}