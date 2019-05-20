use arspec_parser::{Error, Matcher};
use super::{Class, FontMetrics};

type R<T> = Result<T, Error<Class>>;
type M<'a> = Matcher<'a, Class>;

/// Attempt to consume all tokens in `m` and produce a [`FontMetrics`][fom]
/// instance.
///
/// [fom]: ../struct.FontMetrics.html
pub fn root(mut m: &mut M) -> R<FontMetrics> {
    Ok(FontMetrics {})
}