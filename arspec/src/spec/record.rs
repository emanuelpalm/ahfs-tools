use arspec_parser::Span;
use crate::spec::TypeRef;

#[derive(Debug)]
pub struct Record<'a> {
    pub name: Span<'a>,
    pub entries: Vec<RecordEntry<'a>>,
    pub comment: Option<Span<'a>>,
}

impl<'a> Record<'a> {
    #[inline]
    pub fn new(name: Span<'a>, comment: Option<Span<'a>>) -> Self {
        Record {
            name,
            entries: Vec::new(),
            comment,
        }
    }
}

#[derive(Debug)]
pub struct RecordEntry<'a> {
    pub name: Span<'a>,
    pub type_ref: TypeRef<'a>,
    pub comment: Option<Span<'a>>,
}