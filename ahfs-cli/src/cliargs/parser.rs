use std::fmt;
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
        let (arg_flags, arg_values) = args.iter()
            .take_while(|arg| arg.as_str() != "--")
            .fold((Vec::new(), Vec::new()), |(mut flags, mut values), arg| {
                match arg.starts_with("-") && arg.len() > 1 {
                    true => flags.push(arg as &str),
                    false => values.push(arg as &str),
                };
                (flags, values)
            });
        for rule in self.rules {
            if rule.apply(&arg_values, &arg_flags)? {
               return Ok(())
            }
        }
        Err(match arg_values.get(0) {
            Some(arg) => Error::ArgUnknown(arg.to_string()),
            None => Error::FlagUnknown(arg_flags[0].to_string()),
        })
    }
}

impl<'a> fmt::Display for Parser<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.description)?;
        for rule in self.rules {
            write!(f, "\n{}", rule)?;
        }
        Ok(())
    }
}