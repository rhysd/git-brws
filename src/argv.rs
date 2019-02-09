extern crate getopts;

use crate::command;
use crate::env::EnvConfig;
use crate::error::{Error, Result};
use crate::git;
use crate::git::Git;
use getopts::Options;
use std::env;

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

fn normalize_repo_format(mut slug: String, git: &Option<Git>) -> Result<String> {
    if let Some(git) = git {
        if let Ok(url) = git.remote_url(&slug) {
            return Ok(url);
        }
    }

    if !slug.ends_with(".git") {
        slug.push_str(".git");
    }

    if slug.starts_with("git@") || slug.starts_with("https://") || slug.starts_with("http://") {
        return Ok(slug);
    }

    match slug.chars().filter(|c| *c == '/').count() {
        1 => Ok(format!("https://github.com/{}", slug)),
        2 => Ok(format!("https://{}", slug)),
        _ => Err(Error::BrokenRepoFormat { input: slug }),
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

    let env = EnvConfig::from_iter(env::vars())?;
    let git_dir = git::git_dir(matches.opt_str("d"), env.git_command.as_str());
    let branch = matches.opt_str("b");
    let (repo, git_dir) = {
        // Create scope for borrowing git_dir ref
        match matches.opt_str("r") {
            Some(r) => {
                let git_dir = git_dir.ok();
                let git = git_dir.as_ref().map(|d| Git::new(d, &env.git_command));
                (normalize_repo_format(r, &git)?, git_dir)
            }
            None => {
                // When --repo is not set, remote URL needs to know its URL in this case
                let git_dir = git_dir?;
                let git = Git::new(&git_dir, &env.git_command);
                (git.tracking_remote(&branch)?, Some(git_dir))
            }
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
