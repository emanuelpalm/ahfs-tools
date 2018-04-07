use super::{Lexeme, LexemeKind};

/// A parser triple.
///
/// Contains lexemes for a `subject`, a `predicate`, an `object`, and a
/// `description`.
///
/// The triple is only sure to be syntactically correct. No guarantees are
/// given about it expressing anything of relevance.
#[derive(Debug)]
pub struct Triple<'a> {
    subject: Lexeme<'a, ()>,
    predicate: Lexeme<'a, ()>,
    object: Lexeme<'a, ()>,
    description: Lexeme<'a, ()>,
}

impl<'a> Triple<'a> {
    #[inline]
    pub fn new<L>(subject: L, predicate: L, object: L, end: Lexeme<'a>) -> Self
        where L: Into<Lexeme<'a, ()>>,
    {
        Triple {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
            description: match *end.kind() {
                LexemeKind::Description => end.repackage(()),
                _ => Lexeme::new((), ""),
            },
        }
    }

    #[inline]
    pub fn subject(&self) -> &Lexeme<'a, ()> {
        &self.subject
    }

    #[inline]
    pub fn predicate(&self) -> &Lexeme<'a, ()> {
        &self.predicate
    }

    #[inline]
    pub fn object(&self) -> &Lexeme<'a, ()> {
        &self.object
    }

    #[inline]
    pub fn description(&self) -> Option<&Lexeme<'a, ()>> {
        if self.description.as_str().len() == 0 {
            return None;
        }
        Some(&self.description)
    }
}

impl<'a> Eq for Triple<'a> {}

impl<'a> PartialEq for Triple<'a> {
    fn eq<'b>(&self, other: &Triple<'b>) -> bool {
        return lexemes_eq(&self.subject, &other.subject)
            && lexemes_eq(&self.predicate, &other.predicate)
            && lexemes_eq(&self.object, &other.object)
            && lexemes_eq(&self.description, &other.description);

        #[inline]
        fn lexemes_eq<'a, 'b>(a: &Lexeme<'a, ()>, b: &Lexeme<'b, ()>) -> bool {
            a.as_str() == b.as_str()
        }
    }
}