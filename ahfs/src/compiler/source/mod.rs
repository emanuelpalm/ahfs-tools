//! Various types and utilities related to managing source code texts.
//!
//! # Compiler Passes
//!
//! This module significantly contains the [`Source`][source] type, which is
//! used to hold both a collection of source code texts and an arbitrary data
//! structure created by analyzing its texts. The data structure, referred to as
//! as the source _tree_, can be transformed using the
//! [`Source::apply()`][apply] method by providing it with a _compiler pass_.
//!
//! [source]: struct.Source.html
//! [apply]: struct.Source.html#method.apply

mod error;
mod line;
mod lines;
mod region;
mod text;

pub use self::error::Error;
pub use self::line::Line;
pub use self::lines::Lines;
pub use self::region::Region;
pub use self::text::Text;

use std::ops;
use std::result;

/// Refers to a range of bytes within some arbitrary `str`.
pub type Range = ops::Range<usize>;

/// The result of a source-related compiler operation.
pub type Result<'a, T> = result::Result<T, Error<'a>>;

/// A collection of source code `texts` and their `tree` interpretation.
pub struct Source<'a, T> {
    texts: &'a [Text<'a>],
    tree: T,
}

impl<'a, T: 'a> Source<'a, T> {
    /// `Source` texts.
    #[inline]
    pub fn texts(&self) -> &'a [Text<'a>] {
        self.texts
    }

    /// `Source` tree.
    #[inline]
    pub fn tree(&self) -> &T {
        &self.tree
    }

    pub fn end_region(&self) -> Option<Region<'a>> {
        self.texts().last().map(|text| text.end_region())
    }

    /// Applies given `pass` to `Source`, potentially transforming its `tree`.
    #[inline]
    pub fn apply<P, U: 'a>(&'a self, pass: P) -> Result<'a, Source<'a, U>>
        where P: FnOnce(&'a Self) -> Result<'a, U>,
    {
        let tree = pass(self)?;
        Ok(Source { texts: self.texts, tree })
    }
}

impl<'a> Source<'a, ()> {
    /// Creates new `Source` from given `texts`.
    pub fn new<S>(texts: S) -> Self
        where S: Into<&'a [Text<'a>]>
    {
        Source { texts: texts.into(), tree: () }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let texts: &[Text] = &[
            Text::new("alpha.ahs", concat!(
                "A type System;\n",
                "A consumes B;\r\n",
                "A produces C;\n",
            )),
            Text::new("beta.ahs", concat!(
                "X",
            )),
        ];
        let source = Source::new(texts);
        let get = |index: usize, range: Range| {
            format!("{}", source.texts()
                .get(index).unwrap()
                .get_region(range)
                .unwrap())
        };
        {
            assert_eq!(get(1, 0..1).as_str(), concat!(
                "      : ", str_color!(blue: "beta.ahs"), "\n",
                "      |\n",
                "    1 | ", str_color!(none: "X"), "\n",
                "      | ", str_color!( red: "^"), "\n"));
        }
        {
            assert_eq!(get(0, 0..1).as_str(), concat!(
                "      : ", str_color!(blue: "alpha.ahs"), "\n",
                "      |\n",
                "    1 | ", str_color!(none: "A type System;"), "\n",
                "      | ", str_color!( red: "^"), "\n"));
        }
        {
            assert_eq!(get(0, 17..25).as_str(), concat!(
                "      : ", str_color!(blue: "alpha.ahs"), "\n",
                "      |\n",
                "    2 | ", str_color!(none: "A consumes B;"), "\n",
                "      | ", str_color!( red: "  ^^^^^^^^"), "\n"));
        }
        {
            assert_eq!(get(0, 30..42).as_str(), concat!(
                "      : ", str_color!(blue: "alpha.ahs"), "\n",
                "      |\n",
                "    3 | ", str_color!(none: "A produces C;"), "\n",
                "      | ", str_color!( red: "^^^^^^^^^^^^"), "\n"));
        }
        {
            assert_eq!(get(0, 17..40).as_str(), concat!(
                "      : ", str_color!(blue: "alpha.ahs"), "\n",
                "      |\n",
                "    2 | ", str_color!(none: "A consumes B;"), "\n",
                "      | ", str_color!( red: "  ^^^^^^^^^^^"), "\n",
                "    3 | ", str_color!(none: "A produces C;"), "\n",
                "      | ", str_color!( red: "^^^^^^^^^^"), "\n"));
        }
        {
            assert_eq!(get(0, 7..40).as_str(), concat!(
                "      : ", str_color!(blue: "alpha.ahs"), "\n",
                "      |\n",
                "    1 | ", str_color!(none: "A type System;"), "\n",
                "      | ", str_color!( red: "       ^^^^^^^"), "\n",
                "    2 | ", str_color!(none: "A consumes B;"), "\n",
                "      | ", str_color!( red: "^^^^^^^^^^^^^"), "\n",
                "     ...\n"));
        }
        {
            assert_eq!(get(0, 42..42).as_str(), concat!(
                "      : ", str_color!(blue: "alpha.ahs"), "\n",
                "      |\n",
                "    3 | ", str_color!(none: "A produces C;"), "\n",
                "      | ", str_color!( red: "            ^"), "\n"));
        }
    }
}