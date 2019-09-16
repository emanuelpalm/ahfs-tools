mod app;
mod cliargs;
mod log;

use arspec::Error;
use arspec_macro::color;
use std::env;
use std::process;

fn main() {
    let doc_s = cliargs::FlagCell::new();
    let help = cliargs::FlagCell::new();
    let new_i = cliargs::FlagCell::new();
    let new_n = cliargs::FlagCell::new();

    let cli = cliargs::Parser {
        description: concat!(
            color!(g: "Available AHFS commands:")
        ),
        rules: &[
            cliargs::Rule {
                name: "doc",
                name_details: "",
                description: "Generate project documentation files.",
                flags: &[
                    cliargs::Flag {
                        short: Some("s"),
                        long: "skip-verifications",
                        description: "Skip some source file verifications.",
                        out: cliargs::FlagOut::new_bool(&doc_s),
                    },
                ],
                callback: &|args| app::doc(args, doc_s.take_or(false)),
            },
            cliargs::Rule {
                name: "help",
                name_details: "",
                description: "Display this help message.",
                flags: &[],
                callback: &|_args| {
                    help.set(true);
                    Ok(())
                },
            },
            cliargs::Rule {
                name: "list",
                name_details: "",
                description: "List all project source files.",
                flags: &[],
                callback: &|args| app::list(args),
            },
            cliargs::Rule {
                name: "new",
                name_details: "<path>",
                description: concat!(
                    "Create new AHFS project at ",
                    color!(g: "<path>"),
                    "."
                ),
                flags: &[
                    cliargs::Flag {
                        short: Some("i"),
                        long: "ignore-if-exists",
                        description: "Raise no error if project exists.",
                        out: cliargs::FlagOut::new_bool(&new_i),
                    },
                    cliargs::Flag {
                        short: Some("n"),
                        long: "name",
                        description: "Set project name.",
                        out: cliargs::FlagOut::new_string(&new_n),
                    },
                ],
                callback: &|args| {
                    app::new(args, new_i.take_or(false), new_n.take())
                },
            },
            cliargs::Rule {
                name: "status",
                name_details: "",
                description: "Show project status.",
                callback: &|args| app::status(args),
                flags: &[],
            },
        ],
    };
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        log::anomaly(&"No command specified.");
    }
    if let Err(error) = cli.parse(&args[1..]) {
        failure(&error);
    }
    if help.take_or(false) {
        log::completion(&cli);
    }
}

fn failure(error: &dyn Error) -> ! {
    log::failure(error);
    log::suggestion(&"Run `arspec help` to see a list of available commands.");
    process::exit(1)
}