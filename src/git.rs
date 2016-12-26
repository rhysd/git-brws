use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Git<'a> {
    command: String,
    dir: &'a str,
}

type ErrorMsg = String;

impl<'a> Git<'a> {
    fn command<S: AsRef<OsStr>>(&self, args: &[S]) -> Result<String, ErrorMsg> {
        let out = Command::new(&self.command)
                    .arg(format!("--git-dir={}", self.dir))
                    .args(args)
                    .output()
                    .map_err(|e| format!("Error on executing git command: {}", e))?;
        if !out.status.success() {
            return Err("Git command exited with non-zero status".to_string())?
        }
        Ok(format!("{}", String::from_utf8_lossy(&out.stdout)))
    }

    pub fn hash<S: AsRef<str>>(&self, commit: &S) -> Result<String, ErrorMsg> {
        self.command(&["rev-parse", commit.as_ref()])
    }

    pub fn remote_url<S: AsRef<str>>(&self, name: &S) -> Result<String, ErrorMsg> {
        self.command(&["remote", "get-url", name.as_ref()])
    }

    pub fn tracking_remote(&self) -> Result<(String, String), ErrorMsg> {
        let out = self.command(&["rev-parse", "--abbrev-ref", "--symbolic", "@{u}"])?;
        let mut split = out.split("/");
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
}

pub fn new(dir: &PathBuf) -> Result<Git, ErrorMsg> {
    let command = env::var("GIT_BRWS_GIT_COMMAND").unwrap_or("git".to_string());
    let path = dir.to_str().ok_or(format!("Failed to retrieve directory path as UTF8 string: {:?}", dir))?;
    Ok(Git { command: command, dir: path })
}
