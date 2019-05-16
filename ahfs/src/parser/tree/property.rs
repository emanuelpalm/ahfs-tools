use ahfs_parse::Span;
use crate::parser::Value;

#[derive(Debug)]
pub struct Property<'a> {
    pub name: Span<'a>,
    pub value: Value<'a>,
    pub comment: Option<Span<'a>>,
}