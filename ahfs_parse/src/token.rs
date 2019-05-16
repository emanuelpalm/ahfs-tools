use crate::Span;

/// Identifies a typed [`Span`][spa] of some [`Source`][src].
///
/// [spa]: struct.Span.html
/// [src]: struct.Source.html
#[derive(Clone, Debug)]
pub struct Token<'a, Kind> {
    /// Concrete token type.
    pub kind: Kind,

    /// Location of token in some source text.
    pub span: Span<'a>,
}
