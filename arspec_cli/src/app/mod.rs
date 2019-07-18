mod error;

pub use self::error::Error;

use arspec::spec::parser;
use arspec::project::Project;
use arspec_doc::Font;
use arspec_doc::svg;
use arspec_parser::Text;
use crate::log;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;

/// Prints list of all project source files and exits.
pub fn doc(args: &[&str]) -> arspec::Result {
    if args.len() != 0 {
        return Err(Error::ListArgCountNot0.into());
    }
    let project = Project::locate(".")?;
    fs::create_dir_all(&project.target())?;

    let mut buffer = Vec::<u8>::new();

    for path in project.files()?.iter() {
        let source = Text::read_at(path)?;
        let tree = parser::parse(&source)?;

        for record in tree.records {
            buffer.clear();
            svg::render(&record,  &mut buffer)?;
            let target_path = project.target()
                .join(format!("record-{}.svg", record.name.as_str()));

            fs::write(target_path, &mut buffer)?;
        }

        for service in tree.services {
            buffer.clear();
            svg::render(&service, &mut buffer)?;
            let target_path = project.target()
                .join(format!("service-{}.svg", service.name.as_str()));

            fs::write(target_path, &mut buffer)?;
        }

        for system in tree.systems {
            buffer.clear();
            svg::render(&system, &mut buffer)?;
            let target_path = project.target()
                .join(format!("system-{}.svg", system.name.as_str()));

            fs::write(target_path, &mut buffer)?;
        }
    }

    for font in Font::all() {
        fs::File::create(project.target().join(font.source_name()))
            .and_then(|mut file| file.write_all(font.source()))?;
    }

    Ok(())
}

/// Prints list of all project source files and exits.
pub fn list(args: &[&str]) -> arspec::Result {
    if args.len() != 0 {
        return Err(Error::ListArgCountNot0.into());
    }
    let project = Project::locate(".")?;
    let files = project.files()?;
    for file in files.iter() {
        log::completion(&file.canonicalize()?.to_string_lossy());
    }
    log::completion(&format!("Files found: {}", files.len()));
    Ok(())
}

/// Creates new project at path in `args` at index 0 and exits.
pub fn new(args: &[&str], ignore_if_exists: bool, name: Option<String>) -> arspec::Result {
    match args {
        &[path] => {
            let path: PathBuf = path.into();
            let name = name.unwrap_or(path
                .file_name()
                .map(|name| name.to_string_lossy().into())
                .unwrap_or("Empty Project".into()));

            match Project::create(name, path) {
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
        }
        _ => Err(Error::NewArgCountNot1.into()),
    }
}

/// Prints various project status information and exits.
pub fn status(args: &[&str]) -> arspec::Result {
    if args.len() != 0 {
        return Err(Error::StatusArgCountNot0.into());
    }
    let project = Project::locate(".")?;
    let conf = project.configuration();
    log::completion(&format!(
        concat!(
              "Project:     {}\n",
            "  Version:     {}\n",
            "  Path:        {}\n",
            "  Description: {}\n",
        ),
        conf.name,
        conf.version,
        project.root().canonicalize()?.to_string_lossy(),
        conf.description.as_ref().unwrap_or(&"<none>".to_string()),
    ));
    Ok(())
}