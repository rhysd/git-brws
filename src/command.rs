extern crate open;

use errors::*;
use page::parse_page;
use service;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub repo: String,
    pub branch: Option<String>,
    pub git_dir: PathBuf,
    pub args: Vec<String>,
}

pub fn url(cfg: &Config) -> Result<String> {
    let page = parse_page(&cfg)?;
    service::parse_and_build_page_url(&cfg.repo, &page, &cfg.branch)
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
        Ok(url) => open(url.as_str()),
        Err(msg) => Some(msg),
    }
}
