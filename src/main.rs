#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

mod argv;
mod async_runtime;
mod config;
mod error;
mod git;
mod github_api;
mod page;
mod pull_request;
mod service;
mod url;

#[cfg(test)]
mod test;

use crate::argv::Parsed;
use std::env::args;
use std::process::exit;

fn run() -> error::Result<()> {
    match Parsed::from_iter(args())? {
        Parsed::Help(usage) => eprintln!("{}", usage),
        Parsed::Version(version) => println!("{}", version),
        Parsed::OpenPage(ref opts) if opts.stdout => println!("{}", url::build_url(opts)?),
        Parsed::OpenPage(ref opts) => url::browse(&url::build_url(opts)?, &opts.env)?,
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
