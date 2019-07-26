mod error;

pub use self::error::Error;

use arspec::spec::parser;
use arspec::project::Project;
use arspec_doc::{Font, FontStyle, FontWeight, html, scripts, styles, svg};
use arspec_parser::Corpus;
use crate::log;
use std::fs;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Generates documentation files.
pub fn doc(args: &[&str], skip_verification: bool) -> arspec::Result {
    if args.len() != 0 {
        return Err(Error::DocArgCountNot0.into());
    }

    // Load project.
    let project = Project::locate(".")?;
    let target_path = &project.target();
    fs::create_dir_all(&target_path)?;

    // Load project specification.
    let corpus = Corpus::read_from(project.files()?.iter())?;
    let spec = parser::parse(&corpus)?;

    // Verify specification correctness.
    if !skip_verification {
        &spec.verify()?;
    }

    let mut buffer = Vec::<u8>::new();

    // Generate figures.
    {
        let figures_path = &target_path.join("figures");
        fs::create_dir_all(figures_path)?;

        for enum_ in &spec.enums {
            buffer.clear();
            svg::render(&enum_, false, &mut buffer)?;
            let target_path = figures_path
                .join(format!("enum-{}.svg", enum_.name.as_str()));

            fs::write(target_path, &mut buffer)?;
        }

        for record in &spec.records {
            buffer.clear();
            svg::render(&record, false, &mut buffer)?;
            let target_path = figures_path
                .join(format!("record-{}.svg", record.name.as_str()));

            fs::write(target_path, &mut buffer)?;
        }

        for service in &spec.services {
            buffer.clear();
            svg::render(&service, false, &mut buffer)?;
            let target_path = figures_path
                .join(format!("service-{}.svg", service.name.as_str()));

            fs::write(target_path, &mut buffer)?;
        }

        for system in &spec.systems {
            buffer.clear();
            svg::render(&system, false, &mut buffer)?;
            let target_path = figures_path
                .join(format!("system-{}.svg", system.name.as_str()));

            fs::write(target_path, &mut buffer)?;
        }
    }

    // Create font files.
    {
        let fonts_path = &target_path.join("fonts");
        fs::create_dir_all(fonts_path)?;

        for font in Font::all() {
            fs::File::create(fonts_path.join(font.source_name()))
                .and_then(|mut file| file.write_all(font.source()))?;
        }
    }

    // Create script files.
    {
        let scripts_path = &target_path.join("scripts");
        fs::create_dir_all(scripts_path)?;

        fs::File::create(scripts_path.join("main.js"))
            .and_then(|mut file| file.write_all(scripts::MAIN))?;
    }

    // Create style files.
    {
        let styles_path = &target_path.join("styles");
        fs::create_dir_all(styles_path)?;

        fs::File::create(styles_path.join("fonts.css"))
            .and_then(|mut file| {
                for font in Font::all() {
                    write!(
                        file,
                        concat!(
                            "@font-face {{\n",
                            "  font-family: \"{}\";\n",
                            "  src: \"../fonts/{}\" format('truetype');\n",
                        ),
                        font.name(), font.source_name(),
                    )?;
                    let style = font.style();
                    if *style != FontStyle::Normal {
                        write!(file, "  font-style: \"{}\";\n", style)?;
                    }
                    let weight = font.weight();
                    if *weight != FontWeight::Normal {
                        write!(file, "  font-weight: \"{}\";\n", weight)?;
                    }
                    write!(file, "}}\n\n")?;
                }
                Ok(())
            })?;

        fs::File::create(styles_path.join("print.css"))
            .and_then(|mut file| file.write_all(styles::PRINT))?;

        fs::File::create(styles_path.join("screen.css"))
            .and_then(|mut file| file.write_all(styles::SCREEN))?;
    }

    // Generate HTML files.
    {
        let scripts = &[
            Path::new("scripts/main.js"),
        ];
        let styles = &[
            html::Style { path: Path::new("styles/fonts.css"), media: html::StyleMedia::ALL },
            html::Style { path: Path::new("styles/print.css"), media: html::StyleMedia::PRINT },
            html::Style { path: Path::new("styles/screen.css"), media: html::StyleMedia::SCREEN },
        ];

        for system in &spec.systems {
            buffer.clear();
            html::render(&system, scripts, styles, &mut buffer)?;
            let target_path = target_path
                .join(format!("{}-SysD.html", system.name.as_str()));

            fs::write(target_path, &mut buffer)?;
        }
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
