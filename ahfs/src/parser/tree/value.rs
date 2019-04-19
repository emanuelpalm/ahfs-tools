use source::Span;

#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Value<'a> {
    Null,
    Boolean(Span<'a>),
    Integer(Span<'a>),
    Float(Span<'a>),
    String(Span<'a>),
    List(Box<[Value<'a>]>),
    Map(Box<[(Span<'a>, Value<'a>)]>),
}