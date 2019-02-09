extern crate open;

use crate::env::EnvConfig;
use crate::error::{Error, Result};
use crate::git::Git;
use crate::page::parse_page;
use crate::pull_request;
use crate::service;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub repo: String,
    pub branch: Option<String>,
    pub git_dir: Option<PathBuf>,
    pub args: Vec<String>,
    pub stdout: bool,
    pub pull_request: bool,
    pub env: EnvConfig,
}

pub fn url(cfg: &Config) -> Result<String> {
    if cfg.pull_request {
        match cfg.branch {
            Some(ref b) => pull_request::find_url(cfg.repo.as_str(), b.as_str(), &cfg.env),
            None => {
                if let Some(ref git_dir) = &cfg.git_dir {
                    let git = Git::new(git_dir, cfg.env.git_command.as_str());
                    pull_request::find_url(
                        cfg.repo.as_str(),
                        git.current_branch()?.as_str(),
                        &cfg.env,
                    )
                } else {
                    Err(Error::NoLocalRepoFound {
                        operation: "opening a pull request without specifying branch".to_string(),
                    })
                }
            }
        }
    } else {
        let page = parse_page(&cfg)?;
        service::build_page_url(&cfg.repo, &page, &cfg.branch, &cfg.env)
    }
}

fn open(url: &str) -> Result<()> {
    match open::that(url) {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => {
            let url = url.to_string();
            let msg = if let Some(code) = status.code() {
                format!("Command exited with non-zero status {}", code)
            } else {
                "Error on opening URL {}: Command terminated by signal".to_string()
            };
            Err(Error::OpenUrlFailure { url, msg })
        }
        Err(e) => Err(Error::OpenUrlFailure {
            url: url.to_string(),
            msg: format!("{}", e),
        }),
    }
}

pub fn browse(cfg: &Config) -> Result<()> {
    let u = url(cfg)?;
    if cfg.stdout {
        println!("{}", u);
        Ok(())
    } else {
        open(&u)
    }
}
