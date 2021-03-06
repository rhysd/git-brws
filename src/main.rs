#![forbid(unsafe_code)]
#![warn(clippy::dbg_macro)]
#![cfg(not(tarpaulin_include))]

use git_brws::argv::Parsed;
use git_brws::{error, url};
use std::env::args;
use std::process::exit;

fn run() -> error::Result<()> {
    match Parsed::parse_iter(args())? {
        Parsed::Help(usage) => eprintln!("{}", usage),
        Parsed::Version(version) => println!("{}", version),
        Parsed::OpenPage(opts) if opts.stdout => println!("{}", url::build_url(&opts)?),
        Parsed::OpenPage(opts) => url::browse(&url::build_url(&opts)?, &opts.env)?,
    }
    Ok(())
}

// Note: fn main() -> error::Result<()> is not available since it uses {:?} for error message.
fn main() {
    if let Err(e) = run() {
        e.eprintln();
        exit(3);
    }
}
