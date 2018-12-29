#[macro_use]
extern crate serde_derive;

mod argv;
mod command;
mod env;
mod error;
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

fn run() -> error::Result<()> {
    let argv = args().collect::<Vec<_>>();
    match parse_options(argv.as_slice())? {
        ParsedArgv::Help(usage) => eprintln!("{}", usage),
        ParsedArgv::Version(version) => println!("{}", version),
        ParsedArgv::Parsed(opts) => command::browse(&opts)?,
    }
    Ok(())
}

// Note: fn main() -> error::Result<()> is not available since it uses {:?} for error message.
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        exit(3);
    }
}
