use arspec_parser::Span;
use crate::spec::Value;

/// A named property [`Value`][val], with an optional documentation comment.
///
/// [val]: enum.Value.html
#[derive(Debug)]
pub struct Property<'a> {
    /// Property name.
    pub name: Span<'a>,

    /// Property values.
    pub value: Value<'a>,

    /// Any documentation comment.
    pub comment: Option<Span<'a>>,
}