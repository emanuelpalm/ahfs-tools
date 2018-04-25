use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

pub struct Settings {
    path: Box<Path>,
    data: HashMap<Box<str>, Box<str>>,
}

impl Settings {
    pub fn read<P>(path: P) -> io::Result<Settings>
        where P: Into<Box<Path>>,
    {
        let path = path.into();
        let mut source = String::new();
        File::open(&path)?.read_to_string(&mut source)?;
        Ok(Settings { // TODO: Parse key:value pairs directly. No hash map!
            path,
            data: source.lines()
                .filter_map(|line| {
                    let (key, value) = line.split_at(line.find(":")?);
                    Some((key.into(), value.into()))
                })
                .collect(),
        })
    }
}