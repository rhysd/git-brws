#[macro_use]
extern crate serde_derive;

mod argv;
mod command;
mod env;
mod errors;
mod git;
mod github_api;
mod page;
mod pull_request;
mod service;

#[cfg(test)]
mod test;

use crate::argv::{parse_options, ParsedArgv};
use std::env::args;
use std::process::exit;

fn main() {
    let argv = args().collect::<Vec<_>>();
    let parsed = match parse_options(argv.as_slice()) {
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
        ParsedArgv::Parsed(opts) => command::browse(&opts),
    };

    if let Some(m) = msg {
        eprintln!("{}", m);
        exit(3);
    } else {
        exit(0);
    }
}
