extern crate ahfs;

mod app;

use ahfs::cliargs;
use std::env;
use std::process;

fn main() {
    let ignore_if_exists = cliargs::FlagCell::new();
    let cli = cliargs::Parser {
        description: concat!(
            "AHFS Project Tool"
        ),
        rules: &[
            cliargs::Rule::Action {
                name: "new",
                description: "Create new AHFS project in current folder.",
                flags: &[
                    cliargs::Flag {
                        short: Some("i"),
                        long: "ignore-if-exists",
                        description: "Raise no error if project exists.",
                        out: cliargs::FlagOut::new(&ignore_if_exists),
                    }
                ],
                callback: &|args| app::new(args, ignore_if_exists.take_or(false)),
            },
            cliargs::Rule::Menu {
                name: "status",
                description: "Show project status.",
                items_header: "Subcommands:",
                items: &[
                    cliargs::Rule::Action {
                        name: "ahfs-version",
                        description: "Display only AHFS compatibility version.",
                        flags: &[],
                        callback: &|_args| app::status(app::Status::AhfsVersion),
                    }
                ],
                callback: &|_args| app::status(app::Status::Summary),
            },
            cliargs::Rule::Action {
                name: "help",
                description: "Display this help message.",
                flags: &[],
                callback: &|_args| {},
            },
        ],
    };
    let args = env::args().collect::<Vec<String>>();
    if let Err(err) = cli.parse(&args[1..]) {
        println!(concat!(
            "{}\n",
            "\n",
            "Run `{} help` for a list of available commands.")
                 , err, &args[0]);
        process::exit(1)
    }
    println!("{}", cli);
}