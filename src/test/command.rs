use command::*;
use std::env;
use std::path::Path;

// Somke test only
#[test]
fn smoke() {
    let mut d = env::current_dir().unwrap();
    d.push(Path::new(".git"));
    let c = Config {
        repo: "ssh://git@github.com:22/rhysd/git-brws.git".to_string(),
        branch: None,
        git_dir: d,
        args: vec![],
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
