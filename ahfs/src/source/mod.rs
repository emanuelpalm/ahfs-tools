//! Various types and utilities related to managing source code texts.

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

/// A collection of source code [`Text`s](struct.Text.html).
pub struct Source {
    texts: Box<[Text]>,
}

impl Source {
    /// Creates new `Source` from given `texts`.
    pub fn new<S>(texts: S) -> Self
        where S: Into<Box<[Text]>>
    {
        Source { texts: texts.into() }
    }

    /// `Source` texts.
    #[inline]
    pub fn texts(&self) -> &[Text] {
        &self.texts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let texts = vec![
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