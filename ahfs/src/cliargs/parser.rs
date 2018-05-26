use super::{Result, Rule, Error};

pub struct Parser<'a> {
    pub description: &'static str,
    pub rules: &'a [Rule<'a>],
}

impl<'a> Parser<'a> {
    pub fn parse<S>(&self, args: S) -> Result<()>
        where S: AsRef<[String]>,
    {
        return parse(self.rules, args.as_ref());
    }
}

fn parse<'a>(rules: &'a [Rule<'a>], args: &[String]) -> Result<()> {
    if rules.len() == 0 || args.len() == 0 {
        return Ok(());
    }
    for rule in rules {
        if rule.try_args(args)? {
            return Ok(());
        }
    }
    Err(Error::ArgUnknown(args[0].clone()))
}