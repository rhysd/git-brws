extern crate open;

use std::path::PathBuf;
use page::parse_page;
use service::parse_service;

#[derive(Debug)]
pub struct Config {
    pub repo: String,
    pub branch: Option<String>,
    pub git_dir: PathBuf,
    pub args: Vec<String>,
}

type ErrorMsg = String;

pub fn url(cfg: Config) -> Result<String, ErrorMsg> {
    let page = parse_page(&cfg)?;
    let service = parse_service(&cfg.repo)?;
    service.page_url(&page, &cfg.branch)
}

fn open(url: String) -> Option<ErrorMsg> {
    match open::that(&url) {
        Ok(status) => {
            if status.success() {
                None
            } else {
                if let Some(code) = status.code() {
                    Some(format!("Error on opening URL {}: Command exited with non-zero status {}", url, code))
                } else {
                    Some(format!("Error on opening URL {}: Command terminated by signal", url))
                }
            }
        },
        Err(e) => Some(format!("Error on opening URL {}: {}", url, e))
    }
}

pub fn browse(cfg: Config) -> Option<ErrorMsg> {
    match url(cfg) {
        Ok(url) => open(url),
        Err(msg) => Some(msg),
    }
}

