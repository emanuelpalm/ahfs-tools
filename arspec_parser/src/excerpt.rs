use arspec_macro::color;
use crate::{Lines, Range, Text};
use std::fmt;

/// Owned part of some original [`Text`][txt] containing a significant range
/// of characters.
///
/// [txt]: struct.Text.html
#[derive(Debug)]
pub struct Excerpt {
    /// Source text excerpt.
    pub text: Text,

    /// Number of first line in source excerpt.
    pub line_number: usize,

    /// Range of significant characters in source excerpt.
    pub range: Range,
}

impl fmt::Display for Excerpt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, concat!(
            "      : ", color!(b: "{}"), "\n",
            "      |\n"), self.text.name)?;
        let lines = Lines {
            source: &self.text.body,
            number: self.line_number,
            range: Some(self.range),
        };
        for line in lines {
            write!(f, "{}", line)?;
        }
        Ok(())
    }
}
