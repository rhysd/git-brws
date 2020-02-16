use git_brws::argv::Parsed;
use git_brws::{error, url};
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
        e.eprintln();
        exit(3);
    }
}
