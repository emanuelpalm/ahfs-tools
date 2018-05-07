use std::fmt;
use super::{LineIter};

/// Represents a type holding a set of lines that touches some significant
/// region of characters.
pub trait Lines {
    /// Creates new line iterator.
    fn lines<'a>(&'a self) -> LineIter<'a>;

    /// Writes lines, with given `header`, in human-readable form.
    fn fmt(&self, f: &mut fmt::Formatter, header: &str) -> fmt::Result {
        write!(f, concat!(
            "      : ", str_color!(blue: "{}"), "\n",
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