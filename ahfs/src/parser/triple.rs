use super::Lexeme;

pub struct Triple<'a> {
    subject: Lexeme<'a>,
    predicate: Lexeme<'a>,
    object: Lexeme<'a>,
}

impl<'a> Triple<'a> {
    #[inline]
    pub fn new<L>(subject: L, predicate: L, object: L) -> Self
        where L: Into<Lexeme<'a>>
    {
        Triple {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
        }
    }

    #[inline]
    pub fn subject(&self) -> &Lexeme<'a> {
        &self.subject
    }

    #[inline]
    pub fn predicate(&self) -> &Lexeme<'a> {
        &self.predicate
    }

    #[inline]
    pub fn object(&self) -> &Lexeme<'a> {
        &self.object
    }
}