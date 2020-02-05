use crate::config::{Config, EnvConfig};
use crate::error::Error;
use crate::test::helper::empty_env;
use crate::url;
use std::env;

#[cfg(not(target_os = "windows"))]
fn executable_path(cmd: &str) -> String {
    cmd.to_string()
}

#[cfg(target_os = "windows")]
fn executable_path(cmd: &str) -> String {
    Path::new(file!())
        .canonicalize()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("..\\..\\testdata\\{}.exe", cmd))
        .to_str()
        .unwrap()
        .to_string()
}

fn browse_env_config(cmd: String) -> EnvConfig {
    let mut env = empty_env();
    env.browse_command = Some(cmd);
    env
}

#[test]
fn smoke() {
    let c = Config {
        repo_url: "ssh://git@github.com:22/rhysd/git-brws.git".to_string(),
        branch: None,
        cwd: env::current_dir().unwrap(),
        args: vec![],
        stdout: false,
        pull_request: false,
        website: false,
        blame: false,
        remote: None,
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
    let exe = executable_path("true");
    let env = browse_env_config(exe);
    url::browse("https://example.com", &env).unwrap();
}

#[test]
fn fail_to_browse_url_with_user_command() {
    let exe = executable_path("false");
    let env = browse_env_config(exe.clone());
    match url::browse("https://example.com", &env) {
        Err(Error::UserBrowseCommandFailed { cmd, url, .. }) => {
            assert_eq!(cmd, exe);
            assert_eq!(url, "https://example.com");
        }
        r => assert!(false, "Unexpected result: {:?}", r),
    }
}

#[test]
fn browse_command_is_not_found() {
    let env = browse_env_config("this-command-is-not-existing-yeah".to_string());
    match url::browse("https://example.com", &env) {
        Err(Error::IoError { .. }) => { /* ok */ }
        r => assert!(false, "Unexpected result: {:?}", r),
    }
}
