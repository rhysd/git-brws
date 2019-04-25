use crate::config::Config;
use crate::test::helper::empty_env;
use crate::url;
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
