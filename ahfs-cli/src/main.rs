extern crate ahfs;

mod app;
mod cliargs;

use std::env;
use std::fmt;
use std::process;

fn main() {
    let help = cliargs::FlagCell::new();
    let new_i = cliargs::FlagCell::new();

    let cli = cliargs::Parser {
        description: concat!(
            "AHFS Project Tool"
        ),
        rules: &[
            cliargs::Rule {
                name: "new",
                name_details: "<path>",
                description: "Create new AHFS project at <path>.",
                flags: &[
                    cliargs::Flag {
                        short: Some("i"),
                        long: "ignore-if-exists",
                        description: "Raise no error if project exists.",
                        out: cliargs::FlagOut::new(&new_i),
                    }
                ],
                callback: &|args| app::new(args, new_i.take_or(false)),
            },
            cliargs::Rule {
                name: "graph",
                name_details: "",
                description: "Creates graphs from project source files.",
                flags: &[],
                callback: &|args| app::graph(args),
            },
            cliargs::Rule {
                name: "list",
                name_details: "",
                description: "Lists all project source files.",
                flags: &[],
                callback: &|args| app::list(args),
            },
            cliargs::Rule {
                name: "status",
                name_details: "",
                description: "Show project status.",
                callback: &|args| app::status(args),
                flags: &[],
            },
            cliargs::Rule {
                name: "help",
                name_details: "",
                description: "Display this help message.",
                flags: &[],
                callback: &|_args| help.set(true),
            },
        ],
    };
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        error(&args[0], &"No command specified.");
    }
    if let Err(err) = cli.parse(&args[1..]) {
        error(&args[0], &err);
    }
    if help.take_or(false) {
        println!("{}", cli);
    }
}

fn error(bin: &str, message: &fmt::Display) -> ! {
    println!(concat!(
            "{}\n",
            "\n",
            "Run `{} help` to see a list of available commands."),
             message, bin);
    process::exit(1)
}