use arspec_parser::Span;
use super::{Attribute, TypeRef};

/// An enumerator type definition.
#[derive(Debug)]
pub struct Primitive<'a> {
    /// Name of enum type.
    pub generic_parameters: Vec<Span<'a>>,

    /// Type definition.
    pub definition: TypeRef<'a>,

    /// Any attributes.
    pub attributes: Vec<Attribute<'a>>,
}

impl<'a> Primitive<'a> {
    /// Create new enum type definition.
    #[inline]
    pub fn new(type_ref: TypeRef<'a>, attributes: Vec<Attribute<'a>>) -> Self {
        Primitive {
            generic_parameters: Vec::new(),
            definition: type_ref,
            attributes,
        }
    }
}

impl<'a> AsRef<str> for Primitive<'a> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.definition.name.as_str()
    }
}