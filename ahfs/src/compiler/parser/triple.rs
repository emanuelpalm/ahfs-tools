use super::Lexeme;

#[derive(Debug)]
pub struct Triple {
    subject: Lexeme<()>,
    predicate: Lexeme<()>,
    object: Lexeme<()>,
}

impl Triple {
    pub fn new<L>(subject: L, predicate: L, object: L) -> Self
        where L: Into<Lexeme<()>>
    {
        Triple {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
        }
    }
}

impl Eq for Triple {}

impl PartialEq for Triple {
    fn eq(&self, other: &Triple) -> bool {
        return lexemes_eq(&self.subject, &other.subject)
            && lexemes_eq(&self.predicate, &other.predicate)
            && lexemes_eq(&self.object, &other.object);

        #[inline]
        fn lexemes_eq(a: &Lexeme<()>, b: &Lexeme<()>) -> bool {
            a.start() == b.start() && a.end() == b.end()
        }
    }
}