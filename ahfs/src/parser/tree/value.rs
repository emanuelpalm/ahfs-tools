use crate::source::Span;

#[derive(Debug)]
pub enum Value<'a> {
    Null,
    Boolean(Span<'a>),
    Integer(Span<'a>),
    Float(Span<'a>),
    String(Span<'a>),
    List(Box<[Value<'a>]>),
    Map(Box<[(Span<'a>, Value<'a>)]>),
}

impl<'a> Value<'a> {
    #[inline]
    pub fn parse_boolean(span: Span<'a>) -> Option<bool> {
        match span.as_str() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    }
}