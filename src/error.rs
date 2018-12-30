/*
use std::ffi::OsString;
use std::io;

pub type Result<T> = ::std::result::Result<T, failure::Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(
        display = "Invalid repository format '{}'. Format must be one of 'user/repo', 'service/user/repo', remote name or Git URL",
        input
    )]
    InvalidRepoFormat { input: String },
    #[fail(display = "Git command with argument {:?} failed: {}", args, err)]
    GitExecutionFail { args: Vec<OsString>, err: io::Error },
    #[fail(
        display = "Git command with argument {:?} exited with non-zero status: {}",
        args, stderr
    )]
    GitExitFailure { args: Vec<OsString>, stderr: String },
    #[fail(display = "Default remote name not found: {}", message)]
    GitDefaultRemoteNotFound { message: String },
}
*/

extern crate getopts;
extern crate reqwest;
extern crate url;

use std::ffi::OsString;
use std::fmt;
use std::io;
use std::path::PathBuf;

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
    DiffWrongNumberOfArgs(usize),
    DiffDotsNotFound,
    DiffHandIsEmpty {
        input: String,
    },
    FileDirWrongNumberOfArgs(usize),
    FileDirNotInRepo {
        repo_root: PathBuf,
        path: PathBuf,
    },
    PageParseError {
        args: Vec<String>,
        attempts: Vec<Error>,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BrokenRepoFormat {input} => write!(f, "Invalid repository format '{}'. Format must be one of 'user/repo', 'service/user/repo', remote name or Git URL", input),
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
            Error::DiffWrongNumberOfArgs(arity) => write!(f, "Invalid number of arguments for commit. 1 is expected but given {}", arity),
            Error::DiffDotsNotFound => write!(f, "'..' or '...' must be contained for diff"),
            Error::DiffHandIsEmpty{input} => write!(f, "Not a diff format since LHS and/or RHS is empty {}", input),
            Error::FileDirWrongNumberOfArgs(arity) => write!(f, "Invalid number of arguments for file or directory. 1..2 is expected but given {}", arity),
            Error::FileDirNotInRepo{repo_root, path} => write!(f, "Given path '{:?}' is not in repository '{:?}'", path, repo_root),
            Error::PageParseError{args, attempts} => {
                write!(f, "Error on parsing command line arguments {:?}", args)?;
                for err in attempts.iter() {
                    write!(f, "\n  {}", err)?;
                }
                Ok(())
            }
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