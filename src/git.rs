use crate::error::{Error, Result};
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
            .output()?;
        if out.status.success() {
            Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
        } else {
            Err(Error::GitCommandError {
                stderr: String::from_utf8_lossy(&out.stderr)
                    .trim()
                    .replace('\n', " "),
                args: args.iter().map(|a| a.as_ref().to_owned()).collect(),
            })
        }
    }

    pub fn hash<S: AsRef<str>>(&self, commit: S) -> Result<String> {
        self.command(&["rev-parse", commit.as_ref()])
            .map_err(|err| Error::GitObjectNotFound {
                kind: "commit",
                object: commit.as_ref().to_string(),
                msg: format!("{}", err),
            })
    }

    pub fn tag_hash<S: AsRef<str>>(&self, tagname: S) -> Result<String> {
        let tagname = tagname.as_ref();
        let stdout = self
            .command(&["show-ref", "--tags", tagname])
            .map_err(|err| Error::GitObjectNotFound {
                kind: "tag name",
                object: tagname.to_string(),
                msg: format!("{}", err),
            })?;
        // Output must be in format '{rev} {ref name}'
        Ok(stdout.splitn(2, ' ').next().unwrap().to_string())
    }

    pub fn remote_url<S: AsRef<str>>(&self, name: S) -> Result<String> {
        // XXX:
        // `git remote get-url {name}` is not available because it's added recently (at 2.6.1).
        // Note that git installed in Ubuntu 14.04 is 1.9.1.
        let name = name.as_ref();
        self.command(&["config", "--get", &format!("remote.{}.url", name)])
            .map_err(|err| Error::GitObjectNotFound {
                kind: "remote",
                object: name.to_string(),
                msg: format!("{}", err),
            })
    }

    pub fn tracking_remote<S: AsRef<str>>(&self, branch: &Option<S>) -> Result<String> {
        let rev = match branch {
            Some(b) => format!("{}@{{u}}", b.as_ref()),
            None => "@{u}".to_string(),
        };

        let out = match self.command(&["rev-parse", "--abbrev-ref", "--symbolic", rev.as_str()]) {
            Ok(stdout) => stdout,
            Err(Error::GitCommandError { ref stderr, .. })
                if stderr.contains("does not point to a branch") =>
            {
                return Ok(self.remote_url("origin")?);
            }
            Err(err) => return Err(err),
        };

        // out is formatted as '{remote-name}/{branch-name}'
        match out.splitn(2, '/').next() {
            Some(ref u) => self.remote_url(u),
            None => Err(Error::UnexpectedRemoteName(out.clone())),
        }
    }

    pub fn root_dir(&self) -> Result<PathBuf> {
        // XXX:
        // `git rev-parse` can't be used with --git-dir arguments.
        // `git --git-dir ../.git rev-parse --show-toplevel` always returns
        // current working directory.
        // So here root directory is calculated from git-dir.
        let p = self
            .git_dir
            .parent()
            .ok_or_else(|| Error::GitRootDirNotFound {
                git_dir: self.git_dir.to_owned(),
            })?;
        Ok(p.to_owned())
    }

    pub fn current_branch(&self) -> Result<String> {
        self.command(&["rev-parse", "--abbrev-ref", "--symbolic", "HEAD"])
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
        cmd.current_dir(fs::canonicalize(&d)?);
    }

    let out = cmd.output()?;
    if !out.status.success() {
        return Err(Error::GitCommandError {
            stderr: String::from_utf8_lossy(&out.stderr)
                .trim()
                .replace('\n', " "),
            args: vec![
                OsStr::new(git_cmd).to_os_string(),
                OsStr::new("rev-parse").to_os_string(),
                OsStr::new("--absolute-git-dir").to_os_string(),
            ],
        });
    }

    let stdout = str::from_utf8(&out.stdout)
        .expect("Invalid UTF-8 sequence in stdout of git command")
        .trim();

    Ok(Path::new(stdout).canonicalize()?)
}
