use arspec_parser::Span;
use super::Value;

/// An arbitrary attribute, associated with some other specification element.
#[derive(Debug)]
pub struct Attribute<'a> {
    /// Attribute name.
    pub name: Span<'a>,

    /// Arbitrary attribute value.
    pub value: Value<'a>,
}