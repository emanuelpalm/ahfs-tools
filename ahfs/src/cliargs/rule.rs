use std::fmt;
use super::{Error, Flag, Result};

pub struct Rule<'a> {
    /// Name used to invoke rule.
    pub name: &'static str,

    /// Additional name details, such as argument descriptions.
    pub name_details: &'static str,

    /// Human-readable description of rule.
    pub description: &'static str,

    /// Command line flags.
    pub flags: &'a [Flag],

    /// Function called if action is invoked.
    pub callback: &'a Fn(&[&str]),
}

impl<'a> Rule<'a> {
    #[doc(hidden)]
    pub fn apply(&self, args: &[&str], flags: &[&str]) -> Result<bool> {
        if let Some((first, rest)) = args.split_first() {
            if first != &self.name {
                return Ok(false);
            }
            parse_flags(self.flags, flags)?;
            (self.callback)(rest);
            return Ok(true);
        }
        Ok(false)
    }
}

#[inline]
fn parse_flags(flags: &[Flag], args: &[&str]) -> Result<()> {
    for arg in args {
        let (is_long, name) = match arg.starts_with("--") {
            true => (true, &arg[2..]),
            false => (false, &arg[1..]),
        };
        let (name, value) = match name.find("=") {
            Some(offset) => (&name[..offset], &name[(offset + 1)..]),
            None => (name, ""),
        };
        let flag = {
            let mut it = flags.iter();
            let flag = match is_long {
                true => it.find(|flag| flag.long == name),
                false => it.find(|flag| flag.short == Some(name)),
            };
            flag.ok_or_else(|| Error::FlagUnknown(arg.to_string()))?
        };
        flag.out.write(value)?;
    }
    Ok(())
}

impl<'a> fmt::Display for Rule<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "{} {}\n    {}\n",
            self.name, self.name_details, self.description
        )?;
        if self.flags.len() > 0 {
            write!(f, "\n")?;
            for flag in self.flags {
                write!(f, "    {}\n", flag)?;
            }
        }
        Ok(())
    }
}