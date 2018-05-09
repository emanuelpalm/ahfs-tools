use super::Triple;

/// A [`Graph`][gra] `Query`.
///
/// Implementors of this trait can be used to iterate over a selected subset of
/// [`Graph`][gra] [`Triples`][tri]. The subset is selected by calling the
/// methods provided by the trait.
///
/// [gra]: trait.Graph.html
/// [tri]: struct.Triple.html
pub trait Query<'a, 'b: 'a>: Iterator<Item=&'a Triple<'b>> {
    /// Selects a `subject`.
    fn subject(self, subject: &'a str) -> Self;

    /// Selects a `predicate`.
    fn predicate(self, predicate: &'a str) -> Self;

    /// Selects an `object`.
    fn object(self, object: &'a str) -> Self;
}