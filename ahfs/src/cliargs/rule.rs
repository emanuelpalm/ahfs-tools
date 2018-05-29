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
        callback: &'a Fn(&[String]),
    },
    /// A named submenu.
    Menu {
        /// Command line name, used to select menu.
        name: &'static str,

        /// Human-readable description.
        description: &'static str,

        /// Menu items.
        items: &'a [Rule<'a>],
    },
}

impl<'a> Rule<'a> {
    /// Tries to apply rule to given `args`.
    #[doc(hidden)]
    pub fn try_args(&self, args: &[String]) -> Result<bool> {
        if let Some((first, rest)) = args.split_first() {
            // TODO: Check if flag ...
            match self {
                &Rule::Action { name, flags, callback, .. } => {
                    if first != name {
                        return Ok(false);
                    }
                    let it = ArgFlagIter::new(rest, flags);
                    let mut offset = 0;
                    for pair in it {
                        offset += 1;
                        let (flag, value) = pair?;
                        flag.out.write(value)?;
                    }
                    (callback)(&args[offset..]);
                    return Ok(true);
                }
                &Rule::Menu { name, items, .. } => {
                    if first != name {
                        return Ok(false);
                    }
                    for rule in items {
                        if rule.try_args(rest)? {
                            return Ok(true);
                        }
                    }
                }
            }
        }
        Ok(false)
    }
}

struct ArgFlagIter<'a, 'b: 'a> {
    args: &'a [String],
    flags: &'b [Flag],
    offset: usize,
}

impl<'a, 'b: 'a> ArgFlagIter<'a, 'b> {
    #[inline]
    fn new(args: &'a [String], flags: &'b [Flag]) -> Self {
        ArgFlagIter { args, flags, offset: 0 }
    }
}

impl<'a, 'b: 'a> Iterator for ArgFlagIter<'a, 'b> {
    type Item = Result<(&'a Flag, &'a str)>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(arg) = self.args.get(self.offset) {
            self.offset += 1;
            if arg.starts_with("--") {
                if arg.len() == 2 {
                    return None;
                }
                let (long, value) = match arg.find("=") {
                    Some(offset) => (&arg[2..offset], &arg[(offset + 1)..]),
                    None => (&arg[2..], ""),
                };
                return self.flags.iter()
                    .find(|flag| flag.long == long)
                    .map(|flag| Ok((flag, value)))
                    .or_else(|| Some(Err(Error::FlagUnknown(arg.clone()))));
            }
            if arg.starts_with("-") {
                let short = &arg[1..];
                let value = match self.args.get(self.offset) {
                    Some(value) => {
                        self.offset += 1;
                        value
                    },
                    None => "",
                };
                return self.flags.iter()
                    .find(|flag| flag.short == Some(short))
                    .map(|flag| Ok((flag, value)))
                    .or_else(|| Some(Err(Error::FlagUnknown(arg.clone()))));
            }
            self.offset -= 1;
        }
        None
    }
}