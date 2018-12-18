extern crate open;

use crate::envvar;
use crate::errors::*;
use crate::git;
use crate::page::parse_page;
use crate::pull_request;
use crate::service;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub repo: String,
    pub branch: Option<String>,
    pub git_dir: PathBuf,
    pub args: Vec<String>,
    pub stdout: bool,
    pub pull_request: bool,
    pub env: envvar::Envvar,
}

pub fn url(cfg: &Config) -> Result<String> {
    if cfg.pull_request {
        match cfg.branch {
            Some(ref b) => pull_request::find_url(cfg.repo.as_str(), b.as_str(), &cfg.env),
            None => {
                let git = git::new(&cfg.git_dir, cfg.env.git_command.as_str())?;
                pull_request::find_url(cfg.repo.as_str(), git.current_branch()?.as_str(), &cfg.env)
            }
        }
    } else {
        let page = parse_page(&cfg)?;
        service::parse_and_build_page_url(&cfg.repo, &page, &cfg.branch, &cfg.env)
    }
}

fn open(url: &str) -> Option<ErrorMsg> {
    match open::that(url) {
        Ok(status) => {
            if status.success() {
                None
            } else if let Some(code) = status.code() {
                Some(format!(
                    "Error on opening URL {}: Command exited with non-zero status {}",
                    url, code
                ))
            } else {
                Some(format!(
                    "Error on opening URL {}: Command terminated by signal",
                    url
                ))
            }
        }
        Err(e) => Some(format!("Error on opening URL {}: {}", url, e)),
    }
}

pub fn browse(cfg: &Config) -> Option<ErrorMsg> {
    match url(cfg) {
        Ok(ref url) if cfg.stdout => {
            println!("{}", url);
            None
        }
        Ok(url) => open(url.as_str()),
        Err(msg) => Some(msg),
    }
}
