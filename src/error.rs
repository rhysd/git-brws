use std::fmt;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug)]
pub enum ExpectedNumberOfArgs {
    Single(usize),
    Range(usize, usize),
}

impl fmt::Display for ExpectedNumberOfArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpectedNumberOfArgs::Single(num) => write!(f, "{}", num),
            ExpectedNumberOfArgs::Range(min, max) => write!(f, "{}..{}", min, max),
        }
    }
}

#[derive(Debug)]
pub struct FailedParseAttempts(pub Vec<(&'static str, Error)>);

impl fmt::Display for FailedParseAttempts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Attempts")?;
        for (what, err) in self.0.iter() {
            writeln!(f, "  - {}: {}", what, err)?;
        }
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid repository format '{input}' or unknown remote. Note: Format must be one of 'repo', 'user/repo', 'host/user/repo', Git URL")]
    BrokenRepoFormat { input: String },
    #[error("Could not parse command line arguments")]
    CliParseFail {
        #[from]
        source: getopts::Fail,
    },
    #[error("{msg}: Cannot open URL {url}")]
    OpenUrlFailure { url: String, msg: String },
    #[error("GitLab does not support '..' for comparing diff between commits. Please use '...'")]
    GitLabDiffNotSupported,
    #[error("BitBucket does not support diff between commits (see https://bitbucket.org/site/master/issues/4779/ability-to-diff-between-any-two-commits)")]
    BitbucketDiffNotSupported,
    #[error("Azure Devops does not currently support this operation")]
    AzureDevOpsNotSupported,
    #[error("Can't detect user name from path {path}")]
    NoUserInPath { path: String },
    #[error("Can't detect repository name from path {path}")]
    NoRepoInPath { path: String },
    #[error("Unknown hosting service for URL {url}. If you want to use custom URL for GitHub Enterprise, please set $GIT_BRWS_GHE_URL_HOST")]
    UnknownHostingService { url: String },
    #[error("Broken URL '{url}': {msg}")]
    BrokenUrl { url: String, msg: String },
    #[error("--pr or -p does not support the service {service}")]
    PullReqNotSupported { service: String },
    #[error("GitHub API failure with response status {status}: {msg}")]
    GitHubStatusFailure {
        status: reqwest::StatusCode,
        msg: String,
    },
    #[error("Network request failed")]
    HttpClientError {
        #[from]
        source: reqwest::Error,
    },
    #[error(
        "I/O error happened. Note: Git command or current directory or file path may not exist"
    )]
    IoError {
        #[from]
        source: io::Error,
    },
    #[error(
        "Git command failed with stderr '{stderr}': `git {cmdline}` failed with non-zero status"
    )]
    GitCommandError { stderr: String, cmdline: String },
    #[error("Tracking name must be remote-url/branch-name: {0}")]
    UnexpectedRemoteName(String),
    #[error("Git could not find {kind} '{object}': {msg}")]
    GitObjectNotFound {
        kind: &'static str,
        object: String,
        msg: String,
    },
    #[error("Cannot locate root directory at {cwd:?}: {stderr}")]
    GitRootDirNotFound { cwd: PathBuf, stderr: String },
    #[error("Invalid number of arguments for {kind}. {expected} is expected but {actual} given")]
    WrongNumberOfArgs {
        expected: ExpectedNumberOfArgs,
        actual: usize,
        kind: String,
    },
    #[error("'..' or '...' must be contained for diff")]
    DiffDotsNotFound,
    #[error("Not a diff format since LHS and/or RHS is empty at input '{input}'")]
    DiffHandIsEmpty { input: String },
    #[error("Given path '{path:?}' is not in repository '{repo_root:?}'")]
    FileDirNotInRepo { repo_root: PathBuf, path: PathBuf },
    #[error("Cannot parse command line arguments '{cmdline}'\n{attempts}")]
    PageParseError {
        cmdline: String,
        attempts: FailedParseAttempts,
    },
    #[error("Issue number must start with '#' followed by numbers like #123")]
    InvalidIssueNumberFormat,
    #[error("Directory cannot have line number: {0:?}")]
    LineSpecifiedForDir(PathBuf),
    #[error("Cannot load environment variable")]
    EnvLoadError {
        #[from]
        source: envy::Error,
    },
    #[error(".git directory was not found. For {operation}, local repository must be known")]
    NoLocalRepoFound { operation: String },
    #[error("No repository was hit for query '{query}'")]
    NoSearchResult { query: String },
    #[error("{flag} option does not allow any command line argument. It opens page based on {{repo}}, but argument(s) {args:?} retrives information from local directory")]
    ArgsNotAllowed {
        flag: &'static str,
        args: Vec<String>,
    },
    #[error("GitHub Enterprise requires API token. Please set $GIT_BRWS_GHE_TOKEN")]
    GheTokenRequired,
    #[error("File path is not given to blame")]
    BlameWithoutFilePath,
    #[error("Cannot blame directory '{dir}'. Please specify file path")]
    CannotBlameDirectory { dir: String },
    #[error("Command '{cmd}' failed to open URL {url}. Please check $GIT_BRWS_BROWSE_COMMAND. stderr: {msg}")]
    UserBrowseCommandFailed {
        cmd: String,
        url: String,
        msg: String,
    },
    #[error("Specified directory '{dir}' with -d option does not exist")]
    SpecifiedDirNotExist { dir: String },
}

pub type Result<T> = ::std::result::Result<T, Error>;
