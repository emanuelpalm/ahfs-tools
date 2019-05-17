use arspec_parser::Span;
use crate::spec::TypeRef;

/// A record type definition.
#[derive(Debug)]
pub struct Record<'a> {
    /// Name of record type.
    pub name: Span<'a>,

    /// Field definitions.
    pub entries: Vec<RecordEntry<'a>>,

    /// Any documentation comment.
    pub comment: Option<Span<'a>>,
}

impl<'a> Record<'a> {
    /// Create new record type definition.
    #[inline]
    pub fn new(name: Span<'a>, comment: Option<Span<'a>>) -> Self {
        Record {
            name,
            entries: Vec::new(),
            comment,
        }
    }
}

/// A record type definition field.
#[derive(Debug)]
pub struct RecordEntry<'a> {
    /// Field name.
    pub name: Span<'a>,

    /// Field type reference.
    pub type_ref: TypeRef<'a>,

    /// Any documentation comment.
    pub comment: Option<Span<'a>>,
}