use crate::git::{git_dir, Git};
use std::env;

#[test]
fn get_git_dir() {
    let current = env::current_dir().unwrap();
    for (ref d, ref c) in &[
        (None, ""),
        (Some(&current), ""),
        (None, "git"),
        (Some(&current), "git"),
    ] {
        let dir = git_dir(d.clone(), c).unwrap();
        assert!(dir.ends_with(".git"), "{:?} with {:?}, {:?}", dir, d, c);
    }
}

#[test]
fn git_get_current_branch() {
    let g = Git::new(".git".as_ref(), "git");
    let b = g.current_branch();
    assert!(b.is_ok(), "{:?}", b);
}

#[test]
fn git_get_commit_hash() {
    let g = Git::new(".git".as_ref(), "git");
    let b = g.current_branch().unwrap();

    for spec in &["HEAD", &b] {
        match g.hash(spec) {
            Ok(h) => {
                for c in h.chars() {
                    assert!(c.is_ascii_hexdigit(), "{:?}", h);
                }
            }
            Err(err) => assert!(false, "{:?}", err),
        };
    }
}

#[test]
fn git_root_dir() {
    let g = Git::new(".git".as_ref(), "git");
    let p = g.root_dir().unwrap();
    assert!(p.join(".git").exists(), "{:?}", p);
}

#[test]
fn git_check_remote_contains() {
    let g = Git::new(".git".as_ref(), "git");
    g.remote_contains("HEAD").unwrap();
    // Note: Do not check returned value since it is depending on whether current commit is already
    // pushed or not
}
