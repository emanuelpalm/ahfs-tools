use arspec_parser::Span;
use crate::spec::Value;

#[derive(Debug)]
pub struct Property<'a> {
    pub name: Span<'a>,
    pub value: Value<'a>,
    pub comment: Option<Span<'a>>,
}