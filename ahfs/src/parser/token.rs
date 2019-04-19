use parser::Name;
use source::{Range, Span};

/// Identifies a typed [`Span`][spa] of some [`Source`][src].
///
/// [spa]: ../../source/struct.Span.html
/// [src]: ../../source/struct.Source.html
#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Token<'a> {
    name: Name,
    span: Span<'a>,
}

impl<'a> Token<'a> {
    /// Creates new `Token` from given `name` and `region`.
    #[inline]
    pub fn new(name: Name, region: Span<'a>) -> Self {
        Token { name, span: region }
    }

    /// `Token` name.
    #[inline]
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// `Token` region.
    #[inline]
    pub fn span(&self) -> &Span<'a> {
        &self.span
    }

    /// `Token` region.
    #[inline]
    pub fn into_span(self) -> Span<'a> {
        self.span
    }
}
