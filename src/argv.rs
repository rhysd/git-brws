use crate::async_runtime;
use crate::config::{Config, EnvConfig};
use crate::error::{Error, ErrorKind, Result};
use crate::git::Git;
use crate::github_api::Client;
use getopts::Options;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

fn handle_scp_like_syntax(mut url: String) -> String {
    // ref: https://git-scm.com/book/en/v2/Git-on-the-Server-The-Protocols

    if url.contains("://") {
        // When url is a URL like https://server/project.git
        return url;
    }

    // When the target is an scp-like syntax: [user@]server:project.git
    // Handle ':' in the syntax. Note that GitHub user name may start with number (#24)
    if let Some(i) = url.find(':') {
        let after_colon = i + 1;
        if let Some(i) = url[after_colon..].find(':') {
            // When port number exists
            //  git@service.com:123:user/repo.git -> git@service.com:123/user/repo.git
            let i = after_colon + i;
            url.replace_range(i..i + 1, "/");
        } else {
            // When a port number is omitted, default SSH port is 22
            //  git@service.com:user/repo.git -> git@service.com:22/user/repo.git
            url.insert_str(after_colon, "22/"); // Insert port number after colon
        }
    }

    // Examples:
    //  git@service.com:22/user/repo.git -> ssh://git@service.com:22/user/repo.git
    url.insert_str(0, "ssh://");
    url
}

#[cfg_attr(feature = "cargo-clippy", allow(clippy::large_enum_variant))]
#[derive(Debug)]
pub enum Parsed {
    Help(String),
    Version(&'static str),
    OpenPage(Config),
}

fn is_scp_like_syntax_with_user(s: &str) -> bool {
    // SCP-like syntax like user@project:project
    // user@ cannot be omitted in SCP-like syntax because `s` can be a search query
    //
    // Note that checking it starts with "git@" does not work because SSH protocol for
    // visualstudio.com is "mycompany@vs-ssh.visualstudio.com:v3/project.git"
    if s.contains(char::is_whitespace) {
        return false; // Containing whitespace means it's a search query
    }
    for (i, c) in s.char_indices() {
        match c {
            '@' if i == 0 => return false,          // Seems query like "@rhysd"
            '@' => return s[i + 1..].contains(':'), // SCP-like syntax must also contain ':'
            ':' => return false,                    // '@' must be put before ':'
            _ => continue,
        }
    }
    false
}

fn normalize_repo_format(mut slug: String, env: &EnvConfig) -> Result<String> {
    if slug.is_empty() {
        return Error::err(ErrorKind::BrokenRepoFormat { input: slug });
    }

    // - URL like https://server/project
    // - SCP-like syntax user@project:project
    //
    // Note: user@ cannot be omitted in SCP-like syntax because `slug` can be search word
    if slug.starts_with("https://")
        || slug.starts_with("ssh://")
        || slug.starts_with("http://")
        || slug.starts_with("file://")
        || is_scp_like_syntax_with_user(&slug)
    {
        if !slug.ends_with(".git") {
            slug.push_str(".git");
        }
        return Ok(slug);
    }

    match slug.chars().filter(|c| *c == '/').count() {
        1 => Ok(format!("https://github.com/{}.git", slug)),
        2 => Ok(format!("https://{}.git", slug)),
        0 => {
            let client = Client::build("api.github.com", &env.github_token, &env.https_proxy)?;
            async_runtime::blocking(client.most_popular_repo_by_name(&slug))
                .map(|repo| repo.clone_url)
        }
        _ => Error::err(ErrorKind::BrokenRepoFormat { input: slug }),
    }
}

fn get_cwd(specified: Option<String>) -> Result<PathBuf> {
    if let Some(dir) = specified {
        let p = fs::canonicalize(&dir)?;
        if !p.exists() {
            return Error::err(ErrorKind::SpecifiedDirNotExist { dir });
        }
        Ok(p)
    } else {
        Ok(env::current_dir()?.canonicalize()?)
    }
}

const USAGE: &str = "\
Usage: git brws [Options] {Args}

  Open a repository, file, commit, diff or pull request, issue or project's
  website in your web browser from command line.
  GitHub, Bitbucket, GitLab, GitHub Enterprise, Azure DevOps are supported as
  hosting service.
  git-brws looks some environment variables for configuration. Please see
  https://github.com/rhysd/git-brws#readme for more details.

Examples:
  - Current repository:

