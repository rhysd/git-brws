extern crate open;

use std::path::PathBuf;
use page::parse_page;
use service;

#[derive(Debug)]
pub struct Options {
    pub repo: String,
    pub branch: String,
    pub git_dir: PathBuf,
    pub args: Vec<String>,
}

type ErrorMsg = String;

pub fn url(opts: Options) -> Result<String, ErrorMsg> {
    let page = parse_page(&opts)?;
    service::parse_url(&opts.repo, &opts.branch, &page)
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

pub fn browse(opts: Options) -> Option<ErrorMsg> {
    match url(opts) {
        Ok(url) => open(url),
        Err(msg) => Some(msg),
    }
}

