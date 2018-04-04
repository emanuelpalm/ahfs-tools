mod source;
mod triple;

pub use self::triple::Triple;

use self::source::Source;
use std::result;
use super::Error;
use super::lexer::{Lexeme, LexemeKind};

const HASH: &'static [LexemeKind] = &[LexemeKind::Hash];
const NEWLINE: &'static [LexemeKind] = &[LexemeKind::Semicolon];
const WORD: &'static [LexemeKind] = &[LexemeKind::Word];

/// The result of a parsing attempt.
pub type Result<'a, T, K:'a = LexemeKind> = result::Result<T, Error<'a, K>>;

pub fn parse<'a>(lexemes: &'a [Lexeme], source: &'a str) -> Result<'a,
    Vec<Triple>> {
    let mut source = Source::new(lexemes, source);

    let mut triples = Vec::new();
    while !source.at_end() {
        if comment(&mut source) {
            continue;
        }
        triples.push(triple(&mut source)?);
    }
    return Ok(triples);

    #[inline]
    fn comment<'a>(source: &mut Source<'a>) -> bool {
        source.ignore(|state| {
            if state.is_next(HASH) {
                state.skip_until(NEWLINE);
                state.skip();
                return true;
            }
            false
        })
    }

    #[inline]
    fn triple<'a>(source: &mut Source<'a>) -> Result<'a, Triple> {
        source.apply(|state| {
            let subject = state.next_if(WORD)?;
            let predicate = state.next_if(WORD)?;
            let object = state.next_if(WORD)?;
            state.next_if(NEWLINE)?;
            Ok(Triple::new(subject, predicate, object))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexeme, LexemeKind, Triple};

    const SOURCE: &'static str = concat!(
            "A type System;\n",
            "B type Service;\n",
            "# Emojis 😜🤖💩!");

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
            Lexeme::new(LexemeKind::Semicolon, 29, 30),
            Lexeme::new(LexemeKind::Hash, 31, 32),
            Lexeme::new(LexemeKind::Word, 33, 39),
            Lexeme::new(LexemeKind::Word, 40, 47),
        ];
        assert_eq!(super::parse(&lexemes, SOURCE).unwrap(), vec![
            Triple::new(
                Lexeme::new((), 0, 1),
                Lexeme::new((), 2, 6),
                Lexeme::new((), 7, 13)),
            Triple::new(
                Lexeme::new((), 15, 16),
                Lexeme::new((), 17, 21),
                Lexeme::new((), 22, 29)),
        ]);
    }
}