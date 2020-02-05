use crate::config::EnvConfig;
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn empty_env() -> EnvConfig {
    EnvConfig {
        git_command: "git".to_string(),
        ghe_ssh_port: None,
        ghe_url_host: None,
        gitlab_ssh_port: None,
        github_token: None,
        ghe_token: None,
        https_proxy: None,
        browse_command: None,
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

pub fn get_root_dir() -> PathBuf {
    let mut root = fs::canonicalize(env::current_dir().unwrap())
        .unwrap()
        .clone();
    loop {
        let prev = root.clone();
        root.pop();
        if prev == root {
            break;
        }
    }
    root
}
