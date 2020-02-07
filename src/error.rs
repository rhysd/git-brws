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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpectedNumberOfArgs::Single(num) => write!(f, "{}", num),
            ExpectedNumberOfArgs::Range(min, max) => write!(f, "{}..{}", min, max),
        }
    }
}

// TODO: Add backtrace when std::backtrace is stabilized
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Box<Error> {
        // TODO: Capture backtrace when std::backtrace is stabilized
        Box::new(Error { kind })
    }

    pub fn err<T>(kind: ErrorKind) -> Result<T> {
        Err(Error::new(kind))
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub enum ErrorKind {
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
    AzureDevOpsNotSupported,
    NoUserInPath {
        path: String,
    },
    NoRepoInPath {
        path: String,
    },
    UnknownHostingService {
        url: String,
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
    GitObjectNotFound {
        kind: &'static str,
        object: String,
        msg: String,
    },
    GitRootDirNotFound {
        cwd: PathBuf,
        stderr: String,
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
        attempts: Vec<(&'static str, Error)>,
    },
    InvalidIssueNumberFormat,
    LineSpecifiedForDir(PathBuf),
    EnvLoadError(envy::Error),
    NoLocalRepoFound {
        operation: String,
    },
    NoSearchResult {
        query: String,
    },
    ArgsNotAllowed {
        flag: &'static str,
        args: Vec<String>,
    },
    GheTokenRequired,
    BlameWithoutFilePath,
    CannotBlameDirectory {
        dir: String,
    },
    UserBrowseCommandFailed {
        cmd: String,
        url: String,
        msg: String,
    },
    SpecifiedDirNotExist {
        dir: String,
    },
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::BrokenRepoFormat {input} => write!(f, "Invalid repository format '{}' or unknown remote. Note: Format must be one of 'repo', 'user/repo', 'host/user/repo', Git URL", input),
            ErrorKind::CliParseFail(e) => write!(f, "{}", e),
            ErrorKind::OpenUrlFailure {url, msg} => write!(f, "{}: Cannot open URL {}", msg, url),
            ErrorKind::GitLabDiffNotSupported => write!(f, "GitLab does not support '..' for comparing diff between commits. Please use '...'"),
            ErrorKind::BitbucketDiffNotSupported => write!(f, "BitBucket does not support diff between commits (see https://bitbucket.org/site/master/issues/4779/ability-to-diff-between-any-two-commits)"),
            ErrorKind::AzureDevOpsNotSupported => write!(f, "Azure Devops does not currently support this operation"),
            ErrorKind::NoUserInPath{path} => write!(f, "Can't detect user name from path {}", path),
            ErrorKind::NoRepoInPath{path} => write!(f, "Can't detect repository name from path {}", path),
            ErrorKind::UnknownHostingService {url} => write!(f, "Unknown hosting service for URL {}. If you want to use custom URL for GitHub Enterprise, please set $GIT_BRWS_GHE_URL_HOST", url),
            ErrorKind::BrokenUrl {url, msg} => write!(f, "Broken URL '{}': {}", url, msg),
            ErrorKind::PullReqNotSupported {service} => write!(f, "--pr or -p does not support the service {}", service),
            ErrorKind::GitHubStatusFailure {status, msg} => write!(f, "GitHub API response status {}: {}", status, msg),
            ErrorKind::HttpClientError(err) => write!(f, "{}", err),
            ErrorKind::IoError(err) => write!(f, "I/O error: {}. Note: Git command or current directory or file path may not exist", err),
            ErrorKind::GitCommandError{stderr, args} => {
                if stderr.is_empty() {
                    write!(f, "`git")?;
                } else {
                    write!(f, "{}: `git", stderr)?;
                }
                for arg in args.iter() {
                    write!(f, " '{}'", arg.to_string_lossy())?;
                }
                write!(f, "` exited with non-zero status")
            }
            ErrorKind::GitObjectNotFound{kind, object, msg} if msg.is_empty() => write!(f, "Git could not find {} '{}'", kind, object),
            ErrorKind::GitObjectNotFound{kind, object, msg} => write!(f, "Git could not find {} '{}': {}", kind, object, msg),
            ErrorKind::GitRootDirNotFound{cwd, stderr} => write!(f, "Cannot locate root directory at {:?}: {}", cwd, stderr),
            ErrorKind::UnexpectedRemoteName(name) => write!(f, "Tracking name must be remote-url/branch-name: {}", name),
            ErrorKind::WrongNumberOfArgs{expected, actual, kind} => write!(f, "Invalid number of arguments for {}. {} is expected but given {}", kind, expected, actual),
            ErrorKind::DiffDotsNotFound => write!(f, "'..' or '...' must be contained for diff"),
            ErrorKind::DiffHandIsEmpty{input} => write!(f, "Not a diff format since LHS and/or RHS is empty {}", input),
            ErrorKind::FileDirNotInRepo{repo_root, path} => write!(f, "Given path '{:?}' is not in repository '{:?}'", path, repo_root),
            ErrorKind::PageParseError{args, attempts} => {
                write!(f, "Cannot parse command line arguments {:?}\nAttempts:", args)?;
                for (what, err) in attempts.iter() {
                    write!(f, "\n  - {}: {}", what, err.kind())?;
                }
                Ok(())
            }
            ErrorKind::InvalidIssueNumberFormat => write!(f, "Issue number must start with '#' followed by numbers like #123"),
            ErrorKind::LineSpecifiedForDir(path) => write!(f, "Directory cannot have line number: {:?}", path),
            ErrorKind::EnvLoadError(err) => write!(f, "Cannot load environment variable: {}", err),
            ErrorKind::NoLocalRepoFound{operation} => write!(f, ".git directory was not found. For {}, local repository must be known", operation),
            ErrorKind::NoSearchResult{query} => write!(f, "No repository was hit for query '{}'", query),
            ErrorKind::ArgsNotAllowed{flag, args} => write!(f, "{} option does not allow any command line argument. It opens page based on {{repo}}, but argument(s) {:?} retrives information from local directory.", flag, args),
            ErrorKind::GheTokenRequired => write!(f, "GitHub Enterprise requires API token. Please set $GIT_BRWS_GHE_TOKEN"),
            ErrorKind::BlameWithoutFilePath => write!(f, "File path is not given to blame"),
            ErrorKind::CannotBlameDirectory{dir} => write!(f, "Cannot blame directory '{}'. Please specify file path", dir),
            ErrorKind::UserBrowseCommandFailed{cmd, url, msg} => write!(f, "Command '{}' failed to open URL {}. Please check $GIT_BRWS_BROWSE_COMMAND. stderr: {}", cmd, url, msg),
            ErrorKind::SpecifiedDirNotExist{dir} => write!(f, "Specified directory '{}' with -d option does not exist", dir),
        }
    }
}

macro_rules! error_from {
    ($cause:ty, $kind:ident) => {
        impl From<$cause> for Box<Error> {
            fn from(err: $cause) -> Box<Error> {
                Error::new(ErrorKind::$kind(err))
            }
        }
    };
}

error_from!(io::Error, IoError);
error_from!(reqwest::Error, HttpClientError);
error_from!(getopts::Fail, CliParseFail);
error_from!(envy::Error, EnvLoadError);

pub type Result<T> = ::std::result::Result<T, Box<Error>>;
