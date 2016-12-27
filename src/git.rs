use std::env;
use std::fs;
use std::str;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Git<'a> {
    command: String,
    git_dir: &'a str,
}

type ErrorMsg = String;

impl<'a> Git<'a> {
    pub fn command<S: AsRef<OsStr>>(&self, args: &[S]) -> Result<String, ErrorMsg> {
        let out = Command::new(&self.command)
                    .arg("--git-dir")
                    .arg(self.git_dir)
                    .args(args)
                    .output()
                    .map_err(|e| format!("Error on executing git command: {}", e))?;
        if !out.status.success() {
            return Err(format!("Git command exited with non-zero status: {}", str::from_utf8(&out.stderr).expect("Failed to convert git command output from UTF8")));
        }
        let s = str::from_utf8(&out.stdout).map_err(|e| format!("Invalid UTF-8 sequence in output of git command: {}", e))?;
        Ok(s.trim().to_string())
    }

    pub fn hash<S: AsRef<str>>(&self, commit: &S) -> Result<String, ErrorMsg> {
        self.command(&["rev-parse", commit.as_ref()])
    }

    pub fn remote_url<S: AsRef<str>>(&self, name: &S) -> Result<String, ErrorMsg> {
        self.command(&["remote", "get-url", name.as_ref()])
    }

    pub fn tracking_remote(&self) -> Result<(String, String), ErrorMsg> {
        let out = self.command(&["rev-parse", "--abbrev-ref", "--symbolic", "@{u}"])?;
        let mut split = out.splitn(2, "/");
        let url = match split.next() {
            None => return Err(format!("Invalid tracking remote name: {}", out)),
            Some(ref u) => self.remote_url(&u)?,
        };
        let branch = match split.next() {
            None => return Err(format!("Invalid tracking remote name: {}", out)),
            Some(ref u) => u.to_string(),
        };
        Ok((url, branch))
    }

    pub fn root_dir(&self) -> Result<PathBuf, ErrorMsg> {
        let s = self.command(&["rev-parse", "--show-toplevel"])?;
        Ok(PathBuf::from(s))
    }
}

pub fn get_git_command() -> String {
    env::var("GIT_BRWS_GIT_COMMAND").unwrap_or("git".to_string())
}

pub fn new(dir: &PathBuf) -> Result<Git, ErrorMsg> {
    let command = get_git_command();
    let path = dir.to_str().ok_or(format!("Failed to retrieve directory path as UTF8 string: {:?}", dir))?;
    Ok(Git { command: command, git_dir: path })
}

fn set_current_dir(p: &PathBuf) -> Result<(), ErrorMsg> {
    env::set_current_dir(p).map_err(|e| format!("Error on setting current direcotry to {:?}: {}", p, e))
}

fn git_revparse_git_dir(current: &PathBuf) -> Result<PathBuf, ErrorMsg> {
    let out = Command::new(get_git_command())
                .arg("rev-parse")
                .arg("--git-dir")
                .output()
                .map_err(|e| format!("{}", e))?;
    if !out.status.success() {
        return Err(format!("Git command exited with non-zero status: {}", str::from_utf8(&out.stderr).expect("Failed to convert git command output from UTF8")));
    }

    let s = str::from_utf8(&out.stdout).map_err(|e| format!("Invalid UTF-8 sequence in output of git command: {}", e))?.trim();

    let p = Path::new(s);
    if p.is_relative() {
        Ok(current.join(&p))
    } else {
        Ok(p.to_owned())
    }
}

pub fn git_dir(dir: Option<String>) -> Result<PathBuf, ErrorMsg> {
    let current_dir = env::current_dir().map_err(|e| format!("{}", e))?;
    match dir {
        Some(d) => {
            let d = fs::canonicalize(&d).map_err(|e| format!("Cannot locate canonical path for '{}': {}", d, e))?;
            set_current_dir(&d)?;
            let p = git_revparse_git_dir(&d).map_err(|e| format!("{}", e));
            set_current_dir(&current_dir)?;
            p
        },
        None => git_revparse_git_dir(&current_dir),
    }
}
