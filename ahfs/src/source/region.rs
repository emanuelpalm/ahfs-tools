use std::fmt;
use super::{Lines, Range, Text};

/// Represents a significant region within a source code text.
#[derive(Clone, Debug)]
pub struct Region<'a> {
    text: &'a Text,
    range: Range,
}

impl<'a> Region<'a> {
    /// It is the responsibility of the caller to ensure given
    /// [`range`](type.Range.html) is within valid UTF-8 bounds of given
    /// [`text`](struct.Text.html).
    #[doc(hidden)]
    #[inline]
    pub unsafe fn new(text: &'a Text, range: Range) -> Self {
        Region { text, range }
    }

    /// Gets string representing only significant range within this `Region`.
    #[inline]
    pub fn as_str(&self) -> &'a str {
        unsafe {
            self.text.body().get_unchecked(self.range.clone())
        }
    }

    /// Gets iterator over lines touched by this `Region`.
    #[inline]
    pub fn lines(&self) -> Lines<'a> {
        unsafe { Lines::new(self.text.body(), self.range.clone()) }
    }

    /// Byte range of this `Region` within its `text`.
    #[inline]
    pub fn range(&self) -> &Range {
        &self.range
    }

    /// [`Text`](struct.Text.html) in which `Region` is located.
    #[inline]
    pub fn text(&self) -> &'a Text {
        self.text
    }
}

impl<'a> AsRef<str> for Region<'a> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> fmt::Display for Region<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, concat!(
            "      : ", str_color!(blue: "{}"), "\n",
            "      |\n"), self.text.name())?;
        for (i, line) in self.lines().enumerate() {
            if i < 2 {
                write!(f, "{}", line)?;
            } else {
                writeln!(f, "     ...")?;
                break;
            }
        }
        Ok(())
    }
}

impl<'a> PartialEq<str> for Region<'a> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<'a> PartialEq<Region<'a>> for str {
    #[inline]
    fn eq(&self, other: &Region<'a>) -> bool {
        other == self
    }
}