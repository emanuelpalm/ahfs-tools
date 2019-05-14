use ahfs_macro::color;
use crate::source::LineIter;
use std::fmt;

/// Represents a type holding a set of lines that touches some significant
/// region of characters.
pub trait Lines {
    /// Creates new line iterator.
    fn lines(&self) -> LineIter;

    /// Writes lines, with given `header`, in human-readable form.
    fn fmt(&self, f: &mut fmt::Formatter, header: &str) -> fmt::Result {
        write!(f, concat!(
            "      : ", color!(b: "{}"), "\n",
            "      |\n"), header)?;
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