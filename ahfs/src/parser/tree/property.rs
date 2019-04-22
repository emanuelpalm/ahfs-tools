use crate::parser::Value;
use crate::source::Span;

#[derive(Debug)]
pub struct Property<'a> {
    pub name: Span<'a>,
    pub value: Value<'a>,
    pub comment: Option<Span<'a>>,
}