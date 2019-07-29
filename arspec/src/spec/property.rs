use arspec_parser::Span;
use crate::spec::{Attribute, Value};

/// A named property [`Value`][val], with an optional documentation comment.
///
/// [val]: enum.Value.html
#[derive(Debug)]
pub struct Property<'a> {
    /// Property name.
    pub name: Span<'a>,

    /// Property values.
    pub value: Value<'a>,

    /// Any attributes.
    pub attributes: Vec<Attribute<'a>>,
}