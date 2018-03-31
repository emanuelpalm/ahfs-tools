use super::{Lexeme, LexemeKind, Source, Result};

pub fn parse<'a>(mut source: &mut Source<'a>) -> Result<'a, Vec<Triple>> {
    let mut triples = Vec::new();
    while !source.at_end() {
        let subject = subject(&mut source)?;
        let predicate = predicate(&mut source)?;
        let object = object(&mut source)?;
        triples.push(Triple {
            subject: subject.repackage(()),
            predicate: predicate.repackage(()),
            object,
        });
    }
    return Ok(triples);

    #[inline]
    fn subject<'a>(source: &mut Source<'a>) -> Result<'a, Lexeme> {
        source.apply(|state| state.next(&[LexemeKind::Word]))
    }

    #[inline]
    fn predicate<'a>(source: &mut Source<'a>) -> Result<'a, Lexeme> {
        source.apply(|state| {
            let word = state.next(&[LexemeKind::Word])?;
            state.next(&[LexemeKind::Colon])?; // TODO: Colon starts text?
            Ok(word)
        })
    }

    #[inline]
    fn object<'a>(source: &mut Source<'a>) -> Result<'a, Lexeme<()>> {
        // TODO: Support function and identifier objects---not just texts.
        source.apply(|state| {
            let text = state.join(&[
                LexemeKind::Newline,
                LexemeKind::Hash,
                LexemeKind::ParenthesisLeft,
                LexemeKind::ParenthesisRight,
                LexemeKind::Colon,
                LexemeKind::BracketLeft,
                LexemeKind::BracketRight,
                LexemeKind::BraceLeft,
                LexemeKind::BraceRight,
                LexemeKind::Word,
            ]);
            // TODO: Support escaped semicolons? Balanced braces?
            state.next(&[LexemeKind::Semicolon])?;
            Ok(text)
        })
    }
}

pub struct Triple {
    subject: Lexeme<()>,
    predicate: Lexeme<()>,
    object: Lexeme<()>,
}
