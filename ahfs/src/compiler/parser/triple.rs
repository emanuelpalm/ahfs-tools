use super::{Lexeme, LexemeKind};

/// A parser triple.
///
/// Contains lexemes for a `subject`, a `predicate`, an `object`, and a
/// `description`.
///
/// The triple is only sure to be syntactically correct. No guarantees are
/// given about it expressing anything of relevance.
#[derive(Debug)]
pub struct Triple {
    subject: Lexeme<()>,
    predicate: Lexeme<()>,
    object: Lexeme<()>,
    description: Lexeme<()>,
}

impl Triple {
    #[inline]
    pub fn new<L>(subject: L, predicate: L, object: L, end: Lexeme) -> Self
        where L: Into<Lexeme<()>>,
    {
        Triple {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
            description: match *end.kind() {
                LexemeKind::Description => end.repackage(()),
                _ => Lexeme::new((), 0, 0),
            },
        }
    }

    #[inline]
    pub fn subject(&self) -> &Lexeme<()> {
        &self.subject
    }

    #[inline]
    pub fn predicate(&self) -> &Lexeme<()> {
        &self.predicate
    }

    #[inline]
    pub fn object(&self) -> &Lexeme<()> {
        &self.object
    }

    #[inline]
    pub fn description(&self) -> Option<&Lexeme<()>> {
        if self.description.end() == 0 {
            return None;
        }
        Some(&self.description)
    }
}

impl Eq for Triple {}

impl PartialEq for Triple {
    fn eq(&self, other: &Triple) -> bool {
        return lexemes_eq(&self.subject, &other.subject)
            && lexemes_eq(&self.predicate, &other.predicate)
            && lexemes_eq(&self.object, &other.object)
            && lexemes_eq(&self.description, &other.description);

        #[inline]
        fn lexemes_eq(a: &Lexeme<()>, b: &Lexeme<()>) -> bool {
            a.start() == b.start() && a.end() == b.end()
        }
    }
}