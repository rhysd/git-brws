mod argv;
mod command;
mod git;
mod page;
mod service;
mod util;

#[cfg(test)]
mod test;

use argv::{parse_options, ParsedArgv};
use std::env;
use std::process::exit;

fn main() {
    let argv = env::args().collect::<Vec<_>>();
    let parsed = match parse_options(argv) {
        Ok(p) => p,
        Err(reason) => {
            eprintln!("{}", reason);
            exit(3);
        }
    };

    let msg = match parsed {
        ParsedArgv::Help(usage) => {
            eprintln!("{}", usage);
            None
        }
        ParsedArgv::Version(version) => {
            println!("{}", version);
            None
        }
        ParsedArgv::Parsed(opts, false) => command::browse(opts),
        ParsedArgv::Parsed(opts, true) => match command::url(opts) {
            Ok(url) => {
                println!("{}", url);
                None
            }
            Err(msg) => Some(msg),
        },
    };

    if let Some(m) = msg {
        eprintln!("{}", m);
        exit(3);
    } else {
        exit(0);
    }
}
