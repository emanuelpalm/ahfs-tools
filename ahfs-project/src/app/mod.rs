mod error;

pub use self::error::Error;

use ahfs::project::Project;
use ahfs::util;

/// Creates new project at first path in `args`.
pub fn new(args: &[&str], ignore_if_exists: bool) -> ! {
    util::exit_after(|| {
        if args.len() != 1 {
            return Err(Error::NewArgMissing.into());
        }
        match Project::create(args[0].clone()) {
            Ok(_) => Ok(()),
            Err(err) => if err.error_code() == "F010" && ignore_if_exists {
                Ok(())
            } else {
                Err(err)
            }
        }
    })
}

/// Describes what status information to print.
#[derive(Eq, PartialEq)]
pub enum Status {
    AhfsVersion,
    Summary,
}

/// Prints various project status information and exists.
pub fn status(status: Status) -> ! {
    util::exit_after(|| {
        let project = Project::locate(".")?;

        println!("Project:      {}", project.root().canonicalize()?.file_name()
            .unwrap().to_string_lossy());

        if status == Status::AhfsVersion || status == Status::Summary {
            println!("AHFS Version: {}", project.settings().ahfs_version())
        }
        Ok(())
    })
}