use std::fmt;
use super::{Error, Flag, Result};

/// A command line argument parsing rule.
pub enum Rule<'a> {
    /// A named action with associated flags.
    Action {
        /// Command line name, used to invoke action.
        name: &'static str,

        /// Human-readable description.
        description: &'static str,

        /// Any flags associated with action.
        flags: &'a [Flag],

        /// Function called if action is invoked.
        callback: &'a Fn(&[&str]),
    },
    /// A named submenu.
    Menu {
        /// Command line name, used to select menu.
        name: &'static str,

        /// Human-readable description.
        description: &'static str,

        /// Header of items menu.
        items_header: &'static str,

        /// Menu items.
        items: &'a [Rule<'a>],

        /// Function called if menu is invoked without an item.
        callback: &'a Fn(&[&str]),
    },
}

impl<'a> Rule<'a> {
    /// Tries to apply rule to given `args`.
    #[doc(hidden)]
    pub fn apply(&self, arg_values: &[&str], arg_flags: &[&str]) -> Result<bool> {
        if let Some((first, rest)) = arg_values.split_first() {
            match self {
                &Rule::Action { ref name, flags, callback, .. } => {
                    if first != name {
                        return Ok(false);
                    }
                    parse_flags(flags, arg_flags)?;
                    (callback)(rest);
                    return Ok(true);
                }
                &Rule::Menu { ref name, items, callback, .. } => {
                    if first != name {
                        return Ok(false);
                    }
                    for rule in items {
                        if rule.apply(rest, arg_flags)? {
                            return Ok(true);
                        }
                    }
                    (callback)(rest);
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}

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
        match self {
            &Rule::Action { name, description, flags, .. } => {
                writeln!(f, "{}\n    {}", name, description)?;
                if flags.len() > 0 {
                    writeln!(f)?;
                    for flag in flags {
                        writeln!(f, "    {}", flag)?;
                    }
                }
            }
            &Rule::Menu { name, description, items_header, items, .. } => {
                writeln!(f, "{}\n    {}", name, description)?;
                if items.len() > 0 {
                    writeln!(f, "\n    {}", items_header)?;
                    for item in items {
                        let (name, description) = match *item {
                            Rule::Action { name, description, .. } |
                            Rule::Menu { name, description, .. } => {
                                (name, description)
                            }
                        };
                        writeln!(f, "        {}{:offset$}    {}",
                                 name, "", description,
                                 offset = 18usize.saturating_sub(name.len()))?;
                    }
                }
            }
        }
        Ok(())
    }
}