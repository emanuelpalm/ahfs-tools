mod error;

pub use self::error::Error;

use ahfs::project::Project;
use ahfs::util;
use std::io;

/// Creates new project at first path in `args`.
pub fn new(args: &[&str], ignore_if_exists: bool) -> ! {
    util::exit_after(|| {
        if args.len() != 1 {
            return Err(Error::NewArgCountNot1.into());
        }
        match Project::create(args[0].clone()) {
            Ok(_) => Ok(()),
            Err(err) => {
                if ignore_if_exists {
                    let ignore = err.as_io_error().map_or(false, |err| {
                        err.kind() == io::ErrorKind::AlreadyExists
                    });
                    if ignore {
                        return Ok(());
                    }
                }
                Err(err)
            }
        }
    })
}

/// Prints various project status information and exists.
pub fn status(args: &[&str]) -> ! {
    util::exit_after(|| {
        if args.len() != 0 {
            return Err(Error::StatusArgCountNot0.into());
        }
        let project = Project::locate(".")?;
        println!("Project:      {}", project.root().canonicalize()?.file_name()
            .unwrap().to_string_lossy());
        println!("AHFS Version: {}", project.settings().ahfs_version());
        Ok(())
    })
}