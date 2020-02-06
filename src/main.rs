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
use std::error::Error;
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

fn print_error(err: impl Error) {
    fn print_causes(err: &dyn Error) {
        eprint!(": {}", err);
        if let Some(err) = err.source() {
            print_causes(err);
        }
    }
    eprint!("Error: {}", err);
    if let Some(cause) = err.source() {
        print_causes(cause);
    }
    eprintln!();
}

// Note: fn main() -> error::Result<()> is not available since it uses {:?} for error message.
fn main() {
    if let Err(err) = run() {
        print_error(err);
        exit(3);
    }
}
