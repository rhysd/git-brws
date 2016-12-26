use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Git<'a> {
    command: String,
    dir: &'a Path,
}

type ErrorMsg = String;

impl<'a> Git<'a> {
    pub fn hash(&self, refname: &String) -> Result<String, ErrorMsg> {
        let out = Command::new(&self.command)
                            .arg("rev-parse")
                            .arg(refname)
                            .output()
                            .map_err(|e| format!("Error on getting hash of {}: {}", refname, e))?;

        if !out.status.success() {
            return Err(format!("Error on getting hash of {}: {}", refname, String::from_utf8_lossy(&out.stderr)))
        }

        Ok(format!("{}", String::from_utf8_lossy(&out.stdout)))
    }
}

pub fn new(dir: &PathBuf) -> Git {
    let command = env::var("GIT_BRWS_GIT_COMMAND").unwrap_or("git".to_string());
    Git { command: command, dir: dir.as_path() }
}
