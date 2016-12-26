extern crate getopts;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::io::Write;
use self::getopts::Options;

#[derive(Debug)]
pub struct CommandOptions {
    repo: Option<String>,
    dir: PathBuf,
    args: Vec<String>,
    url: bool,
}

pub enum ParsedArgv {
    Help,
    Version,
    Parsed(CommandOptions),
}

type ErrorMsg = String;

fn normalize_repo_format(mut s: String) -> Result<String, ErrorMsg> {
    if !s.ends_with(".git") {
        s.push_str(".git");
    }

    if s.starts_with("git@") || s.starts_with("https://") || s.starts_with("http://") {
        return Ok(s);
    }

    match s.chars().filter(|c| *c == '/').count() {
        1 => Ok(format!("https://github.com/{}", s)),
        2 => Ok(format!("https://{}", s)),
        _ => Err(format!("Error: Invalid repository format '{}'. Format must be one of 'user/repo', 'service/user/repo' or Git URL.", s)),
    }
}

pub fn parse_options(argv: Vec<String>) -> Result<ParsedArgv, ErrorMsg> {
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

    let repo = match matches.opt_str("r") {
        None => None,
        Some(r) => Some(normalize_repo_format(r)?),
    };

    let dir = match matches.opt_str("d") {
        Some(d) => fs::canonicalize(d).map_err(|e| format!("Error on --dir option: {}", e))?,
        None => env::current_dir().map_err(|e| format!("Error on --dir option: {}", e))?,
    };

    Ok(ParsedArgv::Parsed(CommandOptions {
        repo: repo,
        dir: dir,
        url: matches.opt_present("u"),
        args: matches.free,
    }))
}

