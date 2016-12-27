use std::path::PathBuf;

#[derive(Debug)]
pub struct Options {
    pub repo: String,
    pub branch: String,
    pub dir: PathBuf,
    pub args: Vec<String>,
}

type ErrorMsg = String;

pub fn url(opts: Options) -> Result<String, ErrorMsg> {
    Ok("https://to.be.implemented.com".to_string())
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

