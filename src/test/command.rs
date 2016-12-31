use std::env;
use std::path::Path;
use command::*;

// Somke test only
#[test]
fn smoke() {
    let mut d = env::current_dir().unwrap();
    d.push(Path::new(".git"));
    let c = Config {
        repo: "git@.github.com:rhysd/git-brws.git".to_string(),
        branch: None,
        git_dir: d,
        args: vec![],
    };
    assert_eq!(url(c).unwrap(), "https://github.com/rhysd/git-brws");
}