    $ git brws

  - GitHub repository:

    $ git brws -r rhysd/git-brws

  - Most popular GitHub repository by name:

    $ git brws -r git-brws

  - File:

    $ git brws some/file.txt

  - Commit:

    $ git brws HEAD~3

  - Tag:

    $ git brws 0.10.0

  - Diff between commits:

    $ git brws HEAD~3..HEAD

  - Diff between topic and topic's merge base commit:

    $ git brws master...topic

  - Line 123 of file:

    $ git brws some/file.txt#L123

  - Range from line 123 to line 126 of file:

    $ git brws some/file.txt#L123-L126

  - Pull request page (for GitHub and GitHub Enterprise):

    $ git brws --pr

  - Website of repository at current directory

    $ git brws --website

  - Website of other repository

    $ git brws --website --repo react

  - Issue page:

    $ git brws '#8'";

impl Parsed {
    pub fn parse_iter<I>(argv: I) -> Result<Parsed>
    where
        I: IntoIterator,
        I::Item: AsRef<OsStr>,
    {
        let mut opts = Options::new();

        opts.optopt("r", "repo", "Shorthand format (repo, user/repo, host/user/repo) or Git URL you want to see. When only repo name is specified, most popular repository will be searched from GitHub", "REPO");
        opts.optopt("b", "branch", "Branch name to browse", "BRANCH");
        opts.optopt(
            "d",
            "dir",
            "Directory path to the repository. Default value is current working directory",
            "PATH",
        );
        opts.optopt("R", "remote", "Remote name (e.g. origin). Default value is a remote the current branch is tracking. If current branch tracks no branch, it falls back to 'origin'", "REMOTE");
        opts.optflag(
            "u",
            "url",
            "Output URL to stdout instead of opening in browser",
        );
        opts.optflag(
            "p",
            "pr",
            "Open pull request page instead of repository page. If not existing, open 'Create Pull Request' page",
        );
        opts.optflag(
            "w",
            "website",
            "Open website page instead of repository page (homepage URL for GitHub, GitLab pages, Bitbucket Cloud)",
        );
        opts.optflag(
            "B",
            "blame",
            "Open blame page instead of repository page. File path to blame must be passed also.",
        );
        opts.optflag(
            "c",
            "current-branch",
            "Open the current branch instead of default branch",
        );
        opts.optflag("h", "help", "Print this help");
        opts.optflag("v", "version", "Show version");

        let matches = opts.parse(argv.into_iter().skip(1))?;

        if matches.opt_present("h") {
            return Ok(Parsed::Help(opts.usage(USAGE)));
        }

        if matches.opt_present("v") {
            return Ok(Parsed::Version(
                option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"),
            ));
        }

        let env = EnvConfig::from_iter(env::vars())?.with_global_env();
        let cwd = get_cwd(matches.opt_str("d"))?;
        let git = Git::new(&cwd, &env.git_command);
        let branch = if let Some(b) = matches.opt_str("b") {
            Some(b)
        } else if matches.opt_present("c") {
            Some(git.current_branch()?)
        } else {
            None
        };
        let (repo_url, remote) = match (matches.opt_str("r"), matches.opt_str("R")) {
            (Some(repo), remote) => {
                if !matches.free.is_empty() {
                    return Error::err(ErrorKind::ArgsNotAllowed {
                        flag: "--repo {repo}",
                        args: matches.free,
                    });
                }
                (normalize_repo_format(repo, &env)?, remote)
            }
            (None, remote) => {
                let (url, remote) = if let Some(remote) = remote {
                    (git.remote_url(&remote)?, remote)
                } else {
                    git.tracking_remote_url(&branch)?
                };
                (url, Some(remote))
            }
        };

        let repo_url = handle_scp_like_syntax(repo_url);

        Ok(Parsed::OpenPage(Config {
            repo_url,
            branch,
            cwd,
            stdout: matches.opt_present("u"),
            pull_request: matches.opt_present("p"),
            website: matches.opt_present("w"),
            blame: matches.opt_present("B"),
            args: matches.free,
            remote,
            env,
        }))
    }
}
