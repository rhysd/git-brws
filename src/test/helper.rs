use crate::config::EnvConfig;
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn empty_env() -> EnvConfig {
    EnvConfig {
        git_command: "git".to_string(),
        ghe_ssh_port: None,
        ghe_url_host: None,
        gitlab_url_host: None,
        gitlab_ssh_port: None,
        github_token: None,
        ghe_token: None,
        https_proxy: None,
        browse_command: None,
        short_commit_hash: false,
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

// XXX: On CI, run tests for calling GitHub API only on Linux. This is because git-brws uses
// 'GET /search/*' APIs but they have special rate limit 30/min. Running jobs parallelly on
// CI hits the rate limit and tests fails. Even if running the jobs sequentially, it
// sometimes hits the limit.
macro_rules! skip_if_no_token_for_search {
    () => {{
        if let Ok(ref v) = ::std::env::var("GIT_BRWS_CI_SKIP_TEST_FOR_SEARCH_API") {
            if v == "true" {
                return;
            }
        }
        skip_if_no_token!()
    }};
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
