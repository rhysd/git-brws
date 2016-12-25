extern crate getopts;

use std::env;
use std::process::exit;
use std::io::Write;
use getopts::Options;

#[derive(Debug)]
struct CommandOptions {
    repo: Option<String>,
    dir: String,
    args: Vec<String>,
}

macro_rules! errorln(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

enum ParsedArgv {
    Help,
    Version,
    Parsed(CommandOptions),
}

fn parse_options(argv: Vec<String>) -> Result<ParsedArgv, String> {
    let program = argv[0].clone();
    let mut opts = Options::new();

    opts.optflag("h", "help", "Print this help");
    opts.optflag("v", "version", "Show version");
    opts.optflag("u", "url", "Output URL to STDOUT instead of opening in browser");
    opts.optopt("r", "repo", "Shorthand format (user/repo, service/user/repo) or Git URL you want to see", "REPO");
    opts.optopt("d", "dir", "Directory path to your repository", "PATH");

    let matches = opts.parse(&argv[1..]).map_err(|f| format!("{}", f))?;

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [Options] {{Args}}", program);
        errorln!("{}", opts.usage(&brief));
        return Ok(ParsedArgv::Help);
    }

    if matches.opt_present("v") {
        println!("v0.0.0");
        return Ok(ParsedArgv::Version);
    }

    Ok(ParsedArgv::Parsed(CommandOptions {
        repo: matches.opt_str("r"),
        dir: matches.opt_str("d").unwrap_or("".to_string()),
        args: matches.free,
    }))
}

fn main() {
    let argv = env::args().collect::<Vec<_>>();
    let parsed = match parse_options(argv) {
        Ok(p) => p,
        Err(reason) => {
            errorln!("{}", reason);
            exit(3);
        },
    };

    let opts = match parsed {
        ParsedArgv::Help | ParsedArgv::Version => exit(0),
        ParsedArgv::Parsed(o) => o,
    };

    println!("Hello, world! {:?}", opts);
}
