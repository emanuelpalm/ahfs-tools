use super::{Result, Rule, Error};

/// A description of how command line arguments are to be parsed.
pub struct Parser<'a> {
    /// Parser description.
    pub description: &'static str,

    /// Parser rules.
    pub rules: &'a [Rule<'a>],
}

impl<'a> Parser<'a> {
    /// Parses given array of `String`s.
    pub fn parse<S>(&self, args: S) -> Result<()>
        where S: AsRef<[String]>,
    {
        let args = args.as_ref();
        if self.rules.len() == 0 || args.len() == 0 {
            return Ok(());
        }
        for rule in self.rules {
            if rule.try_args(args)? {
                return Ok(());
            }
        }
        Err(Error::ArgUnknown(args[0].clone()))
    }
}