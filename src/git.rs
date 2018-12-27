use crate::errors::Result;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;

pub struct Git<'a> {
    command: &'a str,
    git_dir: &'a Path,
}

impl<'a> Git<'a> {
    pub fn command<S: AsRef<OsStr> + Debug>(&self, args: &[S]) -> Result<String> {
        let out = Command::new(&self.command)
            .arg("--git-dir")
            .arg(self.git_dir)
            .args(args)
            .output()
            .map_err(|e| format!("Error on executing git command: {}", e))?;
        if !out.status.success() {
            let stderr = str::from_utf8(&out.stderr)
                .expect("Failed to convert git command output from UTF8");
            return Err(format!(
                "Git command exited with non-zero status (git-dir: '{:?}', args: '{:?}'): {}",
                self.git_dir, args, stderr
            ));
        }
        let s = str::from_utf8(&out.stdout)
            .map_err(|e| format!("Invalid UTF-8 sequence in output of git command: {}", e))?;
        Ok(s.trim().to_string())
    }

    pub fn hash<S: AsRef<str>>(&self, commit: S) -> Result<String> {
        self.command(&["rev-parse", commit.as_ref()])
    }

    pub fn remote_url<S: AsRef<str>>(&self, name: S) -> Result<String> {
        // XXX:
        // `git remote get-url {name}` is not available because it's added recently (at 2.6.1).
        // Note that git installed in Ubuntu 14.04 is 1.9.1.
        self.command(&["config", "--get", &format!("remote.{}.url", name.as_ref())])
    }

    pub fn tracking_remote<S: AsRef<str>>(&self, branch: &Option<S>) -> Result<String> {
        let rev = match branch {
            Some(b) => format!("{}@{{u}}", b.as_ref()),
            None => "@{u}".to_string(),
        };

        let out = match self.command(&["rev-parse", "--abbrev-ref", "--symbolic", rev.as_str()]) {
            Ok(stdout) => stdout,
            Err(stderr) => {
                return if stderr.find("does not point to a branch").is_some() {
                    Ok(self.remote_url("origin")?)
                } else {
                    Err(format!(
                        "Failed to retrieve default remote name: {}",
                        stderr
                    ))
                }
            }
        };

        // out is formatted as '{remote-url}/{branch-name}'
        match out.splitn(2, '/').next() {
            Some(ref u) => self.remote_url(u),
            None => Err(format!("Invalid tracking remote name: {}", out)),
        }
    }

    pub fn root_dir(&self) -> Result<PathBuf> {
        // XXX:
        // `git rev-parse` can't be used with --git-dir arguments.
        // `git --git-dir ../.git rev-parse --show-toplevel` always returns
        // current working directory.
        // So here root directory is calculated from git-dir.
        let p = self.git_dir.parent().ok_or_else(|| {
            format!(
                "Cannot locate root directory from git-dir '{:?}'",
                self.git_dir
            )
        })?;
        Ok(p.to_owned())
    }

    pub fn current_branch(&self) -> Result<String> {
        match self.command(&["rev-parse", "--abbrev-ref", "--symbolic", "HEAD"]) {
            Ok(stdout) => Ok(stdout),
            Err(stderr) => Err(format!("Cannot get current branch: {}", stderr)),
        }
    }
}

impl<'a> Git<'a> {
    pub fn new<P: AsRef<Path>>(dir: &'a P, command: &'a str) -> Git<'a> {
        Git {
            command,
            git_dir: dir.as_ref(),
        }
    }
}

pub fn git_dir(dir: Option<String>, git_cmd: &str) -> Result<PathBuf> {
    let mut cmd = Command::new(if git_cmd != "" { git_cmd } else { "git" });
    cmd.arg("rev-parse").arg("--absolute-git-dir");
    if let Some(d) = dir {
        let d = fs::canonicalize(&d)
            .map_err(|e| format!("Cannot resolve repository directory {}: {}", &d, e))?;
        cmd.current_dir(d);
    }

    let out = cmd.output().map_err(|e| format!("{}", e))?;
    if !out.status.success() {
        let stderr = str::from_utf8(&out.stderr)
            .map_err(|e| format!("Failed to convert git command output as UTF-8: {}", e))?;
        return Err(format!(
            "Git command exited with non-zero status: {}",
            stderr
        ));
    }

    let stdout = str::from_utf8(&out.stdout)
        .map_err(|e| format!("Invalid UTF-8 sequence in output of git command: {}", e))?
        .trim();

    Path::new(stdout)
        .canonicalize()
        .map_err(|e| format!("Cannot resolve $GIT_DIR: {}", e))
}
