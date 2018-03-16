use ::model::Triple;
use std::slice::Iter;

pub struct Model {
    triples: Vec<Triple>,
}

impl Model {
    #[inline]
    pub fn new() -> Self {
        Model { triples: Vec::new() }
    }

    #[inline]
    pub fn with_triples(triples: Vec<Triple>) -> Self {
        Model { triples }
    }

    #[inline]
    pub fn insert<'a, S>(&mut self, triple: Triple)
        where S: Into<Box<str>>
    {
        self.triples.push(triple)
    }

    #[inline]
    pub fn triples(&self) -> Iter<Triple> {
        self.triples.iter()
    }
}