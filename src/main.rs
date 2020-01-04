#[macro_use]
extern crate serde_derive;

mod argv;
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

async fn run() -> error::Result<()> {
    match Parsed::from_iter(args()).await? {
        Parsed::Help(usage) => eprintln!("{}", usage),
        Parsed::Version(version) => println!("{}", version),
        Parsed::OpenPage(ref opts) if opts.stdout => println!("{}", url::build_url(opts).await?),
        Parsed::OpenPage(ref opts) => url::browse(&url::build_url(opts).await?, &opts.env)?,
    }
    Ok(())
}

// Note: fn main() -> error::Result<()> is not available since it uses {:?} for error message.
#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        exit(3);
    }
}
