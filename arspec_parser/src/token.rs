use crate::Span;

/// Identifies the type of a [`Span`][spa] referring to some [`Text`][txt].
///
/// [spa]: struct.Span.html
/// [txt]: struct.Text.html
#[derive(Clone, Debug)]
pub struct Token<'a, Kind> {
    /// Token enumeration.
    pub class: Kind,

    /// Location of token in some source text.
    pub span: Span<'a>,
}
