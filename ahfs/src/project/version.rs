use std::fmt;

/// A MAJOR.MINOR.PATCH version indicator.
#[derive(Clone, Copy, Eq, PartialEq)]
#[derive(Debug)]
pub struct Version {
    major: usize,
    minor: usize,
    patch: usize,
}

impl Version {
    /// Creates new `Version` from given `major`, `minor` and `patch` integers.
    #[inline]
    pub fn new(major: usize, minor: usize, patch: usize) -> Self {
        Version { major, minor, patch }
    }

    /// Reads version indicator from given `text`.
    ///
    /// For parsing to succeed, `text` must be a string of only three positive
    /// base 10 integers, separated by two dots. There may not be any whitespace
    /// in the `text`. None of the integers may be larger than the `usize` type
    /// provided by the system.
    ///
    /// Examples of valid `text`s could be `"1.2.3"` or `"0.12.90312"`.
    pub fn parse<S>(text: S) -> Option<Version>
        where S: AsRef<str>
    {
        let mut iter = text.as_ref().splitn(3, ".");
        let major = iter.next()?.parse().ok()?;
        let minor = iter.next()?.parse().ok()?;
        let patch = iter.next()?.parse().ok()?;
        Some(Self::new(major, minor, patch))
    }

    /// Major `Version` indicator.
    #[inline]
    pub fn major(&self) -> usize {
        self.major
    }

    /// Minor `Version` indicator.
    #[inline]
    pub fn minor(&self) -> usize {
        self.minor
    }

    /// Patch `Version` indicator.
    #[inline]
    pub fn patch(&self) -> usize {
        self.patch
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let assert_some = |major, minor, patch, string| {
            assert_eq!(
                Some(Version::new(major, minor, patch)),
                Version::parse(string)
            );
        };
        assert_some(1,2,3, "1.2.3");
        assert_some(0,100,0, "0.100.0");
        assert_some(9919, 2, 1234, "9919.2.1234");

        let assert_none = |string| {
            assert_eq!(None, Version::parse(string));
        };
        assert_none("1.2");
        assert_none("x.2.3");
        assert_none("1.x.3");
        assert_none("1.2.x");
        assert_none(".1.2.3");
        assert_none("1.2.3.");
        assert_none(" 1.2.3");
        assert_none("1.2.3 ");
        assert_none("1. 2.3");
        assert_none("1.2..3");
        assert_none("a.b.c");
    }
}