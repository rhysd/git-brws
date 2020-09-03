use crate::config::{Config, EnvConfig};
use crate::error::{Error, ErrorKind, Result};
use crate::page::parse_page;
use crate::service;
use std::process::{Command, ExitStatus, Stdio};

pub fn build_url(cfg: &Config) -> Result<String> {
    let page = parse_page(&cfg)?;
    service::build_page_url(&page, &cfg)
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

#[cfg(unix)]
fn error_without_status(status: ExitStatus) -> String {
    use std::os::unix::process::ExitStatusExt;
    if let Some(sig) = status.signal() {
        format!("Command terminated by signal {}", sig)
    } else {
        "Command terminated by signal".to_string()
    }
}

#[cfg(not(unix))]
fn error_without_status(_status: ExitStatus) -> String {
    "Command terminated by signal".to_string()
}

pub fn browse(url: &str, env: &EnvConfig) -> Result<()> {
    if let Some(cmd) = &env.browse_command {
        return browse_with_cmd(url, cmd);
    }

    match open::that(url) {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => {
            let url = url.to_string();
            let msg = if let Some(code) = status.code() {
                format!("Command exited with non-zero status {}", code)
            } else {
                error_without_status(status)
            };
            Error::err(ErrorKind::OpenUrlFailure { url, msg })
        }
        Err(e) => Error::err(ErrorKind::OpenUrlFailure {
            url: url.to_string(),
            msg: format!("{}", e),
        }),
    }
}
