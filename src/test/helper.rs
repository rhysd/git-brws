use crate::config::EnvConfig;
use std::env;

pub fn empty_env() -> EnvConfig {
    EnvConfig {
        git_command: "git".to_string(),
        ghe_ssh_port: None,
        ghe_url_host: None,
        gitlab_ssh_port: None,
        github_token: None,
        ghe_token: None,
        https_proxy: None,
    }
}

pub fn https_proxy() -> Option<String> {
    env::var("https_proxy")
        .or_else(|_| env::var("HTTPS_PROXY"))
        .ok()
}

macro_rules! skip_if_no_token {
    () => {
        match ::std::env::var("GIT_BRWS_GITHUB_TOKEN").or_else(|_| ::std::env::var("GITHUB_TOKEN"))
        {
            Ok(ref v) if v == "" => return,
            Ok(v) => Some(v),
            Err(_) => return,
        }
    };
}
