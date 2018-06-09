extern crate ahfs;

mod app;

use ahfs::cliargs;
use std::env;
use std::process;

fn main() {
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
                callback: &|_args| {},
            },
        ],
    };
    let args = env::args().collect::<Vec<String>>();
    if let Err(err) = cli.parse(&args[1..]) {
        println!(concat!(
            "{}\n",
            "\n",
            "Run `{} help` to see a list of available commands.")
                 , ahfs::format_error(&err).unwrap(), &args[0]);
        process::exit(1)
    }
    println!("{}", cli);
}