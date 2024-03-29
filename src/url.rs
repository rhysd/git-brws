use crate::config::{Config, EnvConfig};
use crate::error::{Error, ErrorKind, Result};
use crate::page::parse_page;
use crate::service;
use std::process::{Command, Stdio};

pub fn build_url(cfg: &Config) -> Result<String> {
    let page = parse_page(cfg)?;
    service::build_page_url(&page, cfg)
}

fn browse_with_cmd(url: &str, cmd: &str) -> Result<()> {
    let out = Command::new(cmd)
        .arg(url)
        .stdout(Stdio::inherit())
        .output()?;
    if out.status.success() {
        Ok(())
    } else {
        Error::err(ErrorKind::UserBrowseCommandFailed {
            cmd: cmd.to_string(),
            url: url.to_string(),
            msg: String::from_utf8_lossy(&out.stderr)
                .trim()
                .replace('\n', " "),
        })
    }
}

pub fn browse(url: &str, env: &EnvConfig) -> Result<()> {
    if let Some(cmd) = &env.browse_command {
        return browse_with_cmd(url, cmd);
    }

    open::that(url).map_err(|e| {
        Error::new(ErrorKind::OpenUrlFailure {
            url: url.to_string(),
            msg: format!("{}", e),
        })
    })
}
