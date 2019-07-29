use arspec_parser::Span;
use crate::spec::{Attribute, TypeRef};

/// A record type definition.
#[derive(Debug)]
pub struct Record<'a> {
    /// Name of record type.
    pub name: Span<'a>,

    /// Field definitions.
    pub entries: Vec<RecordEntry<'a>>,

    /// Any attributes.
    pub attributes: Vec<Attribute<'a>>,
}

impl<'a> Record<'a> {
    /// Create new record type definition.
    #[inline]
    pub fn new(name: Span<'a>, attributes: Vec<Attribute<'a>>) -> Self {
        Record {
            name,
            entries: Vec::new(),
            attributes,
        }
    }
}

impl<'a> AsRef<str> for Record<'a> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name.as_str()
    }
}

/// A record type definition field.
#[derive(Debug)]
pub struct RecordEntry<'a> {
    /// Field name.
    pub name: Span<'a>,

    /// Field type reference.
    pub type_ref: TypeRef<'a>,

    /// Any attributes.
    pub attributes: Vec<Attribute<'a>>,
}