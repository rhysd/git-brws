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

#[allow(clippy::unit_arg)]
fn main() -> error::Result<()> {
    let argv = args().collect::<Vec<_>>();
    let parsed = parse_options(argv.as_slice())?;
    match parsed {
        ParsedArgv::Help(usage) => Ok(eprintln!("{}", usage)),
        ParsedArgv::Version(version) => Ok(println!("{}", version)),
        ParsedArgv::Parsed(opts) => command::browse(&opts),
    }
}
