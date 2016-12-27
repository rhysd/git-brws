#[macro_use] mod util;
mod argv;
mod git;
mod command;
mod page;
mod service;

use std::env;
use std::io::Write;
use std::process::exit;
use argv::{parse_options, ParsedArgv};

fn main() {
    let argv = env::args().collect::<Vec<_>>();
    let parsed = match parse_options(argv) {
        Ok(p) => p,
        Err(reason) => {
            errorln!("{}", reason);
            exit(3);
        },
    };

    let msg = match parsed {
        ParsedArgv::Help | ParsedArgv::Version => None,
        ParsedArgv::Parsed(opts, false) => command::browse(opts),
        ParsedArgv::Parsed(opts, true) => {
            match command::url(opts) {
                Ok(url) => {
                    println!("{}", url);
                    None
                },
                Err(msg) => Some(msg),
            }
        },
    };

    if let Some(m) = msg {
        errorln!("{}", m);
        exit(3);
    } else {
        exit(0);
    }
}
