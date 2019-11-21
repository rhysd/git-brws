use crate::config::{Config, EnvConfig};
use crate::error::Error;
use crate::test::helper::empty_env;
use crate::url;
use std::env;
use std::path::Path;

fn browse_env_config<S: ToString>(cmd: S) -> EnvConfig {
    let mut env = empty_env();
    env.browse_command = Some(cmd.to_string());
    env
}

#[test]
fn smoke() {
    let mut d = env::current_dir().unwrap();
    d.push(Path::new(".git"));
    let c = Config {
        repo_url: "ssh://git@github.com:22/rhysd/git-brws.git".to_string(),
        branch: None,
        git_dir: Some(d),
        args: vec![],
        stdout: false,
        pull_request: false,
        website: false,
        blame: false,
        env: empty_env(),
    };
    match url::build_url(&c) {
        Ok(u) => assert_eq!(
            u, "https://github.com/rhysd/git-brws",
            "Unexpected URL: {}",
            u
        ),
        Err(e) => assert!(false, "url::build_url() was not processed properly: {}", e),
    }
}

#[test]
fn browse_url_with_user_command() {
    let env = browse_env_config("true");
    url::browse("https://example.com", &env).unwrap();
}

#[test]
fn fail_to_browse_url_with_user_command() {
    let env = browse_env_config("false");
    match url::browse("https://example.com", &env) {
        Err(Error::UserBrowseCommandFailed { cmd, url, .. }) => {
            assert_eq!(cmd, "false");
            assert_eq!(url, "https://example.com");
        }
        r => assert!(false, "Unexpected result: {:?}", r),
    }
}

#[test]
fn browse_command_is_not_found() {
    let env = browse_env_config("this-command-is-not-existing-yeah");
    match url::browse("https://example.com", &env) {
        Err(Error::IoError { .. }) => { /* ok */ }
        r => assert!(false, "Unexpected result: {:?}", r),
    }
}
