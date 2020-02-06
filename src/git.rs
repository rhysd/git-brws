use crate::error::{Error, Result};
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;

pub struct Git<'a> {
    command: &'a str,
    cwd: &'a Path,
}

impl<'a> Git<'a> {
    pub fn command<S: AsRef<OsStr> + Debug>(&self, args: &[S]) -> Result<String> {
        let out = Command::new(&self.command)
            .arg("-C")
            .arg(self.cwd)
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

    pub fn hash(&self, commit: impl AsRef<str>) -> Result<String> {
        self.command(&["rev-parse", commit.as_ref()])
            .map_err(|err| Error::GitObjectNotFound {
                kind: "commit",
                object: commit.as_ref().to_string(),
                msg: format!("{}", err),
            })
    }

    pub fn tag_hash(&self, tagname: impl AsRef<str>) -> Result<String> {
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

    pub fn remote_url(&self, name: impl AsRef<str>) -> Result<String> {
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

    pub fn tracking_remote_url(
        &self,
        branch: &Option<impl AsRef<str>>,
    ) -> Result<(String, String)> {
        let rev = match branch {
            Some(b) => format!("{}@{{u}}", b.as_ref()),
            None => "@{u}".to_string(),
        };

        let out = match self.command(&["rev-parse", "--abbrev-ref", "--symbolic", rev.as_str()]) {
            Ok(stdout) => stdout,
            Err(Error::GitCommandError { ref stderr, .. })
                if stderr.contains("does not point to a branch") =>
            {
                return Ok((self.remote_url("origin")?, "origin".to_string()));
            }
            Err(err) => return Err(err),
        };

        // out is formatted as '{remote-name}/{branch-name}'
        match out.splitn(2, '/').next() {
            Some(u) => Ok((self.remote_url(u)?, u.to_string())),
            None => Err(Error::UnexpectedRemoteName(out.clone())),
        }
    }

    pub fn root_dir(&self) -> Result<PathBuf> {
        match self.command(&["rev-parse", "--show-toplevel"]) {
            Ok(stdout) => Ok(fs::canonicalize(stdout)?),
            Err(Error::GitCommandError { stderr, .. }) => Err(Error::GitRootDirNotFound {
                cwd: self.cwd.to_owned(),
                stderr,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn current_branch(&self) -> Result<String> {
        self.command(&["rev-parse", "--abbrev-ref", "--symbolic", "HEAD"])
    }

    pub fn remote_contains(
        &self,
        spec: impl AsRef<str>,
        remote_branch: impl AsRef<str>,
    ) -> Result<bool> {
        self.command(&[
            "branch",
            "--remote",
            remote_branch.as_ref(),
            "--contains",
            spec.as_ref(),
        ])
        .map(|o| !o.is_empty())
    }

    // Returns {remote}/{branch}
    pub fn remote_branch(
        &self,
        remote_name: &Option<impl AsRef<str>>,
        local_branch: &Option<impl AsRef<str>>,
    ) -> Result<String> {
        let remote = match remote_name {
            Some(r) => r.as_ref(),
            None => "{u}",
        };
        let branch = match local_branch {
            Some(b) => b.as_ref(),
            None => "",
        };
        let rev = format!("{}@{}", branch, remote);
        self.command(&["rev-parse", "--abbrev-ref", "--symbolic", rev.as_str()])
    }
}

impl<'a> Git<'a> {
    pub fn new(cwd: &'a Path, command: &'a str) -> Git<'a> {
        Git { command, cwd }
    }
}
