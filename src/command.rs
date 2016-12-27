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

pub fn browse(opts: Options) -> Option<ErrorMsg> {
    match url(opts) {
        Ok(url) => {
            println!("TODO: Open browser with URL {}", url);
            None
        },
        Err(msg) => Some(msg),
    }
}

