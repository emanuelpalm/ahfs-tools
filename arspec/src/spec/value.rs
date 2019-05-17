use arspec_parser::Span;

/// Value specification.
#[derive(Debug)]
pub enum Value<'a> {
    /// The absence of a meaningful value.
    Null,

    /// A boolean.
    Boolean(Span<'a>),

    /// An integer.
    Integer(Span<'a>),

    /// A floating-point number.
    Float(Span<'a>),

    /// A UTF-8 string.
    String(Span<'a>),

    /// A list of values.
    List(Box<[Value<'a>]>),

    /// A map of values, which is really an ordered list of name/value pairs.
    Map(Box<[(Span<'a>, Value<'a>)]>),
}
