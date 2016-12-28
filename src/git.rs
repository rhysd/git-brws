use std::env;
use std::fs;
use std::str;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use util;

pub struct Git<'a> {
    command: String,
    git_dir: &'a str,
}

impl<'a> Git<'a> {
    pub fn command<S: AsRef<OsStr>>(&self, args: &[S]) -> util::Result<String> {
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

    pub fn hash<S: AsRef<str>>(&self, commit: &S) -> util::Result<String> {
        self.command(&["rev-parse", commit.as_ref()])
    }

    pub fn remote_url<S: AsRef<str>>(&self, name: &S) -> util::Result<String> {
        // XXX:
        // `git remote get-url {name}` is not available because it's added recently (at 2.6.1).
        // Note that git installed in Ubuntu 14.04 is 1.9.1.
        let mut url = self.command(&["config", "--get", &format!("remote.{}.url", name.as_ref())])?;
        if url.starts_with("git@") {
            // Note: Convert SSH protocol URL
            //  git@service.com:user/repo.git -> ssh://git@service.com:22/user/repo.git
            if let Some(i) = url.find(':') {
                util::insert(&mut url, i + 1, "22/");
            }
            util::insert(&mut url, 0, "ssh://");
        }
        Ok(url)
    }

    pub fn tracking_remote(&self) -> util::Result<(String, String)> {
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

    pub fn root_dir(&self) -> util::Result<PathBuf> {
        // XXX:
        // `git rev-parse` can't be used with --git-dir arguments.
        // `git --git-dir ../.git rev-parse --show-toplevel` always returns
        // current working directory.
        // So here root directory is calculated from git-dir.
        let p = Path::new(self.git_dir)
                    .parent()
                    .ok_or(format!("Cannot locate root directory from git-dir '{}'", self.git_dir))?;
        Ok(p.to_owned())
    }
}

pub fn get_git_command() -> String {
    env::var("GIT_BRWS_GIT_COMMAND").unwrap_or("git".to_string())
}

pub fn new(dir: &PathBuf) -> util::Result<Git> {
    let command = get_git_command();
    let path = dir.to_str().ok_or(format!("Failed to retrieve directory path as UTF8 string: {:?}", dir))?;
    Ok(Git { command: command, git_dir: path })
}

fn set_current_dir(p: &PathBuf) -> util::Result<()> {
    env::set_current_dir(p).map_err(|e| format!("Error on setting current direcotry to {:?}: {}", p, e))
}

fn git_revparse_git_dir(current: &PathBuf) -> util::Result<PathBuf> {
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

pub fn git_dir(dir: Option<String>) -> util::Result<PathBuf> {
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
