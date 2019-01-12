extern crate getopts;
extern crate reqwest;
extern crate url;

use std::ffi::OsString;
use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ExpectedNumberOfArgs {
    Single(usize),
    Range(usize, usize),
}

impl fmt::Display for ExpectedNumberOfArgs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpectedNumberOfArgs::Single(num) => write!(f, "{}", num),
            ExpectedNumberOfArgs::Range(min, max) => write!(f, "{}..{}", min, max),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    BrokenRepoFormat {
        input: String,
    },
    CliParseFail(getopts::Fail),
    OpenUrlFailure {
        url: String,
        msg: String,
    },
    GitLabDiffNotSupported,
    BitbucketDiffNotSupported,
    NoUserInPath {
        path: String,
    },
    NoRepoInPath {
        path: String,
    },
    UnknownHostingService {
        url: String,
    },
    GitHubPullReqNotFound {
        author: String,
        repo: String,
        branch: String,
    },
    BrokenUrl {
        url: String,
        msg: String,
    },
    PullReqNotSupported {
        service: String,
    },
    GitHubStatusFailure {
        status: reqwest::StatusCode,
        msg: String,
    },
    HttpClientError(reqwest::Error),
    IoError(io::Error),
    GitCommandError {
        stderr: String,
        args: Vec<OsString>,
    },
    UnexpectedRemoteName(String),
    GitRootDirNotFound {
        git_dir: PathBuf,
    },
    WrongNumberOfArgs {
        expected: ExpectedNumberOfArgs,
        actual: usize,
        kind: String,
    },
    DiffDotsNotFound,
    DiffHandIsEmpty {
        input: String,
    },
    FileDirNotInRepo {
        repo_root: PathBuf,
        path: PathBuf,
    },
    PageParseError {
        args: Vec<String>,
        attempts: Vec<Error>,
    },
    InvalidIssueNumberFormat,
    LineSpecifiedForDir(PathBuf),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BrokenRepoFormat {input} => write!(f, "Invalid repository format '{}'. Format must be one of 'user/repo', 'host/user/repo', remote name or Git URL", input),
            Error::CliParseFail(e) => write!(f, "{}", e),
            Error::OpenUrlFailure {url, msg} => write!(f, "Cannot open URL {}: {}", url, msg),
            Error::GitLabDiffNotSupported => write!(f, "GitLab does not support '..' for comparing diff between commits. Please use '...'"),
            Error::BitbucketDiffNotSupported => write!(f, "BitBucket does not support diff between commits (see https://bitbucket.org/site/master/issues/4779/ability-to-diff-between-any-two-commits)"),
            Error::NoUserInPath{path} => write!(f, "Can't detect user name from path {}", path),
            Error::NoRepoInPath{path} => write!(f, "Can't detect repository name from path {}", path),
            Error::UnknownHostingService {url} => write!(f, "Unknown hosting service for URL {}. If you want to use custom URL for GitHub Enterprise, please set $GIT_BRWS_GHE_URL_HOST", url),
            Error::GitHubPullReqNotFound{author, repo, branch} => write!(f, "No pull request authored by @{} at {}@{}", author, repo, branch),
            Error::BrokenUrl {url, msg} => write!(f, "Broken URL '{}': {}", url, msg),
            Error::PullReqNotSupported {service} => write!(f, "--pr or -p does not support the service {}", service),
            Error::GitHubStatusFailure {status, msg} => write!(f, "GitHub API response status {}: {}", status, msg),
            Error::HttpClientError(err) => write!(f, "{}", err),
            Error::IoError(err) => write!(f, "{}", err),
            Error::GitCommandError{stderr, args} => write!(f, "Git command {:?} exited with non-zero status: {}", args, stderr),
            Error::GitRootDirNotFound{git_dir} => write!(f, "Cannot locate root directory from GIT_DIR {:?}", git_dir),
            Error::UnexpectedRemoteName(name) => write!(f, "Tracking name must be remote-url/branch-name: {}", name),
            Error::WrongNumberOfArgs{expected, actual, kind} => write!(f, "Invalid number of arguments for {}. {} is expected but given {}", kind, expected, actual),
            Error::DiffDotsNotFound => write!(f, "'..' or '...' must be contained for diff"),
            Error::DiffHandIsEmpty{input} => write!(f, "Not a diff format since LHS and/or RHS is empty {}", input),
            Error::FileDirNotInRepo{repo_root, path} => write!(f, "Given path '{:?}' is not in repository '{:?}'", path, repo_root),
            Error::PageParseError{args, attempts} => {
                write!(f, "Error on parsing command line arguments {:?}", args)?;
                for err in attempts.iter() {
                    write!(f, "\n  - {}", err)?;
                }
                Ok(())
            }
            Error::InvalidIssueNumberFormat => write!(f, "Issue number must start with '#' followed by numbers like #123"),
            Error::LineSpecifiedForDir(path) => write!(f, "Directory cannot have line number: {:?}", path),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(inner: reqwest::Error) -> Error {
        Error::HttpClientError(inner)
    }
}

impl From<self::getopts::Fail> for Error {
    fn from(f: self::getopts::Fail) -> Error {
        Error::CliParseFail(f)
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;