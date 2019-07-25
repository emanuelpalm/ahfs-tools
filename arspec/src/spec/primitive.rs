use arspec_parser::Span;
use super::TypeRef;

/// An enumerator type definition.
#[derive(Debug)]
pub struct Primitive<'a> {
    /// Name of enum type.
    pub generic_parameters: Vec<Span<'a>>,

    /// Type definition.
    pub definition: TypeRef<'a>,

    /// Any documentation comment.
    pub comment: Option<Span<'a>>,
}

impl<'a> Primitive<'a> {
    /// Create new enum type definition.
    #[inline]
    pub fn new(type_ref: TypeRef<'a>, comment: Option<Span<'a>>) -> Self {
        Primitive {
            generic_parameters: Vec::new(),
            definition: type_ref,
            comment,
        }
    }
}
