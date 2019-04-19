use parser::Value;
use source::Span;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Property<'a> {
    pub name: Span<'a>,
    pub value: Value<'a>,
    pub comment: Option<Span<'a>>,
}