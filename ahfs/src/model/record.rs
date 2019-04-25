use crate::model::TypeRef;
use crate::source::Span;

#[derive(Debug)]
pub struct Record<'a> {
    name: Span<'a>,
    entries: Vec<RecordEntry<'a>>,
    comment: Option<Span<'a>>,
}

#[derive(Debug)]
pub struct RecordEntry<'a> {
    name: Span<'a>,
    type_ref: TypeRef<'a>,
    comment: Option<Span<'a>>,
}