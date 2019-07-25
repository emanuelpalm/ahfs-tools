use arspec_parser::Span;

/// An enumerator type definition.
#[derive(Debug)]
pub struct Enum<'a> {
    /// Name of enum type.
    pub name: Span<'a>,

    /// Variant names.
    pub variants: Vec<EnumVariant<'a>>,

    /// Any documentation comment.
    pub comment: Option<Span<'a>>,
}

impl<'a> Enum<'a> {
    /// Create new enum type definition.
    #[inline]
    pub fn new(name: Span<'a>, comment: Option<Span<'a>>) -> Self {
        Enum {
            name,
            variants: Vec::new(),
            comment,
        }
    }
}

/// An enumerator variant definition.
#[derive(Debug)]
pub struct EnumVariant<'a> {
    /// Name of enum variant.
    pub name: Span<'a>,

    /// Any documentation comment.
    pub comment: Option<Span<'a>>,
}
