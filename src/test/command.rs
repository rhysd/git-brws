use crate::command::*;
use crate::error::Error;
use crate::test::helper::empty_env;
use std::env;
use std::path::Path;

#[test]
fn smoke() {
    let mut d = env::current_dir().unwrap();
    d.push(Path::new(".git"));
    let c = Config {
        repo: "ssh://git@github.com:22/rhysd/git-brws.git".to_string(),
        branch: None,
        git_dir: Some(d),
        args: vec![],
        stdout: false,
        pull_request: false,
        env: empty_env(),
    };
    match url(&c) {
        Ok(u) => assert_eq!(
            u, "https://github.com/rhysd/git-brws",
            "Unexpected URL: {}",
            u
        ),
        Err(e) => assert!(false, "command::url() was not processed properly: {}", e),
    }
}

#[test]
fn git_dir_mandatory_for_pull_request() {
    let c = Config {
        repo: "ssh://git@github.com:22/rhysd/git-brws.git".to_string(),
        branch: None,
        git_dir: None,
        args: vec![],
        stdout: false,
        pull_request: true,
        env: empty_env(),
    };
    match url(&c) {
        Ok(u) => assert!(false, "Unexpected success: {}", u),
        Err(Error::NoLocalRepoFound { operation }) => assert!(
            operation.contains("opening a pull request"),
            "Unexpected operation: {}",
            operation
        ),
        Err(e) => assert!(false, "Unexpected error: {}", e),
    }
}
