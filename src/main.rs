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

use crate::argv::Parsed;
use std::env::args;
use std::process::exit;

fn run() -> error::Result<()> {
    match Parsed::from_iter(args())? {
        Parsed::Help(usage) => eprintln!("{}", usage),
        Parsed::Version(version) => println!("{}", version),
        Parsed::OpenPage(opts) => command::browse(&opts)?,
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
