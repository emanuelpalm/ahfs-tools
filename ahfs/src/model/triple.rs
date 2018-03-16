use std::fmt;
use ::model::Predicate;

#[derive(Eq, PartialEq)]
pub struct Triple {
    subject: Box<str>,
    predicate: Predicate,
    object: Box<str>,
}

impl Triple {
    #[inline]
    pub fn new<S>(subject: S, predicate: Predicate, object: S) -> Self
        where S: Into<Box<str>>
    {
        Triple {
            subject: subject.into(),
            predicate,
            object: object.into()
        }
    }
}

impl fmt::Display for Triple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}: {};", self.subject, self.predicate, self.object)
    }
}