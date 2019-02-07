extern crate getopts;

use crate::command;
use crate::env::Env;
use crate::error::{Error, Result};
use crate::git;
use crate::git::Git;
use getopts::Options;

fn convert_ssh_url(mut url: String) -> String {
    if url.starts_with("git@") {
        // Note: Convert SSH protocol URL
        //  git@service.com:user/repo.git -> ssh://git@service.com:22/user/repo.git
        if let Some(i) = url.find(':') {
            url.insert_str(i + 1, "22/");
        }
        url.insert_str(0, "ssh://");
    }
    url
}

#[cfg_attr(feature = "cargo-clippy", allow(clippy::large_enum_variant))]
#[derive(Debug)]
pub enum ParsedArgv {
    Help(String),
    Version(&'static str),
    Parsed(command::Config),
}

fn normalize_repo_format(mut s: String, git: &Git) -> Result<String> {
    if let Ok(url) = git.remote_url(&s) {
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
        _ => Err(Error::BrokenRepoFormat { input: s }),
    }
}

const USAGE: &str = "\
Usage: git brws [Options] {Args}

  Open a repository, file, commit, diff or pull request in your web browser from
  command line. GitHub, Bitbucket, GitLab, GitHub Enterprise are supported as
  hosting service.
  git-brws looks some environment variables for configuration. Please see
  https://github.com/rhysd/git-brws for more detail.

Examples:
  - Open current repository:

    $ git brws

  - Open specific GitHub repository:

    $ git brws -r rhysd/git-brws

  - Open a file:

    $ git brws some/file.txt

  - Open specific commit:

    $ git brws HEAD~3

  - Open diff between commits:

    $ git brws HEAD~3..HEAD

  - Open diff between topic and topic's merge base commit:

    $ git brws master...topic

  - Open line 123 of file:

    $ git brws some/file.txt#L123

  - Open range from line 123 to line 126 of file:

    $ git brws some/file.txt#L123-L126

  - Open a pull request page (for GitHub and GitHub Enterprise):

    $ git brws --pr

  - Open an issue page:

    $ git brws '#8'";

pub fn parse_options<T: AsRef<str>>(argv: &[T]) -> Result<ParsedArgv> {
    let mut opts = Options::new();

    opts.optopt("r", "repo", "Shorthand format (user/repo, host/user/repo) or remote name (e.g. origin) or Git URL you want to see", "REPO");
    opts.optopt("b", "branch", "Branch name to browse", "BRANCH");
    opts.optopt("d", "dir", "Directory path to the repository", "PATH");
    opts.optflag(
        "u",
        "url",
        "Output URL to stdout instead of opening in browser",
    );
    opts.optflag(
        "p",
        "pr",
        "Open pull request page instead of repository page",
    );
    opts.optflag("h", "help", "Print this help");
    opts.optflag("v", "version", "Show version");

    let matches = opts.parse(argv[1..].iter().map(|a| a.as_ref()))?;

    if matches.opt_present("h") {
        return Ok(ParsedArgv::Help(opts.usage(USAGE)));
    }

    if matches.opt_present("v") {
        return Ok(ParsedArgv::Version(
            option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"),
        ));
    }

    let env = Env::new();
    let git_dir = git::git_dir(matches.opt_str("d"), env.git_command.as_str())?;
    let branch = matches.opt_str("b");
    let repo = {
        // Create scope for borrowing git_dir ref
        let git = Git::new(&git_dir, env.git_command.as_str());
        match matches.opt_str("r") {
            Some(r) => normalize_repo_format(r, &git)?,
            None => git.tracking_remote(&branch)?,
        }
    };

    let repo = convert_ssh_url(repo);

    let stdout = matches.opt_present("u");
    let pull_request = matches.opt_present("p");

    Ok(ParsedArgv::Parsed(command::Config {
        repo,
        branch,
        git_dir,
        args: matches.free,
        stdout,
        pull_request,
        env,
    }))
}
