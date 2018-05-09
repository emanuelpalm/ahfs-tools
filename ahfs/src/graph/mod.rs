//! Arrowhead specification graph and query utilities.
//!
//! This module contains tools useful for finding data of interest in
//! Arrowhead specification data.

mod query;
mod query_iter;
mod triple;

pub use self::query::Query;
pub use self::query_iter::QueryIter;
pub use self::triple::Triple;

use std::slice::Iter;

/// A query-able graph of [`Triples`](struct.Triple.html).
pub trait Graph<'a, 'b: 'a> {
    /// Graph [`Query`](trait.Query.html) type.
    type Query: Query<'a, 'b>;

    /// Creates new object useful for making a query against this `Graph`.
    fn query(&self) -> Self::Query;
}

impl<'a, 'b: 'a> Graph<'a, 'b> for &'a [Triple<'b>] {
    type Query = QueryIter<'a, 'b, Iter<'a, Triple<'b>>>;

    #[inline]
    fn query(&self) -> Self::Query {
        QueryIter::new(self.iter())
    }
}