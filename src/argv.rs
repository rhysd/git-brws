extern crate getopts;

use self::getopts::Options;
use command;
use errors::Result;
use git;
use std::path::PathBuf;

fn convert_ssh_url(mut url: String) -> String {
    if url.starts_with("git@") {
        // Note: Convert SSH protocol URL
        //  git@service.com:user/repo.git -> ssh://git@service.com:22/user/repo.git
        if let Some(i) = url.find(':') {
            &mut url.insert_str(i + 1, "22/");
        }
        &mut url.insert_str(0, "ssh://");
    }
    url
}

#[derive(Debug)]
pub enum ParsedArgv {
    Help(String),
    Version(&'static str),
    Parsed(command::Config, bool),
}

fn normalize_repo_format(mut s: String, git_dir: &PathBuf) -> Result<String> {
    if let Ok(url) = git::new(git_dir)?.remote_url(&s) {
        return Ok(url);
    }

    if !s.ends_with(".git") {
        s.push_str(".git");
    }

    if s.starts_with("git@") || s.starts_with("https://") || s.starts_with("http://") {
        return Ok(s);
    }

    match s.chars().filter(|c| *c == '/').count() {
        1 => Ok(format!("https://github.com/{}", s)),
        2 => Ok(format!("https://{}", s)),
        _ => Err(format!("Error: Invalid repository format '{}'. Format must be one of 'user/repo', 'service/user/repo' or remote name or Git URL.", s)),
    }
}

fn usage(program: &String) -> String {
    format!(
        "\
Usage: {} [Options] {{Args}}

  Open a repository, file, commit or diff in your web browser from command line.
  GitHub, Bitbucket, GitLab, GitHub Enterprise are supported as hosting service.
  Please see https://github.com/rhysd/git-brws for more detail.

Examples:
  - Open current repository:

    $ git brws

  - Open a file:

    $ git brws some/file.txt

  - Open specific commit:

    $ git brws HEAD~3

  - Open diff between commits:

    $ git brws HEAD~3..HEAD

  - Open line 123 of file:

    $ git brws some/file.txt#L123",
        program
    )
}

pub fn parse_options(argv: Vec<String>) -> Result<ParsedArgv> {
    let program = argv[0].clone();
    let mut opts = Options::new();

    opts.optopt("r", "repo", "Shorthand format (user/repo, service/user/repo) or remote name (e.g. origin) or Git URL you want to see", "REPO");
    opts.optopt("b", "branch", "Branch name of the repository", "BRANCH");
    opts.optopt("d", "dir", "Directory path to your repository", "PATH");
    opts.optflag(
        "u",
        "url",
        "Output URL to STDOUT instead of opening in browser",
    );
    opts.optflag("h", "help", "Print this help");
    opts.optflag("v", "version", "Show version");

    let matches = opts.parse(&argv[1..]).map_err(|f| format!("{}", f))?;

    if matches.opt_present("h") {
        let brief = usage(&program);
        return Ok(ParsedArgv::Help(opts.usage(&brief)));
    }

    if matches.opt_present("v") {
        return Ok(ParsedArgv::Version(
            option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"),
        ));
    }

    let git_dir = git::git_dir(matches.opt_str("d"))?;

    let repo = match matches.opt_str("r") {
        Some(r) => normalize_repo_format(r, &git_dir)?,
        None => git::new(&git_dir)?.tracking_remote()?,
    };
    let repo = convert_ssh_url(repo);

    let show_url = matches.opt_present("u");

    Ok(ParsedArgv::Parsed(
        command::Config {
            repo,
            branch: matches.opt_str("b"),
            git_dir,
            args: matches.free,
        },
        show_url,
    ))
}
